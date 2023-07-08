#![feature(extract_if)]
#![feature(slice_group_by)]

use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::VecDeque;
use std::f64;
use std::fs;

type Point = (i32, i32);

pub fn manhattan(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn clockwise_angle(a: &Point, b: &Point) -> f64 {
    let mut angle = (b.1 - a.1) as f64;
    angle = angle.atan2((b.0 - a.0) as f64);
    if angle < 0.0 {
        angle += 2.0 * std::f64::consts::PI;
    }
    angle * 180.0 / std::f64::consts::PI
}

fn parse(inp: &str) -> Vec<Point> {
    Vec::from_iter(inp.trim().lines().enumerate().flat_map(|(y, l)| {
        l.trim().char_indices().filter_map(move |(x, c)| match c {
            '#' => Some((x as i32, y as i32)),
            _ => None,
        })
    }))
}

fn find_optimal_station(asteroids: &[Point]) -> (Point, usize) {
    asteroids
        .iter()
        .map(|a| {
            (
                *a,
                asteroids
                    .iter()
                    .filter(|&b| a != b)
                    .map(|b| OrderedFloat(clockwise_angle(a, b)))
                    .unique()
                    .count(),
            )
        })
        .max_by_key(|(_, visible)| *visible)
        .unwrap()
}

fn part1(asteroids: &[Point]) -> usize {
    find_optimal_station(asteroids).1
}

fn part2(asteroids: &[Point]) -> i32 {
    fn do_sweep(asteroids: &[Point], station: &Point) -> VecDeque<Point> {
        let mut roid_angles = asteroids
            .iter()
            .filter(|&a| a != station)
            .map(|a| {
                (
                    a,
                    OrderedFloat((90.0f64 + clockwise_angle(station, a)) % 360.0f64),
                )
            })
            .collect::<Vec<_>>();
        roid_angles.sort_unstable_by_key(|&(_, ang)| ang);
        roid_angles
            .group_by(|(_, ang1), (_, ang2)| ang1 == ang2)
            .map(|grp| {
                grp.iter()
                    .min_by_key(|(pt, _)| manhattan(station, pt))
                    .unwrap()
            })
            .map(|(&p, _)| p)
            .collect::<VecDeque<_>>()
    }

    let (station, _) = find_optimal_station(asteroids);
    let mut sweep: VecDeque<Point> = VecDeque::new();
    let mut answer: i32 = 0;
    let mut roids = asteroids.to_vec();

    for _ in 0..200 {
        if sweep.is_empty() {
            sweep = do_sweep(&roids, &station);
            assert!(!sweep.is_empty());

            let _ = roids.extract_if(|i| sweep.contains(i));
        }

        let target = sweep.pop_front().unwrap();
        answer = target.0 * 100 + target.1
    }
    answer
}

fn main() {
    let inp = fs::read_to_string("inputs/10.txt").unwrap();

    let asteroids = parse(&inp);

    println!("{} asteroids", asteroids.len());
    println!("Answer #1: {}", part1(&asteroids));
    println!("Answer #2: {}", part2(&asteroids));
}
