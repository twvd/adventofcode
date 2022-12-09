use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn rel(self: &Self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn relxy(self: &Self, rx: i32, ry: i32) -> Self {
        Self {
            x: self.x + rx,
            y: self.y + ry,
        }
    }
}

type Move = (char, i32);

fn follow(leader: &Point, follower: &Point) -> Point {
    let dx = leader.x - follower.x;
    let dy = leader.y - follower.y;

    if dx.abs() >= 2 || dy.abs() >= 2 {
        follower.relxy(dx.signum(), dy.signum())
    } else {
        // Still close, no change
        follower.clone()
    }
}

fn translate_move(c: char) -> Result<Point, ()> {
    match c {
        'U' => Ok(Point { x: 0, y: -1 }),
        'D' => Ok(Point { x: 0, y: 1 }),
        'L' => Ok(Point { x: -1, y: 0 }),
        'R' => Ok(Point { x: 1, y: 0 }),
        _ => Err(()),
    }
}

fn run(moves: &Vec<Move>, num_knots: usize) -> usize {
    let mut knots: Vec<Point> = vec![Point::new(); num_knots];
    let mut visits = HashSet::new();

    for m in moves {
        let rel_move = translate_move(m.0).unwrap();
        for _ in 0..m.1 {
            knots[0] = knots[0].rel(&rel_move);
            for k in 1..num_knots {
                knots[k] = follow(&knots[k - 1], &knots[k]);
            }
            visits.insert(knots[num_knots - 1]);
        }
    }

    visits.len()
}

fn main() {
    let inp = fs::read_to_string("inputs/9.txt").unwrap();
    let moves: Vec<Move> = inp
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.trim().split(' ').collect();
            (s[0].chars().nth(0).unwrap(), s[1].parse::<i32>().unwrap())
        })
        .collect();

    println!("Answer #1: {}", run(&moves, 2));
    println!("Answer #2: {}", run(&moves, 10));
}
