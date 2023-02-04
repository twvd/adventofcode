use aoc_2019::intcode::{IntComputer, Word};
use itertools::iproduct;
use std::fs;

fn run(inp: &str, x: Word, y: Word) -> bool {
    *IntComputer::from_str(&inp)
        .unwrap()
        .input(&[x, y])
        .run()
        .unwrap()
        .output()
        .get(0)
        .unwrap()
        == 1
}

#[allow(dead_code)]
fn printbeam(inp: &str) {
    for y in 0..30 {
        for x in 0..60 {
            print!("{}", if run(inp, x, y) { "#" } else { "." });
        }
        println!("");
    }
}

fn part1(inp: &str) -> usize {
    iproduct![0..50, 0..50]
        .map(|(x, y)| run(inp, x, y))
        .filter(|&b| b)
        .count()
}

fn part2(inp: &str) -> Word {
    const EDGE: Word = 99;
    let mut first_x: Word = 0;

    for y in EDGE.. {
        let mut seen_x = false;
        for x in first_x.. {
            if !run(inp, x, y) {
                if seen_x {
                    break;
                }
                continue;
            }
            if !seen_x {
                first_x = x;
                seen_x = true;
            }
            if !run(inp, x + EDGE, y) {
                break;
            }
            if !run(inp, x, y + EDGE) {
                continue;
            }
            if !run(inp, x + EDGE, y + EDGE) {
                break;
            }
            return x * 10000 + y;
        }
    }

    unreachable!();
}

fn main() {
    let inp = fs::read_to_string("inputs/19.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
