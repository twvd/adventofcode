use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Match;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeBounds;

type Point = (i32, i32);
type Sensors = HashMap<Point, Point>; // K = sensor, V = beacon

fn manhattan(p1: Point, p2: Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn diamond_edges(center: &Point, radius: i32) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];

    for dx in 0..radius {
        let dy = radius - dx;
        ret.push((center.0 - dx, center.1 - dy));
        ret.push((center.0 - dx, center.1 + dy));
        ret.push((center.0 + dx, center.1 - dy));
        ret.push((center.0 + dx, center.1 + dy));
    }
    ret
}

fn check_coverage(sensors: &Sensors, pt: &Point) -> bool {
    sensors
        .iter()
        .any(|(s, b)| manhattan(*s, *pt) <= manhattan(*s, *b))
}

fn tuning_freq(pt: &Point) -> u128 {
    pt.0 as u128 * 4000000 + pt.1 as u128
}

fn parse_input(inp: &str) -> Sensors {
    fn getint(g: Option<Match<'_>>) -> i32 {
        g.unwrap().as_str().parse::<i32>().unwrap()
    }

    lazy_static! {
        static ref SENSORRE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }

    let mut ret: Sensors = HashMap::new();

    for m in SENSORRE.captures_iter(inp) {
        ret.insert(
            (getint(m.get(1)), getint(m.get(2))),
            (getint(m.get(3)), getint(m.get(4))),
        );
    }

    assert_eq!(ret.len(), inp.lines().count());

    ret
}

fn part1(sensors: &Sensors, search_y: i32) -> usize {
    let beacons_y = sensors
        .values()
        .filter(|(_, y)| *y == search_y)
        .unique()
        .count();
    let max_dist: i32 = sensors
        .iter()
        .map(|(s, b)| manhattan(*s, *b))
        .max()
        .unwrap();

    let x_min: i32 = sensors.keys().map(|(x, _)| x).min().unwrap() - max_dist;
    let x_max: i32 = sensors.keys().map(|(x, _)| x).max().unwrap() + max_dist;

    (x_min..=x_max)
        .filter(|x| check_coverage(sensors, &(*x, search_y)))
        .count()
        - beacons_y
}

fn part2<R: RangeBounds<i32>>(sensors: &Sensors, search_area: R) -> Option<u128> {
    // Scan along the outer search borders of each scanner for an uncovered spot
    for (s, b) in sensors {
        let dist = manhattan(*s, *b) + 1;

        for p in diamond_edges(s, dist)
            .iter()
            .filter(|(x, y)| search_area.contains(x) && search_area.contains(y))
        {
            if !check_coverage(&sensors, &p) {
                return Some(tuning_freq(&p));
            }
        }
    }

    None
}

fn main() {
    let inp = fs::read_to_string("inputs/15.txt").unwrap();

    let sensors = parse_input(&inp);

    println!("Answer #1: {}", part1(&sensors, 2000000));
    println!("Answer #2: {}", part2(&sensors, 0..=4000000).unwrap());
}
