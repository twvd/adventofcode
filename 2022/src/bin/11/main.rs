use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs;

type MItem = i128;
type Monkies = Vec<Monkey>;

struct Monkey {
    items: VecDeque<MItem>,
    test_mod: MItem,
    test_true: usize,
    test_false: usize,
    op: fn(&Self, MItem) -> MItem,
    val: Option<MItem>,
    inspects: usize,
}

impl Monkey {
    fn op_old_add_old(self: &Self, old: MItem) -> MItem {
        old + old
    }

    fn op_old_mul_old(self: &Self, old: MItem) -> MItem {
        old * old
    }

    fn op_old_add(self: &Self, old: MItem) -> MItem {
        old + self.val.unwrap()
    }

    fn op_old_mul(self: &Self, old: MItem) -> MItem {
        old * self.val.unwrap()
    }

    fn parse_all(inp: &str) -> Monkies {
        lazy_static! {
            static ref MONKIES: Regex = Regex::new(
                r"Monkey (\d+):
  Starting items: ([0-9, ]+)
  Operation: new = (old|\d+) (\*|\+) (old|\d+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"
            )
            .unwrap();
        }

        let matches = MONKIES.captures_iter(&inp);
        let mut ret: Monkies = vec![];

        for cap in matches {
            assert_eq!(ret.len(), cap[1].parse().unwrap());

            let op = match (&cap[3], &cap[4], &cap[5]) {
                ("old", "*", "old") => Monkey::op_old_mul_old,
                ("old", "+", "old") => Monkey::op_old_add_old,
                ("old", "*", _) => Monkey::op_old_mul,
                ("old", "+", _) => Monkey::op_old_add,
                _ => panic!("{} {} {}", &cap[3], &cap[4], &cap[5]),
            };

            ret.push(Monkey {
                items: cap[2]
                    .split(',')
                    .map(|i| i.trim().parse::<MItem>().unwrap())
                    .collect(),
                test_mod: cap[6].parse().unwrap(),
                test_true: cap[7].parse::<usize>().unwrap(),
                test_false: cap[8].parse::<usize>().unwrap(),
                op,
                val: cap[5].parse::<MItem>().ok(),
                inspects: 0,
            });
        }

        ret
    }

    fn test(self: &Self, i: MItem) -> bool {
        i % self.test_mod == 0
    }
}

fn monkey_round(monkies: &mut Monkies, idx: usize, relief: bool, monkey_lcm: MItem) {
    let items: Vec<MItem> = monkies[idx].items.drain(..).collect();

    for i in items {
        let ttrue = monkies[idx].test_true;
        let tfalse = monkies[idx].test_false;
        let mut j = (monkies[idx].op)(&monkies[idx], i);
        if relief {
            j /= 3;
        }
        j %= monkey_lcm;
        if monkies[idx].test(j) {
            monkies[ttrue].items.push_back(j);
        } else {
            monkies[tfalse].items.push_back(j);
        }
        monkies[idx].inspects += 1;
    }
}

fn run_rounds(inp: &str, rounds: usize, relief: bool) -> usize {
    let mut monkies = Monkey::parse_all(&inp);
    let monkeycount = monkies.len();

    // Optimize using the Least Common Divisor of all the tests. We can
    // cap the worry levels on that.
    let mut monkey_lcm: MItem = 1;
    for m in &monkies {
        monkey_lcm *= m.test_mod;
    }

    for _ in 0..rounds {
        for idx in 0..monkeycount {
            monkey_round(&mut monkies, idx, relief, monkey_lcm);
        }
    }

    // Calculate monkey business
    monkies.sort_by(|a, b| a.inspects.cmp(&b.inspects));
    monkies.reverse();
    return monkies[0].inspects * monkies[1].inspects;
}

fn main() {
    let inp = fs::read_to_string("inputs/11.txt").unwrap();

    println!("Answer #1: {}", run_rounds(&inp, 20, true));
    println!("Answer #2: {}", run_rounds(&inp, 10000, false));
}
