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
type Segment = i32;

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

    fn coord_to_segment(self: &Self, coord: Coord) -> Segment {
        // This assumes a fixed layout in the input
        match coord {
            (50..=99, 0..=49) => 0,
            (100..=149, 0..=49) => 1,
            (50..=99, 50..=99) => 2,
            (0..=49, 100..=149) => 3,
            (50..=99, 100..=149) => 4,
            (0..=49, 150..=199) => 5,
            _ => unreachable!(),
        }
    }

    fn transition_segment(
        self: &Self,
        p: Coord,
        d: Direction,
        oldseg: Segment,
    ) -> (Coord, Direction) {
        // This assumes a fixed layout in the input
        match (oldseg, d) {
            (0, Direction::Up) => ((0, 100 + p.0), Direction::Right),
            (0, Direction::Left) => ((0, 149 - p.1), Direction::Right),
            (1, Direction::Up) => ((p.0 - 100, 199), d),
            (1, Direction::Right) => ((99, 149 - p.1), Direction::Left),
            (1, Direction::Down) => ((99, p.0 - 50), Direction::Left),
            (2, Direction::Right) => ((p.1 + 50, 49), Direction::Up),
            (2, Direction::Left) => ((p.1 - 50, 100), Direction::Down),
            (3, Direction::Up) => ((50, p.0 + 50), Direction::Right),
            (3, Direction::Left) => ((50, 149 - p.1), Direction::Right),
            (4, Direction::Right) => ((149, 149 - p.1), Direction::Left),
            (4, Direction::Down) => ((49, 100 + p.0), Direction::Left),
            (5, Direction::Right) => ((p.1 - 100, 149), Direction::Up),
            (5, Direction::Down) => ((p.0 + 100, 0), d),
            (5, Direction::Left) => ((p.1 - 100, 0), Direction::Down),
            _ => unreachable!(),
        }
    }

    fn next_coord(self: &Self, coord: Coord, dir: Direction, part2: bool) -> (Coord, Direction) {
        let rel = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let step = ((coord.0 + rel.0), (coord.1 + rel.1));

        match self.get(step) {
            MapTile::Wall | MapTile::Floor => (step, dir),
            MapTile::Void => {
                if !part2 {
                    let mut p = coord;
                    while self.get(p) != MapTile::Void {
                        p = (p.0 - rel.0, p.1 - rel.1);
                    }
                    ((p.0 + rel.0, p.1 + rel.1), dir)
                } else {
                    self.transition_segment(step, dir, self.coord_to_segment(coord))
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
                    let (newpos, newdir) = map.next_coord(pos, dir, part2);
                    if map.get(newpos) == MapTile::Wall {
                        break;
                    }
                    pos = newpos;
                    dir = newdir;
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
