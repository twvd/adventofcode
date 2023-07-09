use aoc_2019::intcode::{IntComputer, Word};
use std::fs;

#[allow(dead_code)]
fn dump_output(out: &[Word]) {
    println!(
        "{}",
        String::from_iter(out.iter().map(|&w| char::from_u32(w as u32).unwrap()))
    );
}

fn run(prog: &str, instr: &[&str]) -> Option<Word> {
    IntComputer::from_str(prog)
        .ok()?
        .input_str(instr.join("\n").as_str())
        .input_str("\n")
        .run()
        .ok()?
        .output()
        .last()
        .copied()
}

#[allow(dead_code)]
fn run_dbg(prog: &str, instr: &[&str]) -> Option<Word> {
    println!(
        "{}",
        IntComputer::from_str(prog)
            .ok()?
            .input_str(instr.join("\n").as_str())
            .input_str("\n")
            .run()
            .ok()?
            .output_str()
            .unwrap()
    );
    None
}

#[rustfmt::skip]
fn part1(prog: &str) -> Word {
    run(
        prog,
        &[
            "NOT C J",
            "AND D J",

            "NOT A T",
            "OR T J",

            "WALK",
        ],
    )
    .unwrap()
}

#[rustfmt::skip]
fn part2(prog: &str) -> Word {
    run(
        prog,
        &[
            "NOT C J",

            "AND H J",
            "NOT B T",
            "OR T J",

            "OR E J",

            "NOT A T",
            "OR T J",

            "AND D J",

            "RUN",
        ],
    )
    .unwrap()
}

fn main() {
    let inp = fs::read_to_string("inputs/21.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
