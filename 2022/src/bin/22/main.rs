use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32, combinator::map,
    combinator::value, error::Error, multi::many0,
};
use std::collections::HashMap;
use std::fs;
use std::ops::Range;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Move(i32),
    TurnLeft,
    TurnRight,
}
type Instructions = Vec<Instruction>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

impl Direction {
    fn turn_left(self: &Self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    fn turn_right(self: &Self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

type Coord = (i32, i32);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapTile {
    Void,
    Floor,
    Wall,
}

struct Map {
    map: HashMap<Coord, MapTile>,
}

impl Map {
    fn parse(inp: &str) -> Map {
        let mut map = HashMap::new();
        for (y, l) in inp.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                map.insert(
                    (x as i32, y as i32),
                    match c {
                        ' ' => MapTile::Void,
                        '#' => MapTile::Wall,
                        '.' => MapTile::Floor,
                        _ => panic!("Unknown input"),
                    },
                );
            }
        }
        Map { map }
    }

    fn get(self: &Self, coord: Coord) -> MapTile {
        *self.map.get(&coord).unwrap_or(&MapTile::Void)
    }

    fn range_x(self: &Self) -> Range<i32> {
        0..(*self.map.keys().map(|(x, _)| x).max().unwrap_or(&0))
    }

    fn range_y(self: &Self) -> Range<i32> {
        0..(*self.map.keys().map(|(_, y)| y).max().unwrap_or(&0))
    }

    fn find_start(self: &Self) -> Option<Coord> {
        for y in self.range_y() {
            for x in self.range_x() {
                if self.map[&(x, y)] == MapTile::Floor {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn next_coord(self: &Self, coord: Coord, dir: Direction, part2: bool) -> Coord {
        let rel = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let step = ((coord.0 + rel.0), (coord.1 + rel.1));

        match self.get(step) {
            MapTile::Wall | MapTile::Floor => step,
            MapTile::Void => {
                if !part2 {
                    let mut p = coord;
                    while self.get(p) != MapTile::Void {
                        p = (p.0 - rel.0, p.1 - rel.1);
                    }
                    (p.0 + rel.0, p.1 + rel.1)
                } else {
                    panic!("TODO");
                }
            }
        }
    }
}

fn parse_instructions(inp: &str) -> Instructions {
    many0::<_, _, Error<_>, _>(alt((
        map(i32, Instruction::Move),
        value(Instruction::TurnLeft, tag("L")),
        value(Instruction::TurnRight, tag("R")),
    )))(inp)
    .unwrap()
    .1
}

fn parse(inp: &str) -> (Map, Instructions) {
    (
        Map::parse(&inp.lines().take_while(|l| l.trim().len() > 0).join("\n")),
        parse_instructions(inp.lines().last().unwrap()),
    )
}

fn solve(map: &Map, instr: &Instructions, part2: bool) -> i32 {
    let mut pos = map.find_start().unwrap();
    let mut dir = Direction::Right;

    for i in instr {
        assert_eq!(map.get(pos), MapTile::Floor);
        match i {
            Instruction::TurnLeft => dir = dir.turn_left(),
            Instruction::TurnRight => dir = dir.turn_right(),
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    let newpos = map.next_coord(pos, dir, part2);
                    if map.get(newpos) == MapTile::Wall {
                        break;
                    }
                    pos = newpos;
                }
            }
        }
    }

    // + 1 because we use zero-based coordinates
    (1000 * (pos.1 + 1)) + (4 * (pos.0 + 1)) + dir as i32
}

fn main() {
    let inp = fs::read_to_string("inputs/22.txt").unwrap();

    let (map, instr) = parse(&inp);

    println!("Answer #1: {}", solve(&map, &instr, false));
    println!("Answer #2: {}", solve(&map, &instr, true));
}
