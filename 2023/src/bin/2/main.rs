use std::cmp::max;
use std::fs;

struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

struct Subset {
    r: usize,
    g: usize,
    b: usize,
}

impl Game {
    fn min_power(&self) -> usize {
        max(1, self.subsets.iter().max_by_key(|s| s.r).unwrap().r)
            * max(1, self.subsets.iter().max_by_key(|s| s.g).unwrap().g)
            * max(1, self.subsets.iter().max_by_key(|s| s.b).unwrap().b)
    }
}

type Games = Vec<Game>;

fn parse_subset(raw: &str) -> Subset {
    let mut subset = Subset { r: 0, g: 0, b: 0 };
    for (num, color) in raw
        .split(',')
        .map(|i| i.trim().split_once(' ').unwrap())
        .map(|(n, c)| (n.parse::<usize>().unwrap(), c.trim()))
    {
        match color {
            "red" => subset.r += num,
            "green" => subset.g += num,
            "blue" => subset.b += num,
            _ => unreachable!(),
        }
    }
    subset
}

fn parse_game(line: &str) -> Game {
    let id = line
        .chars()
        .skip(5) // "Game "
        .take_while(|&c| c != ':')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let raw_subsets = line.split(':').nth(1).unwrap().split(';');

    Game {
        id,
        subsets: raw_subsets.map(|i| parse_subset(i.trim())).collect(),
    }
}

fn main() {
    let f = fs::read_to_string("inputs/2.txt").unwrap();
    let inp = f.trim().lines().collect::<Vec<_>>();

    let games = inp.iter().map(|i| parse_game(i)).collect::<Games>();

    let part1 = games
        .iter()
        .filter(|g| {
            g.subsets
                .iter()
                .all(|s| s.r <= 12 && s.g <= 13 && s.b <= 14)
        })
        .fold(0, |a, g| a + g.id);
    let part2 = games.iter().fold(0, |a, g| a + g.min_power());

    println!("Answer #1: {}", part1);
    println!("Answer #2: {}", part2);
}
