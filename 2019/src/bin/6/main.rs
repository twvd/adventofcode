use pathfinding::prelude::bfs;
use std::collections::{HashMap, HashSet};
use std::fs;

type Orbits<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse(inp: &str) -> Orbits {
    let mut ret = Orbits::new();

    for (a, b) in inp
        .trim()
        .lines()
        .map(|l| l.trim().split_once(")").unwrap())
    {
        ret.entry(a).or_insert(HashSet::new()).insert(b);
    }

    ret
}

fn part1(orbs: &Orbits) -> usize {
    fn traverse(orbs: &Orbits, pos: &str, depth: usize) -> usize {
        match orbs.get(pos) {
            Some(orb) => orb
                .iter()
                .map(|o| traverse(orbs, o, depth + 1) + depth)
                .sum(),
            None => 0,
        }
    }

    traverse(orbs, "COM", 1)
}

fn part2(orbits: &Orbits) -> usize {
    let mut paths = orbits.clone();
    for (k, v) in orbits {
        for i in v {
            paths.entry(i).or_insert(HashSet::new()).insert(k);
        }
    }

    // - 2 to remove YOU and SAN
    // - 1 because we are counting the EDGES, not the nodes
    bfs(&"YOU", |o| paths[o].to_owned(), |o| o == &"SAN")
        .unwrap()
        .len()
        - 2
        - 1
}

fn main() {
    let inp = fs::read_to_string("inputs/6.txt").unwrap();
    let orbits = parse(&inp);

    println!("Answer #1: {}", part1(&orbits));
    println!("Answer #2: {}", part2(&orbits));
}
