use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Monkey<'a, T> {
    operation: Option<fn(T, T) -> T>,
    op_left: Option<&'a str>,
    op_right: Option<&'a str>,
    solution: Option<T>,
}

impl<
        'a,
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    > Monkey<'a, T>
{
    fn new_number(num: T) -> Monkey<'a, T> {
        Monkey {
            operation: None,
            op_left: None,
            op_right: None,
            solution: Some(num),
        }
    }

    fn new_op(opc: char, left: &'a str, right: &'a str) -> Monkey<'a, T> {
        Monkey {
            operation: match opc {
                '+' => Some(|a, b| a + b),
                '-' => Some(|a, b| a - b),
                '/' => Some(|a, b| a / b),
                '*' => Some(|a, b| a * b),
                _ => panic!("Unknown input"),
            },
            op_left: Some(left),
            op_right: Some(right),
            solution: None,
        }
    }
}

type Monkies<'a> = HashMap<&'a str, Monkey<'a, i128>>;

fn parse(inp: &str) -> Monkies {
    let mut ret: Monkies = Monkies::new();

    for l in inp.lines() {
        let (name, action) = l.trim().split_once(": ").unwrap();

        ret.insert(
            name,
            match action.trim().parse::<i128>() {
                Ok(i) => Monkey::new_number(i),
                _ => {
                    let spl = action.splitn(3, " ").collect::<Vec<&str>>();
                    Monkey::new_op(spl[1].chars().next().unwrap(), spl[0], spl[2])
                }
            },
        );
    }

    ret
}

fn solve_monkies(monkies: &mut Monkies) {
    while monkies["root"].solution.is_none() {
        // Clone Monkies hashmap because we cannot borrow other
        // monkeys while we hold a mutable borrow of the hashmap or any value
        // in it.
        //
        // This means lots of cloning; there is probably a cleaner
        // way of doing this.
        let snapshot = monkies.clone();

        for monkey in monkies.values_mut() {
            if monkey.solution.is_none() {
                if let (Some(left), Some(right)) = (
                    snapshot[monkey.op_left.unwrap()].solution,
                    snapshot[monkey.op_right.unwrap()].solution,
                ) {
                    monkey.solution = Some((monkey.operation.unwrap())(left, right));
                }
            }
        }
    }
}

fn part1(monkies: &mut Monkies) -> i128 {
    solve_monkies(monkies);
    monkies["root"].solution.unwrap()
}

fn part2_try(input_monkies: &Monkies, val: i128) -> (i128, i128) {
    let mut monkies = input_monkies.clone();
    let name_left = input_monkies["root"].op_left.unwrap();
    let name_right = input_monkies["root"].op_right.unwrap();

    monkies.get_mut("humn").unwrap().solution = Some(val);
    solve_monkies(&mut monkies);

    (
        monkies[name_left].solution.unwrap(),
        monkies[name_right].solution.unwrap(),
    )
}

fn part2(input_monkies: &Monkies) -> Option<i128> {
    // I implemented this as binary search
    let mut search_low = 0i128;
    let mut search_high = i64::MAX as i128;

    while search_low <= search_high {
        let search_mid = (search_high + search_low) / 2;

        let (sol_left, sol_right) = part2_try(input_monkies, search_mid);

        if sol_left == sol_right {
            // Once found, search downwards to find the lowest number.
            // In my input, there are three values that solve the
            // equation, probably because of how the integer divisions
            // are rounded. The website only accepts the lowest;
            // in which the divisions would have produced no remainder.
            for i in 1.. {
                let (new_left, new_right) = part2_try(input_monkies, search_mid - i);
                if new_left != new_right {
                    return Some(search_mid - i + 1);
                }
            }
        }

        // In my input, sol_right is constant and sol_left changes depending
        // on monkey "humn".
        if sol_left > sol_right {
            search_low = search_mid + 1;
        }
        if sol_left < sol_right {
            search_high = search_mid - 1;
        }
    }

    None
}

fn main() {
    let inp = fs::read_to_string("inputs/21.txt").unwrap();

    let monkies = parse(&inp);

    println!("Answer #1: {}", part1(&mut monkies.clone()));
    println!("Answer #2: {}", part2(&mut monkies.clone()).unwrap());
}
