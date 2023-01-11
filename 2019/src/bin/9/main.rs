use aoc_2019::intcode::{IntComputer, Word};
use std::fs;

fn run(inp: &str, mode: Word) -> Word {
    *IntComputer::from_str(&inp)
        .unwrap()
        .input(&[mode])
        .run()
        .unwrap()
        .output()
        .get(0)
        .unwrap()
}

fn part1(inp: &str) -> Word {
    run(inp, 1)
}

fn part2(inp: &str) -> Word {
    run(inp, 2)
}

fn main() {
    let inp = fs::read_to_string("inputs/9.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
