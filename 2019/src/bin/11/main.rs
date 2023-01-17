use aoc_2019::intcode::{IntComputer, Word};
use aoc_2019::stuff::{Direction, Point};
use std::collections::HashMap;
use std::fs;

type Pt = Point<i64>;
type Panels = HashMap<Pt, Word>;

fn paint_hull(inp: &str, start: Word) -> Panels {
    let mut comp = IntComputer::from_str(&inp).unwrap();
    let mut loc: Pt = Point(0, 0);
    let mut face = Direction::Up;
    let mut panels = Panels::new();

    panels.insert(loc, start);

    loop {
        comp.input(&[*panels.get(&loc).unwrap_or(&0)]);
        comp.run_to_output_or_halt().unwrap();
        if comp.is_halted() {
            break;
        }

        panels.insert(loc, *comp.output().get(0).unwrap());

        comp.run_to_output_or_halt().unwrap();
        if comp.is_halted() {
            break;
        }

        face = face.rotate_90deg(match comp.output().get(0).unwrap() {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => unreachable!(),
        });
        loc = loc.move_dir(face, 1);
    }

    panels
}

fn part1(inp: &str) -> usize {
    paint_hull(inp, 0).len()
}

fn part2(inp: &str) -> String {
    let paintjob = paint_hull(inp, 1);
    let rx = paintjob.keys().min_by_key(|p| p.0).unwrap().0
        ..=paintjob.keys().max_by_key(|p| p.0).unwrap().0;
    let ry = paintjob.keys().min_by_key(|p| p.1).unwrap().1
        ..=paintjob.keys().max_by_key(|p| p.1).unwrap().1;

    ry.rev()
        .map(|y| {
            String::from_iter(rx.to_owned().map(|x| match paintjob.get(&Point(x, y)) {
                Some(0) | None => ' ',
                Some(1) => '#',
                _ => unreachable!(),
            }))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let inp = fs::read_to_string("inputs/11.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2:\n{}", part2(&inp));
}
