use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;

use nom::{
    branch::alt, character::complete::char, character::complete::u32, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

#[derive(Clone, Eq, PartialEq)]
enum PacketField {
    Num(u32),
    Sub(Vec<PacketField>),
}

impl Ord for PacketField {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketField::Sub(s), PacketField::Sub(o)) => s.cmp(o),
            (PacketField::Num(s), PacketField::Num(o)) => s.cmp(o),
            (PacketField::Sub(s), PacketField::Num(o)) => s.cmp(&vec![PacketField::Num(*o)]),
            (PacketField::Num(s), PacketField::Sub(o)) => vec![PacketField::Num(*s)].cmp(o),
        }
    }
}

impl PartialOrd for PacketField {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn nomnom_packet(input: &str) -> IResult<&str, PacketField> {
    alt((
        map(u32, PacketField::Num),
        map(
            delimited(
                char('['),
                separated_list0(char(','), nomnom_packet),
                char(']'),
            ),
            PacketField::Sub,
        ),
    ))(input)
}

fn part1(packets: &Vec<PacketField>) -> usize {
    packets
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .fold(0, |acc, (i, _)| acc + (i + 1))
}

fn part2(origpackets: &Vec<PacketField>) -> usize {
    let mut packets = origpackets.to_owned();
    let dividers = [
        nomnom_packet("[[2]]").unwrap().1,
        nomnom_packet("[[6]]").unwrap().1,
    ];
    packets.extend_from_slice(&dividers);
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .fold(1, |acc, (i, _)| acc * (i + 1))
}

fn main() {
    let inp = fs::read_to_string("inputs/13.txt").unwrap();
    let packets: Vec<PacketField> = inp
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|p| nomnom_packet(p).unwrap().1)
        .collect();

    println!("Answer #1: {}", part1(&packets));
    println!("Answer #2: {}", part2(&packets));
}
