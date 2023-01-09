#![feature(is_sorted)]
use std::collections::HashMap;

fn main() {
    let inp = 134564..585159;

    let cnts = Vec::<HashMap<char, usize>>::from_iter(
        inp.into_iter()
            .map(|i| i.to_string().chars().collect::<Vec<char>>())
            .filter(|cs| cs.is_sorted())
            .map(|cs| {
                HashMap::from_iter(
                    cs.iter()
                        .map(|&c| (c, cs.iter().filter(|&&d| d == c).count())),
                )
            }),
    );

    println!(
        "Answer #1: {}",
        cnts.iter()
            .filter(|&cs| *cs.values().max().unwrap() > 1)
            .count()
    );
    println!(
        "Answer #2: {}",
        cnts.iter()
            .filter(|&cs| cs.values().position(|&c| c == 2).is_some())
            .count()
    );
}
