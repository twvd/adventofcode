use aoc_2019::intcode::{IntComputer, Word};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::fs;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, ToPrimitive)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i64, i64);

impl Point {
    fn step(&self, step: Direction) -> Point {
        match step {
            Direction::North => Point(self.0, self.1 - 1),
            Direction::South => Point(self.0, self.1 + 1),
            Direction::West => Point(self.0 - 1, self.1),
            Direction::East => Point(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, FromPrimitive, Eq, PartialEq, Copy, Clone)]
enum StepResult {
    Wall = 0,
    Moved = 1,
    Oxygen = 2,
}

#[derive(Debug)]
struct Step {
    comp: IntComputer,
    pos: Point,
    depth: i64,
    next_step: Direction,
}

impl Step {
    fn new(comp: IntComputer, pos: Point, next_step: Direction, depth: i64) -> Self {
        Self {
            comp,
            pos,
            depth,
            next_step,
        }
    }

    fn deepen(&self, next_step: Direction) -> Self {
        Self::new(
            self.comp.clone(),
            self.next_pos(),
            next_step,
            self.depth + 1,
        )
    }

    fn restart(&self, next_step: Direction) -> Self {
        Self::new(self.comp.clone(), self.next_pos(), next_step, 1)
    }

    fn run(&mut self) -> StepResult {
        self.comp.input(&[self.next_step.to_i64().unwrap() as Word]);
        self.comp.run_to_output().unwrap();
        let res = StepResult::from_i64(*self.comp.output().get(0).unwrap() as i64).unwrap();
        res
    }

    fn next_pos(&self) -> Point {
        self.pos.step(self.next_step)
    }
}

fn find_generator(inp: &str) -> Option<Step> {
    let mut explore: VecDeque<Step> = VecDeque::from_iter(
        Direction::iter()
            .map(|d| Step::new(IntComputer::from_str(&inp).unwrap(), Point(0, 0), d, 1)),
    );
    let mut visited: HashSet<Point> = HashSet::new();

    while let Some(mut e) = explore.pop_front() {
        match e.run() {
            StepResult::Moved => {
                for d in Direction::iter() {
                    if visited.contains(&e.next_pos().step(d)) {
                        continue;
                    }
                    explore.push_back(Step::deepen(&e, d));
                    visited.insert(e.next_pos().step(d));
                }
            }
            StepResult::Oxygen => {
                return Some(e);
            }
            StepResult::Wall => {}
        }
    }

    None
}

fn part1(inp: &str) -> Option<i64> {
    Some(find_generator(inp)?.depth)
}

fn part2(inp: &str) -> Option<i64> {
    let gen = find_generator(inp)?;
    let mut explore: VecDeque<Step> =
        VecDeque::from_iter(Direction::iter().map(|d| gen.restart(d)));
    let mut visited: HashSet<Point> = HashSet::new();
    let mut maxdepth = 0i64;

    while let Some(mut e) = explore.pop_front() {
        maxdepth = max(maxdepth, e.depth);

        if e.run() == StepResult::Moved {
            for d in Direction::iter() {
                if visited.contains(&e.next_pos().step(d)) {
                    continue;
                }
                explore.push_back(e.deepen(d));
                visited.insert(e.next_pos().step(d));
            }
        }
    }

    Some(maxdepth)
}

fn main() {
    let inp = fs::read_to_string("inputs/15.txt").unwrap();

    println!("Answer #1: {}", part1(&inp).unwrap());
    println!("Answer #2: {}", part2(&inp).unwrap());
}
