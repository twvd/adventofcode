use itertools::Itertools;
use std::fs;

fn priority(c: char) -> Result<u8, &'static str> {
    match c {
        'a'..='z' => Ok(c as u8 - 'a' as u8 + 1),
        'A'..='Z' => Ok(c as u8 - 'A' as u8 + 27),
        _ => Err("Invalid character"),
    }
}

fn sum_priorities(items: &Vec<char>) -> u32 {
    items.iter().map(|x| priority(*x).unwrap() as u32).sum()
}

fn part1(lines: &Vec<&str>) -> u32 {
    let mut items: Vec<char> = Vec::new();

    for l in lines {
        assert_eq!(l.len() % 2, 0);

        let comp = l.split_at(l.len() / 2);

        let mut eq = String::from(comp.0);
        eq.retain(|c| comp.1.find(c).is_some());

        items.extend(eq.chars().unique());
    }

    sum_priorities(&items)
}

fn part2(lines: &Vec<&str>) -> u32 {
    let mut items: Vec<char> = Vec::new();

    for grp in lines.chunks(3) {
        let mut grpi = grp.iter();
        let mut eq = String::from(*grpi.next().unwrap());

        for g in grpi {
            eq.retain(|c| g.find(c).is_some());
        }
        items.extend(eq.chars().unique());
    }

    sum_priorities(&items)
}

fn main() {
    assert_eq!(priority('a').unwrap(), 1);
    assert_eq!(priority('z').unwrap(), 26);
    assert_eq!(priority('A').unwrap(), 27);
    assert_eq!(priority('Z').unwrap(), 52);

    let inp = fs::read_to_string("inputs/3.txt").unwrap();
    let lines: Vec<&str> = inp.trim().split('\n').collect();

    println!("Answer #1: {}", part1(&lines));
    println!("Answer #2: {}", part2(&lines));
}
