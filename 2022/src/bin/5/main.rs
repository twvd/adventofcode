use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}
type Moves = Vec<Move>;

fn parse_initial(inp: &Vec<&str>) -> Stacks {
    let mut inp_mut = inp.to_vec();
    let num_stacks: usize = inp_mut
        .pop()
        .unwrap()
        .chars()
        .filter(|c| *c != ' ')
        .collect::<Vec<char>>()
        .len();

    let mut stacks: Stacks = vec![vec![]; num_stacks];

    // Flip stacks in the input now so they end up with the
    // top at the end of the vector later.
    inp_mut.reverse();

    for l in inp_mut {
        for i in 0..num_stacks {
            // [x] [y]
            // 0123456
            let offset: usize = (i * 4) + 1;

            if offset > l.len() {
                break;
            }

            let c = l.chars().nth(offset).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    stacks
}

fn parse_moves(inp: &Vec<&str>) -> Moves {
    lazy_static! {
        static ref MOVE_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    MOVE_REGEX
        .captures_iter(&inp.join(""))
        .filter_map(|cap| {
            let parse = |n| cap.get(n).unwrap().as_str().parse::<usize>().unwrap();

            Some(Move {
                count: parse(1),
                from: parse(2) - 1, // correct for zero-based offset
                to: parse(3) - 1,   // correct for zero-based offset
            })
        })
        .collect()
}

fn top_of_stacks(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn part1(mut state: Stacks, moves: &Moves) -> String {
    for m in moves {
        for _ in 0..m.count {
            assert!(state[m.from].len() > 0);
            let cr = state[m.from].pop().unwrap();
            state[m.to].push(cr);
        }
    }

    top_of_stacks(&state)
}

fn part2(mut state: Stacks, moves: &Moves) -> String {
    for m in moves {
        assert!(state[m.from].len() >= m.count);

        let split_from = state[m.from].len() - m.count;
        let cr = state[m.from].split_off(split_from);
        state[m.to].extend(cr);
    }

    top_of_stacks(&state)
}

fn main() {
    let inp = fs::read_to_string("inputs/5.txt").unwrap();

    // Split input into initial state and moves
    let mut itinp = inp.lines();
    let inp_initial: Vec<&str> = itinp.by_ref().take_while(|s| s.trim().len() != 0).collect();
    let inp_moves: Vec<&str> = itinp.collect();

    let stacks = parse_initial(&inp_initial);
    let moves = parse_moves(&inp_moves);

    println!("Answer #1: {}", part1(stacks.to_vec(), &moves));
    println!("Answer #2: {}", part2(stacks.to_vec(), &moves));
}
