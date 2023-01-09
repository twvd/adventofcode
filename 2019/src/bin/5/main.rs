use aoc_2019::intcode::{IntComputer, Word};
use std::fs;

fn part1(inp: &str) -> Word {
    let res = IntComputer::from_str(&inp)
        .unwrap()
        .input(&[1])
        .run()
        .unwrap()
        .output();

    // All diagnostic output 0
    assert!(res[..(res.len() - 1)].iter().all(|&n| n == 0));

    // Diagnostic code
    res[res.len() - 1]
}

fn part2(inp: &str) -> Word {
    *IntComputer::from_str(&inp)
        .unwrap()
        .input(&[5])
        .run()
        .unwrap()
        .output()
        .get(0)
        .unwrap()
}

fn main() {
    let inp = fs::read_to_string("inputs/5.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
