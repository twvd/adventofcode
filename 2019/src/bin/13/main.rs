use aoc_2019::intcode::{IntComputer, Word};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::{fmt, fs, thread, time};

#[derive(Eq, PartialEq, Copy, Clone, FromPrimitive)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl Tile {
    fn to_str(&self) -> &'static str {
        match self {
            Tile::Empty => " ",
            Tile::Ball => "o",
            Tile::Wall => "#",
            Tile::Block => "@",
            Tile::Paddle => "=",
        }
    }
}

type Point = (Word, Word);

struct Screen {
    content: HashMap<Point, Tile>,
    score: Word,
}

impl Screen {
    fn new() -> Self {
        Self {
            content: HashMap::new(),
            score: 0,
        }
    }

    fn set_raw(&mut self, raw: &[Word]) {
        assert_eq!(raw.len(), 3);
        if (raw[0], raw[1]) == (-1, 0) {
            self.score = raw[2];
        } else {
            self.content
                .insert((raw[0], raw[1]), Tile::from_u32(raw[2] as u32).unwrap());
        }
    }

    fn update(&mut self, data: &[Word]) {
        for u in data.chunks(3) {
            self.set_raw(u);
        }
    }

    fn tiles_iter(&self) -> impl Iterator<Item = &Tile> {
        self.content.values()
    }

    fn tile_pos(&self, tile: Tile) -> Option<Point> {
        self.content
            .iter()
            .filter(|(_, &t)| t == tile)
            .map(|(&p, _)| p)
            .next()
    }

    fn range_x(&self) -> impl Iterator<Item = Word> {
        self.content.keys().min_by_key(|(x, _)| x).unwrap().0
            ..=self.content.keys().max_by_key(|(x, _)| x).unwrap().0
    }

    fn range_y(&self) -> impl Iterator<Item = Word> {
        self.content.keys().min_by_key(|(_, y)| y).unwrap().1
            ..=self.content.keys().max_by_key(|(_, y)| y).unwrap().1
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s += format!("Score: {}\n", self.score).as_str();
        for y in self.range_y() {
            for x in self.range_x() {
                s += self.content.get(&(x, y)).unwrap_or(&Tile::Empty).to_str();
            }
            s += "\n";
        }

        write!(f, "{}", s)
    }
}

fn part1(prog: &str) -> usize {
    let output = IntComputer::from_str(&prog)
        .unwrap()
        .run()
        .unwrap()
        .output();
    let mut screen: Screen = Screen::new();
    screen.update(&output);

    screen.tiles_iter().filter(|&&t| t == Tile::Block).count()
}

fn part2(prog: &str, fast: bool) -> Word {
    let mut comp = IntComputer::from_str(&prog).unwrap();

    let mut screen = Screen::new();

    comp.write_mem(0, 2);

    while !comp.is_halted() {
        comp.run_to_input().unwrap();
        screen.update(comp.output().as_slice());

        let ball = screen.tile_pos(Tile::Ball).unwrap();
        let paddle = screen.tile_pos(Tile::Paddle).unwrap();

        comp.input(
            // Hugely advanced AI below:
            if ball.0 > paddle.0 {
                &[1]
            } else if ball.0 < paddle.0 {
                &[-1]
            } else {
                &[0]
            },
        );

        if !fast {
            // Clear terminal
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", &screen);
            thread::sleep(time::Duration::from_millis(50));
        }
    }

    screen.score
}

fn main() {
    let inp = fs::read_to_string("inputs/13.txt").unwrap();

    let answer1 = part1(&inp);
    let answer2 = part2(&inp, false);
    println!("Answer #1: {}", answer1);
    println!("Answer #2: {}", answer2);
}
