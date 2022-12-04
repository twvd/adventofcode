use std::fs;
use std::ops::Range;

type GroupAssign = [Range<u32>; 2];

fn part1(groups: &Vec<GroupAssign>) -> usize {
    groups
        .iter()
        .filter(|g|
            (g[0].start >= g[1].start && g[0].end <= g[1].end) ||
            (g[1].start >= g[0].start && g[1].end <= g[0].end))
        .collect::<Vec<&GroupAssign>>()
        .len()
}

fn part2(groups: &Vec<GroupAssign>) -> usize {
    groups
        .iter()
        .filter(|g| g[0].start <= g[1].end && g[0].end >= g[1].start)
        .collect::<Vec<&GroupAssign>>()
        .len()
}

fn main() {
    let inp = fs::read_to_string("inputs/4.txt")
        .unwrap();

    let mut groups: Vec<GroupAssign> = Vec::new();
    for l in inp.lines() {
        let mut spl = l
            .trim()
            .split(|c| c == ',' || c == '-')
            .map(|x| x.parse::<u32>().unwrap());

        groups.push([
            spl.next().unwrap()..spl.next().unwrap(),
            spl.next().unwrap()..spl.next().unwrap()
        ]);
    }

    println!("Answer #1: {}", part1(&groups));
    println!("Answer #2: {}", part2(&groups));
}
