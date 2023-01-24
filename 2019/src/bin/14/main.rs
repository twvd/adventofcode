#![feature(int_roundings)]

use std::collections::{HashMap, VecDeque};
use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i128, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Clone, Debug)]
struct Component<'a> {
    amount: i128,
    name: &'a str,
}

#[derive(Debug)]
struct Recipe<'a> {
    input: Vec<Component<'a>>,
    output: Component<'a>,
}

fn parse(inp: &str) -> Vec<Recipe> {
    fn component(input: &str) -> IResult<&str, Component> {
        let (i, (amount, name)) = separated_pair(i128, space1, alpha1)(input)?;

        Ok((i, Component { amount, name }))
    }

    fn components(input: &str) -> IResult<&str, Vec<Component>> {
        separated_list1(tag(", "), component)(input)
    }

    fn recipe(input: &str) -> IResult<&str, Recipe> {
        let (i, (input, output)) = separated_pair(components, tag(" => "), component)(input)?;

        Ok((i, Recipe { input, output }))
    }

    inp.trim()
        .lines()
        .map(|l| recipe(l.trim()).unwrap().1)
        .collect()
}

fn ore_required(recipes: &[Recipe], fuel_amt: i128) -> i128 {
    let mut work: VecDeque<Component> = VecDeque::from([Component {
        name: "FUEL",
        amount: fuel_amt,
    }]);

    let mut ore_required: i128 = 0;
    let mut supply: HashMap<&str, i128> = HashMap::new();

    while let Some(w) = work.pop_front() {
        // End of the chain
        if w.name == "ORE" {
            ore_required += w.amount;
            continue;
        }

        // Check supply store
        let in_supply = *supply.get(w.name).unwrap_or(&0);
        if in_supply >= w.amount {
            *supply.entry(w.name).or_insert(0) -= w.amount;
            continue;
        }

        // We need to create it
        let recipe = recipes
            .iter()
            .filter(|&r| r.output.name == w.name)
            .next()
            .unwrap();
        let amt_make = w.amount - in_supply;
        let batches = amt_make.div_ceil(recipe.output.amount);
        for i in &recipe.input {
            work.push_front(Component {
                name: i.name,
                amount: i.amount * batches,
            });
        }
        *supply.entry(w.name).or_insert(0) = batches * recipe.output.amount - amt_make;
    }

    ore_required
}

fn part1(recipes: &[Recipe]) -> i128 {
    ore_required(recipes, 1)
}

fn part2(recipes: &[Recipe]) -> i128 {
    let target = 1000000000000i128;
    let mut search_low = 0i128;
    let mut search_high = target;

    while search_low <= search_high {
        let search_mid = (search_high + search_low) / 2;
        let res = ore_required(recipes, search_mid);

        if res == target {
            return search_mid;
        }

        if res < target {
            search_low = search_mid + 1;
        }
        if res > target {
            search_high = search_mid - 1;
        }
    }

    search_low - 1
}

fn main() {
    let inp = fs::read_to_string("inputs/14.txt").unwrap();

    let recipes = parse(&inp);

    println!("Answer #1: {}", part1(&recipes));
    println!("Answer #2: {}", part2(&recipes));
}
