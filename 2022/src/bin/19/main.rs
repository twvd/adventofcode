#![feature(int_roundings)]
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchState<'a> {
    blueprint: &'a Blueprint,

    ticks: i32,
    resources: ResourceCount,
    bots: ResourceCount,
}

impl<'a> SearchState<'a> {
    fn new_initial(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            ticks: 0,
            resources: ResourceCount::new(),
            bots: ResourceCount::from([(Resource::Ore, 1)]),
        }
    }

    fn can_afford(&self, cost: &ResourceCount) -> bool {
        cost.iter()
            .all(|(res, &amt)| *self.resources.get(res).unwrap_or(&0) >= amt)
    }

    fn can_ever_afford(&self, cost: &ResourceCount) -> bool {
        cost.iter()
            .all(|(res, _)| *self.bots.get(res).unwrap_or(&0) > 0)
    }

    fn can_afford_bot(&self, res: Resource) -> bool {
        self.can_afford(&self.blueprint.cost[&res])
    }

    fn afford_when(&self, cost: &ResourceCount) -> Option<i32> {
        if !self.can_ever_afford(cost) {
            return None;
        }
        cost.iter()
            .map(|(res, &amt)| {
                if *self.resources.get(res).unwrap_or(&0) >= amt {
                    0
                } else {
                    (amt - *self.resources.get(res).unwrap_or(&0))
                        .div_ceil(*self.bots.get(res).unwrap_or(&0))
                }
            })
            .max()
    }

    fn afford_bot_when(&self, bot: Resource) -> Option<i32> {
        self.afford_when(&self.blueprint.cost[&bot])
    }

    fn spend(&mut self, cost: &ResourceCount) {
        assert!(self.can_afford(cost));
        for (res, amt) in cost {
            *self.resources.entry(*res).or_insert(0) -= amt;
        }
    }

    fn produce_resources(&mut self, ticks: i32) {
        for res in Resource::iter() {
            *self.resources.entry(res).or_insert(0) += *self.bots.entry(res).or_insert(0) * ticks;
        }
    }

    fn tick(&mut self, buy_bot: Option<Resource>) {
        // Resource production this tick
        self.produce_resources(1);

        // Purchase new bot
        if let Some(bot) = buy_bot {
            assert!(self.can_afford_bot(bot));
            self.spend(&self.blueprint.cost[&bot]);
            *self.bots.entry(bot).or_insert(0) += 1;
        }

        self.ticks += 1;
    }

    fn tick_advance(&mut self, ticks: i32) {
        self.produce_resources(ticks);
        self.ticks += ticks;
    }

    fn sensible_ever_affordable_bots(&self) -> Vec<(Resource, i32)> {
        // - When we reach the amount of bots equals to the maximum
        //   amount of resource to build something, then we
        //   no longer have to build bots of that type.
        // - Skip ahead in ticks for build decisions (afford_when)
        //   to reduce unnecessary branching.
        Resource::iter()
            .filter_map(|b| {
                if let Some(w) = self.afford_bot_when(b) {
                    Some((b, w))
                } else {
                    None
                }
            })
            .filter(|(b, _)| {
                *b == Resource::Geode
                    || *self.bots.get(b).unwrap_or(&0) < self.blueprint.max_needed_per_sec(*b)
            })
            .collect()
    }
}

impl<'a> std::hash::Hash for SearchState<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.ticks);
        for res in Resource::iter() {
            state.write_i32(*self.resources.get(&res).unwrap_or(&0));
            state.write_i32(*self.bots.get(&res).unwrap_or(&0));
        }
        state.finish();
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Blueprint {
    id: i32,

    cost: HashMap<Resource, ResourceCount>,
}
type ResourceCount = HashMap<Resource, i32>;
type Blueprints = Vec<Blueprint>;

impl Blueprint {
    fn max_needed_per_sec(&self, res: Resource) -> i32 {
        // The highest amount of a resource we would need to build a bot.
        *self
            .cost
            .iter()
            .filter(|(&r, _)| r != res)
            .map(|(_, rc)| rc.get(&res).unwrap_or(&0))
            .max()
            .unwrap_or(&0)
    }

    fn search_optimal(&self, total_ticks: i32) -> i32 {
        let initial_state = SearchState::new_initial(self);
        let mut work: VecDeque<SearchState> = VecDeque::with_capacity(1000);
        let mut seen: HashSet<SearchState> = HashSet::with_capacity(50000);
        let mut highest_geodes: i32 = 0;

        work.push_front(initial_state);

        while let Some(w) = work.pop_front() {
            if w.ticks == total_ticks {
                highest_geodes = max(
                    *w.resources.get(&Resource::Geode).unwrap_or(&0),
                    highest_geodes,
                );
                continue;
            }

            if seen.contains(&w) {
                continue;
            }
            seen.insert(w.clone());

            let mut built = false;
            for (res, when) in w.sensible_ever_affordable_bots() {
                if w.ticks + when >= total_ticks {
                    // Unattainable within the time limit
                    continue;
                }

                let mut wn = w.clone();
                wn.tick_advance(when);
                wn.tick(Some(res));
                work.push_back(wn);
                built = true;
            }

            if !built && w.ticks < total_ticks {
                // Do nothing, skip forward
                let mut wn = w.clone();
                wn.tick_advance(total_ticks - wn.ticks);
                work.push_back(wn);
            }
        }

        println!(
            "  Blueprint {}: {} ({} unique paths)",
            self.id,
            highest_geodes,
            seen.len()
        );
        highest_geodes
    }

    fn quality_level(&self) -> i32 {
        self.id * self.search_optimal(24)
    }
}

fn parse(inp: &str) -> Blueprints {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
    }

    LINE_RE
        .captures_iter(inp)
        .map(|m| Blueprint {
            id: m[1].parse().unwrap(),
            cost: HashMap::from([
                (
                    Resource::Ore,
                    HashMap::from([(Resource::Ore, m[2].parse().unwrap())]),
                ),
                (
                    Resource::Clay,
                    HashMap::from([(Resource::Ore, m[3].parse().unwrap())]),
                ),
                (
                    Resource::Obsidian,
                    HashMap::from([
                        (Resource::Ore, m[4].parse().unwrap()),
                        (Resource::Clay, m[5].parse().unwrap()),
                    ]),
                ),
                (
                    Resource::Geode,
                    HashMap::from([
                        (Resource::Ore, m[6].parse().unwrap()),
                        (Resource::Obsidian, m[7].parse().unwrap()),
                    ]),
                ),
            ]),
        })
        .collect()
}

fn part1(blueprints: &Blueprints) -> i32 {
    blueprints.iter().map(|b| b.quality_level()).sum::<i32>()
}

fn part2(blueprints: &Blueprints) -> i32 {
    blueprints[0..3]
        .iter()
        .fold(1i32, |a, b| a * b.search_optimal(32))
}

fn main() {
    let inp = fs::read_to_string("inputs/19.txt").unwrap();

    let blueprints = parse(&inp);
    assert_eq!(inp.trim().lines().count(), blueprints.len());

    println!("Answer #1: {}", part1(&blueprints));
    println!("Answer #2: {}", part2(&blueprints));
}
