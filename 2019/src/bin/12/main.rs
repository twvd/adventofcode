use std::collections::HashMap;
use std::fs;
use std::ops::{Add, AddAssign, Index};

use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point3D {
    p: [i64; 3],
}

impl Point3D {
    fn new() -> Self {
        Self { p: [0, 0, 0] }
    }

    fn new_xyz(x: i64, y: i64, z: i64) -> Self {
        Self { p: [x, y, z] }
    }

    fn sum_abs(&self) -> i64 {
        self.p.iter().map(|&p| p.abs()).sum()
    }
}

impl Index<usize> for Point3D {
    type Output = i64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.p[i]
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            p: [
                self.p[0] + other.p[0],
                self.p[1] + other.p[1],
                self.p[2] + other.p[2],
            ],
        }
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Moon {
    pos: Point3D,
    velocity: Point3D,
    initial_pos: Point3D,
}

impl Moon {
    fn new(pos: Point3D) -> Moon {
        Moon {
            pos,
            initial_pos: pos,
            velocity: Point3D::new(),
        }
    }

    fn gravity(&mut self, other: &Moon) {
        fn grav(s: i64, o: i64) -> i64 {
            if s == o {
                0
            } else if o > s {
                1
            } else {
                -1
            }
        }

        self.velocity += Point3D::new_xyz(
            grav(self.pos[0], other.pos[0]),
            grav(self.pos[1], other.pos[1]),
            grav(self.pos[2], other.pos[2]),
        );
    }

    fn velocity(&mut self) {
        self.pos += self.velocity;
    }

    fn energy(&self) -> i64 {
        self.pos.sum_abs() * self.velocity.sum_abs()
    }

    fn is_cycled(&self, dim: usize) -> bool {
        self.velocity[dim] == 0 && self.pos[dim] == self.initial_pos[dim]
    }
}

fn part1(in_moons: &[Moon]) -> i64 {
    let mut moons: HashMap<usize, Moon> =
        HashMap::from_iter(in_moons.into_iter().copied().enumerate());

    for _ in 0..1000 {
        let snapshot = &moons.clone();

        for mnidx in snapshot.keys().combinations(2) {
            moons
                .get_mut(&mnidx[0])
                .unwrap()
                .gravity(&snapshot[&mnidx[1]]);
            moons
                .get_mut(&mnidx[1])
                .unwrap()
                .gravity(&snapshot[&mnidx[0]]);
        }

        for m in moons.values_mut() {
            m.velocity();
        }
    }

    moons.values().map(|m| m.energy()).sum()
}

fn part2(in_moons: &[Moon]) -> i128 {
    let mut moons: HashMap<usize, Moon> =
        HashMap::from_iter(in_moons.into_iter().copied().enumerate());
    let mut completion: [Option<i128>; 3] = [None; 3];

    'out: for steps in 0i128.. {
        if steps > 0 {
            for dim in 0..3 {
                if completion[dim].is_some() {
                    continue;
                }

                if !moons.values().all(|m| m.is_cycled(dim)) {
                    continue;
                }

                completion[dim] = Some(steps);

                if completion.iter().all(|i| i.is_some()) {
                    break 'out;
                }
            }
        }

        let snapshot = &moons.clone();

        for mnidx in snapshot.keys().combinations(2) {
            moons
                .get_mut(&mnidx[0])
                .unwrap()
                .gravity(&snapshot[&mnidx[1]]);
            moons
                .get_mut(&mnidx[1])
                .unwrap()
                .gravity(&snapshot[&mnidx[0]]);
        }

        for m in moons.values_mut() {
            m.velocity();
        }
    }

    completion
        .into_iter()
        .flatten()
        .reduce(|acc, i| lcm(acc, i))
        .unwrap()
}

fn parse(inp: &str) -> Vec<Moon> {
    lazy_static! {
        static ref PTRE: Regex = Regex::new(r"<x=(\-?\d+), y=(\-?\d+), z=(\-?\d+)>").unwrap();
    }
    PTRE.captures_iter(inp)
        .map(|cap| {
            Moon::new(Point3D::new_xyz(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            ))
        })
        .collect::<Vec<_>>()
}

fn main() {
    let inp = fs::read_to_string("inputs/12.txt").unwrap();

    let moons = parse(&inp);

    println!("Answer #1: {}", part1(&moons));
    println!("Answer #2: {}", part2(&moons));
}
