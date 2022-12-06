use itertools::Itertools;
use std::fs;

fn find_marker(s: &str, marker_len: usize) -> usize {
    s.as_bytes()
        .windows(marker_len)
        .position(|w| w.iter().unique().eq(w))
        .unwrap()
        + marker_len
}

fn main() {
    let inp = fs::read_to_string("inputs/6.txt").unwrap();

    println!("Answer #1: {}", find_marker(&inp, 4));
    println!("Answer #2: {}", find_marker(&inp, 14));
}
