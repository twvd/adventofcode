use aho_corasick::AhoCorasick;
use std::fs;

fn main() {
    let f = fs::read_to_string("inputs/1.txt").unwrap();
    let inp = f.trim().lines().collect::<Vec<_>>();

    let mut part1 = 0;
    for i in &inp {
        let first = i
            .chars()
            .nth(i.find(|c: char| c.is_digit(10)).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last = i
            .chars()
            .nth(i.rfind(|c: char| c.is_digit(10)).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();

        part1 += last + first * 10;
    }

    let mut part2 = 0;
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let replace = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let aho = AhoCorasick::new(patterns).unwrap();
    let aho_rev = AhoCorasick::new(patterns.map(|s| s.chars().rev().collect::<String>())).unwrap();

    for line in &inp {
        let fwd = aho.replace_all(&line, replace);
        let rev = aho_rev.replace_all(&line.chars().rev().collect::<String>(), replace);
        let first = fwd
            .chars()
            .nth(fwd.find(|c: char| c.is_digit(10)).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last = rev
            .chars()
            .nth(rev.find(|c: char| c.is_digit(10)).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();

        part2 += last + first * 10;
    }

    println!("Answer #1: {}", part1);
    println!("Answer #2: {}", part2);
}
