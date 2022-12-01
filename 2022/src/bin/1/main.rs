use std::fs;

fn main() {
    let inp = fs::read_to_string("inputs/1.txt")
        .unwrap();

    let mut elves: Vec<Vec<u32>> = Vec::new();
    for e in inp.split("\n\n") {
        elves.push(e
            .trim()
            .split("\n")
            .map(|x| x.parse::<u32>().unwrap())
            .collect()
        );
    }

    let mut sums: Vec<u32> = elves
        .iter()
        .map(|x| x.iter().sum())
        .collect();
    sums.sort();
    sums.reverse();

    println!("Answer #1: {}",
        sums.iter().max().unwrap());
    println!("Answer #2: {}",
        sums[..3].iter().sum::<u32>());
}
