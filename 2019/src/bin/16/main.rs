use core::iter;
use std::fs;

fn pattern_iter(elements: usize) -> impl Iterator<Item = i32> {
    [0, 1, 0, -1]
        .into_iter()
        .flat_map(move |n| iter::repeat(n).take(elements))
        .cycle()
        .skip(1)
}

fn fft_round(s: &str) -> String {
    String::from_iter((1..=s.len()).map(|i| {
        (s.chars()
            .zip(pattern_iter(i))
            .map(|(c, p)| (c.to_digit(10).unwrap() as i32 * p))
            .sum::<i32>()
            % 10)
            .abs()
            .to_string()
    }))
}

fn fft_iter(s: &str) -> impl Iterator<Item = String> {
    let mut state: String = s.to_string();
    iter::from_fn(move || {
        state = fft_round(state.as_str());
        Some(state.to_owned())
    })
}

fn part1(inp: &str) -> String {
    fft_iter(inp)
        .skip(99)
        .next()
        .unwrap()
        .chars()
        .take(8)
        .collect()
}

fn part2(inp: &str) -> String {
    let idx: usize = inp[0..7].parse().unwrap();
    let mut s: Vec<u32> = inp
        .repeat(10000)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    assert!(idx > inp.len() / 2);

    for _ in 0..100 {
        let mut sum = 0;
        for (i, c) in s.clone().into_iter().enumerate().rev() {
            if i < idx {
                break;
            }
            sum += c;
            s[i] = sum % 10;
        }
    }

    s.into_iter()
        .skip(idx)
        .take(8)
        .map(|i| i.to_string())
        .collect()
}

fn main() {
    let inp = fs::read_to_string("inputs/16.txt").unwrap();
    let inp = inp.trim();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
