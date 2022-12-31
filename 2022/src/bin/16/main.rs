use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinding::prelude::bfs;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Index;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve<'v> {
    name: &'v str,
    flowrate: i32,
    routes: Vec<&'v str>,
}

type BfsCache = HashMap<(String, String), Vec<String>>;

#[derive(Debug)]
struct Valves<'v> {
    valves: HashMap<&'v str, Valve<'v>>,
    bfscache: RefCell<BfsCache>,
}

impl<'v> Index<&str> for Valves<'v> {
    type Output = Valve<'v>;
    fn index(&self, k: &str) -> &Self::Output {
        &self.valves[k]
    }
}

impl<'v> Valves<'v> {
    fn parse(inp: &str) -> Valves {
        lazy_static! {
            static ref LINERE: Regex = Regex::new(
                r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)"
            )
            .unwrap();
        }

        Valves {
            bfscache: RefCell::new(BfsCache::with_capacity(500)),
            valves: HashMap::from_iter(LINERE.captures_iter(inp).map(|m| {
                (
                    m.get(1).unwrap().as_str(),
                    Valve {
                        name: m.get(1).unwrap().as_str(),
                        flowrate: m.get(2).unwrap().as_str().parse().unwrap(),
                        routes: m.get(3).unwrap().as_str().split(", ").collect(),
                    },
                )
            })),
        }
    }

    fn bfs_to(self: &Self, from: &str, destination: &str) -> Vec<String> {
        if !self
            .bfscache
            .borrow()
            .contains_key(&(from.to_owned(), destination.to_owned()))
        {
            self.bfscache.borrow_mut().insert(
                (from.to_owned(), destination.to_owned()),
                bfs(
                    &from,
                    |p| self.valves[p].routes.to_owned(),
                    |&p| p == destination,
                )
                .unwrap()
                .into_iter()
                .skip(1)
                .map(|s| String::from(s))
                .collect::<Vec<String>>(),
            );
        }

        self.bfscache
            .borrow()
            .get(&(from.to_owned(), destination.to_owned()))
            .unwrap()
            .to_owned()
    }

    fn distance_to(self: &Self, from: &str, dest: &str) -> i32 {
        self.bfs_to(from, dest).len() as i32 + 1
    }

    fn split_up(self: &Self, actors: usize) -> Vec<Vec<&'v Valve>> {
        let vs = self
            .valves
            .values()
            .filter(|v| v.flowrate > 0)
            .collect::<Vec<&'v Valve>>();
        let vslen = vs.len();
        vs.into_iter().combinations(vslen / actors).collect()
    }
}

#[derive(Debug, Clone)]
struct State<'s, 'v> {
    position: String,
    valves_open: HashSet<&'s Valve<'v>>,
    minute: i32,
    valves: &'s Valves<'v>,
    total_flow: i32,
    history: Vec<String>,
    valves_usable: HashSet<&'s Valve<'v>>,
    endmin: i32,
}

impl<'s, 'v> State<'s, 'v> {
    fn new(valves: &'s Valves<'v>, usable: Vec<&'v Valve>, endmin: i32) -> State<'s, 'v> {
        State {
            position: String::from("AA"),
            valves_open: HashSet::from_iter(valves.valves.values().filter(|v| v.flowrate == 0)),
            minute: 0,
            valves: &valves,
            total_flow: 0,
            history: vec![],
            valves_usable: HashSet::from_iter(usable.into_iter()),
            endmin,
        }
    }

    fn current_valve(self: &Self) -> &Valve {
        &self.valves[self.position.as_str()]
    }

    fn move_to(self: &mut Self, destination: &str) {
        if self.current_valve().routes.contains(&destination) {
            self.move_to_one(destination);
        } else {
            for step in self
                .valves
                .bfs_to(self.position.as_str(), destination)
                .iter()
            {
                self.move_to_one(&step);
            }
        }
    }

    fn move_to_one(self: &mut Self, destination: &str) {
        if self.minute >= self.endmin {
            return;
        }

        assert!(self.current_valve().routes.contains(&destination));
        assert_ne!(destination, self.position.as_str());
        self.inc_minute();
        self.position = String::from(destination);
        self.history.push(String::from(&self.position));
    }

