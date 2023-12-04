use std::collections::VecDeque;
use std::fs;

struct Card {
    id: usize,
    winning: Vec<usize>,
    nums: Vec<usize>,
}

impl Card {
    fn parse(s: &str) -> Self {
        Self {
            id: s[5..8].trim().parse::<usize>().unwrap(),
            winning: Vec::from_iter(
                s[9..s.find('|').unwrap()]
                    .split(' ')
                    .filter(|&i| i.len() > 0)
                    .map(|i| i.parse::<usize>().unwrap()),
            ),
            nums: Vec::from_iter(
                s[(s.find('|').unwrap() + 1)..]
                    .split(' ')
                    .filter(|&i| i.len() > 0)
                    .map(|i| i.parse::<usize>().unwrap()),
            ),
        }
    }

    fn winning(&self) -> usize {
        self.nums
            .iter()
            .filter(|&n| self.winning.contains(n))
            .count()
    }

    fn won_cards(&self) -> impl Iterator<Item = usize> {
        (self.id + 1)..(self.id + 1 + self.winning())
    }

    fn points(&self) -> usize {
        let winning = self.winning();
        if winning > 0 {
            2_usize.pow(winning.saturating_sub(1) as u32)
        } else {
            winning
        }
    }
}

fn part1(cards: &Vec<Card>) -> usize {
    cards.iter().map(|card| card.points()).sum::<usize>()
}

fn part2(cards: &Vec<Card>) -> usize {
    let mut queue = cards.iter().collect::<VecDeque<&Card>>();
    let mut answer = queue.len();
    while let Some(card) = queue.pop_front() {
        queue.extend(card.won_cards().map(|i| &cards[i - 1]));
        answer += card.won_cards().count();
    }
    answer
}

fn main() {
    let f = fs::read_to_string("inputs/4.txt").unwrap();

    let cards = f
        .trim()
        .lines()
        .map(|l| Card::parse(l.trim()))
        .collect::<Vec<_>>();

    println!("Answer #1: {}", part1(&cards));
    println!("Answer #2: {}", part2(&cards));
}
