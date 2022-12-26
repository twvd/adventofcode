#![feature(slice_partition_dedup)]

use core::ops::RangeInclusive;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

type Coord = (i32, i32);
type Elves = HashSet<Coord>;

type Proposal = Box<dyn Fn(&Coord, &Elves) -> Option<Coord>>;

#[derive(Copy, Clone)]
enum WindDir {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
    E,
    W,
}

fn movewd(c: &Coord, wd: WindDir) -> Coord {
    match wd {
        WindDir::N => (c.0, c.1 - 1),
        WindDir::NE => (c.0 + 1, c.1 - 1),
        WindDir::NW => (c.0 - 1, c.1 - 1),
        WindDir::S => (c.0, c.1 + 1),
        WindDir::SE => (c.0 + 1, c.1 + 1),
        WindDir::SW => (c.0 - 1, c.1 + 1),
        WindDir::E => (c.0 + 1, c.1),
        WindDir::W => (c.0 - 1, c.1),
    }
}

fn adjacent_elves(c: &Coord, e: &Elves, dirs: &[WindDir]) -> bool {
    dirs.iter().any(|&w| e.contains(&movewd(c, w)))
}

fn adjacent_elves_all(coord: &Coord, elves: &Elves) -> bool {
    adjacent_elves(
        coord,
        elves,
        &[
            WindDir::N,
            WindDir::NE,
            WindDir::E,
            WindDir::SE,
            WindDir::S,
            WindDir::SW,
            WindDir::W,
            WindDir::NW,
        ],
    )
}

fn parse(inp: &str) -> Elves {
    let mut ret = HashSet::new();
    for (y, l) in inp.lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            if c == '#' {
                ret.insert((x as i32, y as i32));
            }
        }
    }
    ret
}

fn rounds(in_elves: &Elves, rounds: usize) -> (usize, Elves) {
    let mut elves = in_elves.clone();
    let mut proposal_fns: VecDeque<Proposal> = VecDeque::from([
        Box::new(|c: &Coord, e: &Elves| {
            if !adjacent_elves(c, e, &[WindDir::N, WindDir::NE, WindDir::NW]) {
                Some(movewd(c, WindDir::N))
            } else {
                None
            }
        }) as Proposal,
        Box::new(|c: &Coord, e: &Elves| {
            if !adjacent_elves(c, e, &[WindDir::S, WindDir::SE, WindDir::SW]) {
                Some(movewd(c, WindDir::S))
            } else {
                None
            }
        }) as Proposal,
        Box::new(|c: &Coord, e: &Elves| {
            if !adjacent_elves(c, e, &[WindDir::W, WindDir::SW, WindDir::NW]) {
                Some(movewd(c, WindDir::W))
            } else {
                None
            }
        }) as Proposal,
        Box::new(|c: &Coord, e: &Elves| {
            if !adjacent_elves(c, e, &[WindDir::E, WindDir::SE, WindDir::NE]) {
                Some(movewd(c, WindDir::E))
            } else {
                None
            }
        }) as Proposal,
    ]);

    for round in 1..=rounds {
        let mut proposed_moves: Vec<(Coord, Coord)> = vec![]; // old location, new location

        for elfp in &elves {
            if !adjacent_elves_all(elfp, &elves) {
                continue;
            }

            for pfn in &proposal_fns {
                if let Some(newp) = (pfn)(elfp, &elves) {
                    assert!(!elves.contains(&newp));
                    proposed_moves.push((*elfp, newp));
                    break;
                }
            }
        }

        proposed_moves.sort_by_key(|(_, n)| n.clone());
        let (dedup, dups) = proposed_moves.partition_dedup_by_key(|(_, to)| to.clone());
        let dup_new = dups.iter().map(|(_, n)| n).collect::<Vec<&Coord>>();
        let final_moves = dedup
            .iter()
            .filter(|(_, n)| !dup_new.contains(&n))
            .collect::<Vec<_>>();

        if final_moves.len() == 0 {
            return (round, elves.clone());
        }

        for (old, new) in final_moves {
            elves.remove(&old);
            assert!(!elves.contains(new));
            elves.insert(*new);
        }

        assert_eq!(elves.len(), in_elves.len());
        proposal_fns.rotate_left(1);
    }

    (rounds, elves.clone())
}

fn smallest_rect(elves: &Elves) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    (
        elves.iter().min_by_key(|(x, _)| x).unwrap().0
            ..=elves.iter().max_by_key(|(x, _)| x).unwrap().0,
        elves.iter().min_by_key(|(_, y)| y).unwrap().1
            ..=elves.iter().max_by_key(|(_, y)| y).unwrap().1,
    )
}

fn smallest_rect_size(elves: &Elves) -> (i32, i32) {
    let max_x = elves.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_x = elves.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_y = elves.iter().max_by_key(|(_, y)| y).unwrap().1;
    let min_y = elves.iter().min_by_key(|(_, y)| y).unwrap().1;

    (max_x - min_x + 1, max_y - min_y + 1)
}

#[allow(dead_code)]
fn printelves(elves: &Elves) {
    let (rx, ry) = smallest_rect(&elves);

    println!("{:03} ->", rx.start());
    for y in ry {
        let mut s = String::new();
        for x in rx.clone() {
            if elves.contains(&(x, y)) {
                s += "#";
            } else {
                s += ".";
            }
        }
        println!("{:03} {}", y, s);
    }
    println!("-----------");
}

fn part1(elves: &Elves) -> usize {
    let (rounds, solved) = rounds(elves, 10);
    assert_eq!(rounds, 10);
    let (szx, szy) = smallest_rect_size(&solved);
    (szx as usize * szy as usize) - solved.len()
}

fn part2(elves: &Elves) -> usize {
    rounds(elves, usize::MAX).0
}

fn main() {
    let inp = fs::read_to_string("inputs/23.txt").unwrap();
    let elves = parse(&inp);

    println!("Answer #1: {}", part1(&elves));
    println!("Answer #2: {}", part2(&elves));
}
