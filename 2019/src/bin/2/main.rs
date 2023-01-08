use aoc_2019::intcode::{IntComputer, Word};
use itertools::iproduct;
use std::fs;

fn run(inp: &str, one: Word, two: Word) -> Word {
    IntComputer::from_str(&inp)
        .unwrap()
        .write_mem(1, one)
        .write_mem(2, two)
        .run()
        .unwrap()
        .read_mem(0)
}

fn part1(inp: &str) -> Word {
    run(inp, 12, 2)
}

fn part2(inp: &str) -> Option<Word> {
    for (noun, verb) in iproduct![0..100, 0..100] {
        if run(inp, noun, verb) == 19690720 {
            return Some(100 * noun + verb);
        }
    }

    None
}

fn main() {
    let inp = fs::read_to_string("inputs/2.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp).unwrap());
}
