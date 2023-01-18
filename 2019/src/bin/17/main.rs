use aoc_2019::intcode::{IntComputer, Word};
use aoc_2019::stuff::{Direction, Point};
use std::collections::HashSet;
use std::fs;
use strum::IntoEnumIterator;

type Pt = Point<i64>;

fn feed_to_map(feed: &str) -> HashSet<Pt> {
    feed.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim().char_indices().filter_map(move |(x, c)| match c {
                '#' => Some(Point(x as i64, y as i64)),
                _ => None,
            })
        })
        .collect()
}

fn dump_output(out: &[Word]) {
    println!(
        "{}",
        String::from_iter(out.iter().map(|&w| char::from_u32(w as u32).unwrap()))
    );
}

fn part1(inp: &str) -> i64 {
    let points = feed_to_map(inp);
    points
        .iter()
        .filter(|p| Direction::iter().all(|d| points.contains(&p.move_dir(d, 1))))
        .fold(0, |acc, p| acc + (p.0 * p.1))
}

fn vacuum_computer(prog: &str, main: &str, movfunc: &[&str], camera: bool) -> IntComputer {
    assert!(main.len() <= 20);
    assert!(movfunc.iter().all(|f| f.len() <= 20));

    let mut comp = IntComputer::from_str(prog).unwrap();

    comp.write_mem(0, 2)
        .input_str((String::from(main) + "\n").as_str())
        .input_str(
            movfunc
                .iter()
                .fold(String::new(), |acc, f| acc + f + "\n")
                .as_str(),
        )
        .input_str(if camera { "y\n" } else { "n\n" });

    comp
}

fn part2(prog: &str, map: &str) -> Word {
    let mut comp = vacuum_computer(prog, "A,B", &["L,4", "L,L,4", "4,R"], true);

    comp.run().unwrap();

    let out = comp.output();
    //dump_output(&out[..(out.len() - 2)]);

    todo!();

    out[out.len() - 1]
}

fn main() {
    let inp = fs::read_to_string("inputs/17.txt").unwrap();

    let map = String::from_iter(
        IntComputer::from_str(&inp)
            .unwrap()
            .run()
            .unwrap()
            .output()
            .iter()
            .map(|&c| char::from_u32(c as u32).unwrap()),
    );

    println!("Answer #1: {}", part1(&map));
    println!("Answer #2: {}", part2(&inp, &map));
}
