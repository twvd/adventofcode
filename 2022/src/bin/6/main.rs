use itertools::Itertools;
use std::fs;

fn find_marker(s: &str, marker_len: usize) -> Option<usize> {
    for (i, w) in s.as_bytes().windows(marker_len).enumerate() {
        if w.iter().unique().eq(w) {
            return Some(i + marker_len);
        }
    }

    None
}

fn main() {
    let inp = fs::read_to_string("inputs/6.txt").unwrap();
    let inp_clean = inp.trim();

    println!("Answer #1: {}", find_marker(&inp_clean, 4).unwrap());
    println!("Answer #2: {}", find_marker(&inp_clean, 14).unwrap());
}
