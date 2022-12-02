use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum GameResult {
    Win = 6,
    Loss = 0,
    Draw = 3
}

#[derive(PartialEq, Copy, Clone)]
enum GameChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

fn translate_choice(c: char) -> Result<GameChoice, &'static str> {
    match c {
        'A' | 'X' => Ok(GameChoice::Rock),
        'B' | 'Y' => Ok(GameChoice::Paper),
        'C' | 'Z' => Ok(GameChoice::Scissors),
        _ => Err("Unknown value")
    }
}

fn translate_result(c: char) -> Result<GameResult, &'static str> {
    match c {
        'X' => Ok(GameResult::Loss),
        'Y' => Ok(GameResult::Draw),
        'Z' => Ok(GameResult::Win),
        _ => Err("Unknown value")
    }
}

fn turn(a: GameChoice, b: GameChoice) -> GameResult {
    if a == b {
        GameResult::Draw
    } else {
        match a {
            GameChoice::Rock => match b {
                GameChoice::Scissors => GameResult::Win,
                _ => GameResult::Loss
            },
            GameChoice::Paper => match b {
                GameChoice::Rock => GameResult::Win,
                _ => GameResult::Loss
            },
            GameChoice::Scissors => match b {
                GameChoice::Paper => GameResult::Win,
                _ => GameResult::Loss
            }
        }
    }
}

fn get_choice(choice: GameChoice, result: GameResult) -> GameChoice {
    if result == GameResult::Draw {
        choice
    } else {
        match choice {
            GameChoice::Rock => match result {
                GameResult::Win => GameChoice::Paper,
                _ => GameChoice::Scissors
            },
            GameChoice::Paper => match result {
                GameResult::Win => GameChoice::Scissors,
                _ => GameChoice::Rock
            },
            GameChoice::Scissors => match result {
                GameResult::Win => GameChoice::Rock,
                _ => GameChoice::Paper
            }
        }
    }
}

fn part1(inp: &[(char, char)]) -> u32 {
    let mut score: u32 = 0;
    for (copp, cme) in inp {
        let opp = translate_choice(*copp).unwrap();
        let me = translate_choice(*cme).unwrap();
        let result = turn(me, opp);

        score += me as u32 + result as u32;
    }

    score
}

fn part2(inp: &[(char, char)]) -> u32 {
    let mut score: u32 = 0;
    for (copp, cresult) in inp {
        let opp = translate_choice(*copp).unwrap();
        let result = translate_result(*cresult).unwrap();
        let me = get_choice(opp, result);

        score += me as u32 + result as u32;
    }

    score
}

fn main() {
    let inp = fs::read_to_string("inputs/2.txt")
        .unwrap()
        .lines()
        .map(|l| (
                l.chars().nth(0).unwrap(),
                l.chars().nth(2).unwrap()
            ))
        .collect::<Vec<(char, char)>>();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
