use aoc_2019::stuff::{Direction, Point};
use pathfinding::prelude::bfs;
use std::collections::HashMap;
use std::fs;

type Pt = Point<i32>;
type Pt3D = (Pt, usize);
type Map = HashMap<Pt, Tile>;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Portal {
    Inner,
    Outer,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Tile {
    Invalid,
    Wall,
    Open,
    Placeholder(char),
    Portal(([char; 2], Portal)),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Invalid
    }
}

impl Default for &Tile {
    fn default() -> Self {
        &Tile::Invalid
    }
}

fn is_portal_dest(p: &Tile, dest: &[char; 2]) -> bool {
    if let Tile::Portal((d, _)) = p {
        if dest == d {
            return true;
        }
    }

    false
}

fn parse(inp: &str) -> Option<Map> {
    let mut map = Map::new();
    for (y, l) in inp.lines().enumerate() {
        for (x, c) in l.char_indices() {
            map.insert(
                Point(x.try_into().ok()?, y.try_into().ok()?),
                match c {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    'A'..='Z' => Tile::Placeholder(c),
                    ' ' => continue,
                    _ => unreachable!(),
                },
            );
        }
    }

    // Populate the portals
    let width = map.keys().max_by_key(|p| p.0)?.0;
    let height = map.keys().max_by_key(|p| p.1)?.1;
    let outer_l = width / 4;
    let outer_r = width / 4 * 3;
    let outer_t = height / 4;
    let outer_b = height / 4 * 3;
    let portal_type = |p: Pt| {
        if p.0 < outer_l || p.0 > outer_r || p.1 < outer_t || p.1 > outer_b {
            Portal::Outer
        } else {
            Portal::Inner
        }
    };
    for (p, t) in map
        .clone()
        .into_iter()
        .filter(|&(_, t)| matches!(t, Tile::Placeholder(_)))
    {
        let Tile::Placeholder(c) = t else { unreachable!() };

        // Upwards          Downwards
        // _                .
        // A  <- p_n (c2)   A  <- p_n (c2)
        // B  <- p (c)      B  <- p (c)
        // .  <- p_s        _  <- p_s
        let p_n = p.move_dir(Direction::Up, 1);
        let p_e = p.move_dir(Direction::Right, 1);
        if let Tile::Placeholder(c2) = *map.get(&p_n).unwrap_or_default() {
            let p_s = p.move_dir(Direction::Down, 1);
            match *map.get(&p_s).unwrap_or_default() {
                Tile::Open => {
                    // Portal on bottom tile
                    map.insert(p, Tile::Portal(([c, c2], portal_type(p))));
                    map.remove(&p_n);
                }
                Tile::Invalid => {
                    // Portal on top tile
                    map.insert(p_n, Tile::Portal(([c, c2], portal_type(p))));
                    map.remove(&p);
                }
                Tile::Placeholder(_) => {}
                _ => unreachable!(),
            }
        }
        // Leftwards:  _   A   B   .
        //             p_w p   p_e
        // Rightwards: .   A   B   _
        //             p_w p   p_e
        if let Tile::Placeholder(c2) = *map.get(&p_e).unwrap_or_default() {
            let p_w = p.move_dir(Direction::Left, 1);
            match *map.get(&p_w).unwrap_or_default() {
                Tile::Invalid => {
                    // Portal on the right tile
                    map.insert(p_e, Tile::Portal(([c, c2], portal_type(p))));
                    map.remove(&p);
                }
                Tile::Open => {
                    // Portal on the left tile
                    map.insert(p, Tile::Portal(([c, c2], portal_type(p))));
                    map.remove(&p_e);
                }
                Tile::Placeholder(_) => {}
                _ => unreachable!(),
            }
        }
    }

    assert_eq!(
        map.values()
            .filter(|&&t| matches!(t, Tile::Placeholder(_)))
            .count(),
        0
    );

    Some(map)
}

fn part1(map: &Map) -> Option<usize> {
    let (&entrance, _) = map
        .iter()
        .find(|&(_, c)| matches!(*c, Tile::Portal((['A', 'A'], _))))?;
    let (&end, _) = map
        .iter()
        .find(|&(_, c)| matches!(*c, Tile::Portal((['Z', 'Z'], _))))?;

    let path = bfs(
        &entrance,
        |&p| {
            p.adjacent_straight()
                .into_iter()
                .filter_map(|ap| {
                    if ap == end {
                        return Some(ap);
                    }
                    match *map.get(&ap).unwrap_or_default() {
                        Tile::Open => Some(ap),
                        Tile::Portal((c, _)) => Some(
                            *map.iter()
                                .find(|&(apm, t)| is_portal_dest(t, &c) && *apm != ap)?
                                .0,
                        ),
                        _ => None,
                    }
                })
                .collect::<Vec<Pt>>()
        },
        |&p| p == end,
    )?;

    // bfs() counts the starting position and we include 2
    // tiles for each portal traversal so subtract those.
    Some(
        path.len()
            - 1
            - path
                .iter()
                .filter(|&&p| matches!(map.get(&p).unwrap(), Tile::Portal(_)))
                .count(),
    )
}

fn part2(map: &Map) -> Option<usize> {
    fn traverse_portal(p: Pt3D, dest: Pt, t: Tile) -> Option<Pt3D> {
        assert!(matches!(t, Tile::Portal(_)));

        let Tile::Portal((_, ptype)) = t else { unreachable!(); };

        if p.1 == 0 && ptype == Portal::Outer {
            None
        } else if ptype == Portal::Outer {
            Some((dest, p.1 - 1))
        } else {
            // Inner
            Some((dest, p.1 + 1))
        }
    }

    let (&entrance, _) = map
        .iter()
        .find(|&(_, c)| matches!(*c, Tile::Portal((['A', 'A'], _))))?;
    let (&end, _) = map
        .iter()
        .find(|&(_, c)| matches!(*c, Tile::Portal((['Z', 'Z'], _))))?;

    let path = bfs(
        &(entrance, 0),
        |&(p, z)| {
            p.adjacent_straight()
                .into_iter()
                .filter_map(|ap| {
                    if ap == end {
                        return Some((ap, z));
                    }
                    let t = *map.get(&ap).unwrap_or_default();
                    match t {
                        Tile::Open => Some((ap, z)),
                        Tile::Portal((c, _)) => {
                            let dest = *map
                                .iter()
                                .find(|&(apm, t)| is_portal_dest(t, &c) && *apm != ap)?
                                .0;
                            traverse_portal((ap, z), dest, t)
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<Pt3D>>()
        },
        |&p| p == (end, 0),
    )?;

    // bfs() counts the starting position and we include 2
    // tiles for each portal traversal so subtract those.
    Some(
        path.len()
            - 1
            - path
                .iter()
                .filter(|&&(p, _)| matches!(map.get(&p).unwrap(), Tile::Portal(_)))
                .count(),
    )
}

fn main() {
    let inp = fs::read_to_string("inputs/20.txt").unwrap();
    let map = parse(&inp).unwrap();

    println!("Answer #1: {}", part1(&map).unwrap());
    println!("Answer #2: {}", part2(&map).unwrap());
}