    fn open_valve(self: &mut Self) {
        if self.valves_open.contains(self.current_valve())
            || !self.valves_usable.contains(self.current_valve())
            || self.minute >= self.endmin
        {
            return;
        }
        self.inc_minute();
        self.valves_open
            .insert(&self.valves[self.position.as_str()]);
        self.history
            .push(String::from(&self.position.to_lowercase()));
    }

    fn closed_valves(self: &Self) -> Vec<&'s str> {
        self.valves
            .valves
            .iter()
            .filter_map(|(&n, &ref v)| {
                if self.valves_open.contains(&v) || !self.valves_usable.contains(&v) {
                    None
                } else {
                    Some(n)
                }
            })
            .collect::<_>()
    }

    fn flow_per_minute(self: &Self) -> i32 {
        self.valves_open.iter().fold(0, |acc, v| acc + v.flowrate)
    }

    fn inc_minute(self: &mut Self) {
        assert!(self.minute < self.endmin);
        self.total_flow += self.flow_per_minute();
        self.minute += 1;
    }

    fn advance_minutes(self: &mut Self, to: i32) {
        while self.minute < to {
            self.inc_minute();
        }
    }

    fn get_history(self: &Self) -> String {
        String::from(self.history.join(""))
    }

    fn next_stops(self: &Self) -> Vec<&str> {
        let mut stops: Vec<&str> = self.closed_valves().into_iter().collect();
        stops.sort_by_cached_key(|k| {
            self.valves[k].flowrate / self.valves.distance_to(self.position.as_str(), k)
        });
        stops
    }

    fn potential(self: &Self) -> i32 {
        assert!(self.minute <= self.endmin);
        let min_left = self.endmin - self.minute;
        self.total_flow
            + (self.flow_per_minute() * min_left)
            + (self
                .closed_valves()
                .iter()
                .fold(0, |acc, v| acc + self.valves[v].flowrate)
                * min_left)
    }
}

fn find_best<'a>(
    valves: &'a Valves,
    endmin: i32,
    usable_valves: Vec<&'a Valve>,
) -> Option<State<'a, 'a>> {
    let initial_state: State = State::new(&valves, usable_valves, endmin);
    let mut solves: Vec<State> = vec![];
    let mut max_solve: i32 = 0;
    let mut paths_followed: HashSet<String> = HashSet::with_capacity(50000);

    let mut work: Vec<State> = vec![initial_state];
    work.reserve(1000);

    while let Some(w) = work.pop() {
        if paths_followed.contains(&w.get_history()) {
            continue;
        }
        if w.potential() <= max_solve {
            continue;
        }

        let next_stops = w.next_stops();
        if next_stops.len() == 0 || w.minute == endmin {
            let mut wn = w.to_owned();
            wn.advance_minutes(endmin);
            if wn.total_flow > max_solve {
                max_solve = wn.total_flow;
            }
            solves.push(wn);
        } else {
            for n in next_stops {
                let mut wn = w.to_owned();
                wn.move_to(n);
                wn.open_valve();
                work.push(wn);
            }
        }
        paths_followed.insert(w.get_history());
    }

    solves.into_iter().max_by_key(|s| s.total_flow)
}

fn part1(valves: &Valves) -> i32 {
    find_best(&valves, 30, valves.valves.values().collect::<Vec<&Valve>>())
        .unwrap()
        .total_flow
}

fn part2(valves: &Valves) -> i32 {
    let mut combiscores: HashMap<Vec<&str>, i32> = HashMap::new();
    for vs in valves.split_up(2) {
        let key: Vec<&str> = vs.iter().map(|v| v.name).collect();
        combiscores.insert(
            key,
            find_best(&valves, 26, vs.to_owned()).unwrap().total_flow,
        );
    }

    let mut max_combi = 0;
    for (set1, score1) in combiscores.iter() {
        for (_, score2) in combiscores
            .iter()
            .filter(|(k, _)| !k.into_iter().any(|n| set1.contains(n)))
        {
            max_combi = max(max_combi, score1 + score2);
        }
    }

    max_combi
}

fn main() {
    let inp = fs::read_to_string("inputs/16.txt").unwrap();

    let valves = Valves::parse(&inp);

    assert_eq!(valves.valves.len(), inp.trim().lines().count());

    println!("Answer #1: {}", part1(&valves));
    println!("Answer #2: {}", part2(&valves));
}
