use aoc_2019::stuff::{Direction, Point};
use std::collections::HashMap;
use std::fs;

type Pt = Point<i64>;
type Move = (Direction, i64);
type Moves = Vec<Move>;

const NWIRES: usize = 2;
const ORIGIN: Pt = Point(0, 0);

fn parse(inp: &str) -> Vec<Moves> {
    inp.trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|m| {
                    (
                        Direction::from_ascii(m.chars().nth(0).unwrap()),
                        m[1..].parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn main() {
    let inp = fs::read_to_string("inputs/3.txt").unwrap();

    let wires = parse(&inp);

    let mut grid = HashMap::<Pt, [Option<usize>; NWIRES]>::new();
    let mut intersections = HashMap::<Pt, usize>::new();

    assert_eq!(wires.len(), NWIRES);
    for (wireidx, wire) in wires.iter().enumerate() {
        let mut steps: usize = 0;
        let mut pt: Pt = Point(0, 0);
        let otheridx = wireidx ^ 1;

        for mv in wire {
            for _ in 0..mv.1 {
                grid.entry(pt).or_insert([None, None])[wireidx] = Some(steps);
                if let Some(othersteps) = grid[&pt][otheridx] {
                    if pt != ORIGIN {
                        intersections.insert(pt, steps + othersteps);
                    }
                }

                pt = pt.move_dir(mv.0, 1);
                steps += 1;
            }
        }
    }

    dbg!(&intersections);

    println!(
        "Answer #1: {}",
        intersections
            .keys()
            .map(|p| p.manhattan(ORIGIN))
            .min()
            .unwrap()
    );
    println!("Answer #2: {}", intersections.values().min().unwrap());
}
