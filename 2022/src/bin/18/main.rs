use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn from_tuple((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }

    fn adjacent_faces(self: &Self) -> [Self; 6] {
        [
            Self {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

type Cubes = HashSet<Cube>;

fn parse(inp: &str) -> Cubes {
    Cubes::from_iter(inp.lines().map(|l| {
        Cube::from_tuple(
            l.trim()
                .splitn(3, ",")
                .map(|n| n.parse::<i32>().unwrap())
                .tuples()
                .next()
                .unwrap(),
        )
    }))
}

fn part1(cubes: &Cubes) -> usize {
    cubes
        .iter()
        .flat_map(|c| c.adjacent_faces().into_iter())
        .filter(|c| !cubes.contains(c))
        .count()
}

fn part2(cubes: &Cubes) -> usize {
    let mut outside = Cubes::new();
    let rx = (cubes.iter().min_by_key(|c| c.x).unwrap().x - 1)
        ..=(cubes.iter().max_by_key(|c| c.x).unwrap().x + 1);
    let ry = (cubes.iter().min_by_key(|c| c.y).unwrap().y - 1)
        ..=(cubes.iter().max_by_key(|c| c.y).unwrap().y + 1);
    let rz = (cubes.iter().min_by_key(|c| c.z).unwrap().z - 1)
        ..=(cubes.iter().max_by_key(|c| c.z).unwrap().z + 1);
    let mut work = vec![Cube {
        x: *rx.start(),
        y: *ry.start(),
        z: *rz.start(),
    }];

    while let Some(p) = work.pop() {
        work.extend(p.adjacent_faces().into_iter().filter(|c| {
            rx.contains(&c.x)
                && ry.contains(&c.y)
                && rz.contains(&c.z)
                && !outside.contains(c)
                && !cubes.contains(c)
        }));
        outside.insert(p);
    }

    cubes
        .iter()
        .flat_map(|c| c.adjacent_faces().into_iter())
        .filter(|c| outside.contains(c))
        .count()
}

fn main() {
    let inp = fs::read_to_string("inputs/18.txt").unwrap();

    let cubes = parse(&inp);

    println!("{} cubes", cubes.len());

    println!("Answer #1: {}", part1(&cubes));
    println!("Answer #2: {}", part2(&cubes));
}
