use aoc_2019::intcode::{IntComputer, Word};
use itertools::Itertools;
use std::fs;

fn part1(inp: &str) -> Word {
    (0..5)
        .permutations(5)
        .map(|phases| {
            phases.into_iter().fold(0, |signal, phase| {
                *IntComputer::from_str_input(&inp, &[phase, signal])
                    .unwrap()
                    .run_to_output()
                    .unwrap()
                    .output()
                    .get(0)
                    .unwrap()
            })
        })
        .max()
        .unwrap()
}

fn part2(inp: &str) -> Word {
    (5..10)
        .permutations(5)
        .map(|phases| {
            let mut comps = phases
                .iter()
                .map(|phase| IntComputer::from_str_input(&inp, &[*phase]).unwrap())
                .collect::<Vec<IntComputer>>();
            let mut signal = 0;
            while !comps.iter().any(|c| c.is_halted()) {
                for c in &mut comps {
                    if let Some(new_signal) = c
                        .input(&[signal])
                        .run_to_output_or_halt()
                        .unwrap()
                        .output()
                        .get(0)
                    {
                        signal = *new_signal;
                    }
                }
            }
            signal
        })
        .max()
        .unwrap()
}

fn main() {
    let inp = fs::read_to_string("inputs/7.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
