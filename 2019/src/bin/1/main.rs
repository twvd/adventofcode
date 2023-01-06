use std::fs;

fn accfuel(mass: i32) -> i32 {
    let mut i = mass;
    let mut fuel = 0;
    while i > 0 {
        let addfuel = (i / 3) - 2;
        if addfuel > 0 {
            fuel += addfuel;
        }
        i = addfuel;
    }
    fuel
}

fn main() {
    let inp = fs::read_to_string("inputs/1.txt").unwrap();

    let mass = Vec::<i32>::from_iter(inp.trim().lines().map(|l| l.trim().parse().unwrap()));

    println!(
        "Answer #1: {}",
        mass.iter().map(|&m| (m / 3) - 2).sum::<i32>()
    );
    println!(
        "Answer #2: {}",
        mass.iter().map(|&m| accfuel(m)).sum::<i32>()
    );
}
