use core::ops::RangeInclusive;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs;

type Coord = (i32, i32);

fn manhattan(p1: Coord, p2: Coord) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    position: Coord,
}
type Blizzards = Vec<Blizzard>;

struct MapBase {
    start: Coord,
    end: Coord,
    xspan: RangeInclusive<i32>,
    yspan: RangeInclusive<i32>,

    mapticks: RefCell<Vec<MapTick>>,
    gen_blizzards: RefCell<Blizzards>,
}

// All coordinates that can NOT be moved to in a given tick
type MapTick = HashSet<Coord>;

impl MapBase {
    fn parse(inp: &str) -> Self {
        let lines = inp.trim().lines().map(|l| l.trim()).collect::<Vec<&str>>();

        // The span of the map (excluding walls)
        let yspan = 1i32..=(lines.len() as i32 - 2);
        let xspan = 1i32..=(lines[0].len() as i32 - 2);

        // Detect start/end positions in the top/bottom walls
        let start = (0, lines[0].chars().position(|c| c == '.').unwrap() as i32);
        let end = (
            lines[lines.len() - 1]
                .chars()
                .position(|c| c == '.')
                .unwrap() as i32,
            lines.len() as i32 - 1,
        );

        // Blizzard starting positions
        let mut blizzards = Blizzards::new();
        for (y, l) in lines.into_iter().enumerate() {
            for (x, c) in l.char_indices() {
                match c {
                    'v' => blizzards.push(Blizzard {
                        direction: Direction::South,
                        position: (x as i32, y as i32),
                    }),
                    '^' => blizzards.push(Blizzard {
                        direction: Direction::North,
                        position: (x as i32, y as i32),
                    }),
                    '<' => blizzards.push(Blizzard {
                        direction: Direction::West,
                        position: (x as i32, y as i32),
                    }),
                    '>' => blizzards.push(Blizzard {
                        direction: Direction::East,
                        position: (x as i32, y as i32),
                    }),
                    '.' | '#' => (),
                    _ => unreachable!(),
                }
            }
        }

        Self {
            start,
            end,
            xspan,
            yspan,
            mapticks: RefCell::new(vec![MapTick::from_iter(
                blizzards.iter().map(|b| b.position),
            )]),
            gen_blizzards: RefCell::new(blizzards),
        }
    }

    fn get_tick(&self, tick: usize) -> MapTick {
        // Keeps a cache of all generated MapTicks inside
        // the MapBase to speed up lookups of the state of
        // the blizzards at a specific tick.
        // A 'MapTick' is a list of BLOCKED positions for
        // that specific tick.

        let tickslen = self.mapticks.borrow().len();
        for _ in tickslen..=tick {
            let newblizz = Blizzards::from_iter(self.gen_blizzards.borrow().iter().map(|b| {
                let p = b.position;
                Blizzard {
                    direction: b.direction,
                    position: match b.direction {
                        Direction::North => (
                            p.0,
                            if p.1 - 1 < *self.yspan.start() {
                                *self.yspan.end()
                            } else {
                                p.1 - 1
                            },
                        ),
                        Direction::South => (
                            p.0,
                            if p.1 + 1 > *self.yspan.end() {
                                *self.yspan.start()
                            } else {
                                p.1 + 1
                            },
                        ),
                        Direction::East => (
                            if p.0 + 1 > *self.xspan.end() {
                                *self.xspan.start()
                            } else {
                                p.0 + 1
                            },
                            p.1,
                        ),
                        Direction::West => (
                            if p.0 - 1 < *self.xspan.start() {
                                *self.xspan.end()
                            } else {
                                p.0 - 1
                            },
                            p.1,
                        ),
                    },
                }
            }));
            self.mapticks
                .borrow_mut()
                .push(MapTick::from_iter(newblizz.iter().map(|b| b.position)));
            *self.gen_blizzards.borrow_mut() = newblizz;
        }
        self.mapticks.borrow().get(tick).unwrap().to_owned()
    }

    fn find_shortest_path(&self, start: &Coord, goal: &Coord, start_tick: usize) -> Option<usize> {
        let initial_state = SearchState::new(*start, start_tick);
        let mut work: VecDeque<SearchState> = VecDeque::from([initial_state]);
        let mut shortest_dist = i32::MAX;
        let mut visited: HashSet<(usize, Coord)> = HashSet::new();
        work.reserve(1000);

        while let Some(w) = work.pop_front() {
            let mt: &MapTick = &self.get_tick(w.tick);
            let dist = manhattan(w.position, *goal);

            if w.position == *goal {
                return Some(w.tick - 1);
            }

            // Skip duplicate paths within same tick
            if visited.contains(&(w.tick, w.position)) {
                continue;
            }
            visited.insert((w.tick, w.position));

            // Shorten runtime a bit by skipping paths that are
            // further away from the goal than others.
            if dist < shortest_dist {
                shortest_dist = dist;
            } else if dist > shortest_dist + 25 {
                continue;
            }

            if let Some(moves) = w.possible_steps(self, &mt, start, goal) {
                for m in moves {
                    let mut wn = w.clone();
                    wn.move_to(m);
                    work.push_back(wn);
                }
            }
        }
        None
    }
}

#[derive(Clone)]
struct SearchState {
    position: Coord,
    tick: usize,
}

impl SearchState {
    fn new(position: Coord, start_tick: usize) -> Self {
        Self {
            position,
            tick: start_tick,
        }
    }

    fn possible_steps(
        &self,
        map: &MapBase,
        mt: &MapTick,
        start: &Coord,
        goal: &Coord,
    ) -> Option<Vec<Coord>> {
        let ret = Vec::from_iter(
            [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
                .into_iter()
                .map(|(x, y)| (self.position.0 + x, self.position.1 + y))
                .filter(|(x, y)| {
                    *goal == (*x, *y)
                        || *start == (*x, *y)
                        || (map.xspan.contains(x)
                            && map.yspan.contains(y)
                            && !mt.contains(&(*x, *y)))
                }),
        );
        if ret.len() > 0 {
            Some(ret)
        } else {
            // We can't move and can't stay on the same spot,
            // we're dying an icey death!
            None
        }
    }

    fn move_to(&mut self, new_pos: Coord) {
        self.position = new_pos;
        self.tick += 1;
    }
}

fn part1(map: &MapBase) -> usize {
    map.find_shortest_path(&map.start.to_owned(), &map.end.to_owned(), 1)
        .unwrap()
}

fn part2(map: &MapBase) -> usize {
    let start = map.start.to_owned();
    let end = map.end.to_owned();
    [(start, end), (end, start), (start, end)]
        .into_iter()
        .fold(1, |min, (s, e)| {
            map.find_shortest_path(&s, &e, min).unwrap()
        })
}

fn main() {
    let inp = fs::read_to_string("inputs/24.txt").unwrap();

    let map = MapBase::parse(&inp);

    println!("Answer #1: {}", part1(&map));
    println!("Answer #2: {}", part2(&map));
}
