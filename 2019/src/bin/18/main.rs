use aoc_2019::stuff::{Direction, Point};
use itertools::Itertools;
use pathfinding::prelude::{bfs, dijkstra};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use strum::IntoEnumIterator;

type Pt = Point<i32>;
type Map = HashMap<Pt, Tile>;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Tile {
    Entrance,
    Wall,
    Open,
    Door(char),
    Key(char),
}

#[derive(Debug)]
struct Path {
    len: usize,
    keys_needed: BTreeSet<char>,
    new_key: char,
}
type Paths = HashMap<(Pt, Pt), Path>;

fn parse(inp: &str) -> Option<Map> {
    let mut map = Map::new();
    for (y, l) in inp.trim().lines().enumerate() {
        for (x, c) in l.trim().char_indices() {
            map.insert(
                Point(x.try_into().ok()?, y.try_into().ok()?),
                match c {
                    '.' => Tile::Open,
                    '@' => Tile::Entrance,
                    '#' => Tile::Wall,
                    'a'..='z' => Tile::Key(c),
                    'A'..='Z' => Tile::Door(c),
                    _ => return None,
                },
            );
        }
    }

    Some(map)
}

fn gen_path_cache(map: &Map) -> Option<Paths> {
    let targets = map
        .iter()
        .filter_map(|(p, t)| match t {
            Tile::Key(_) | Tile::Entrance => Some((p, t)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut ret = Paths::new();

    for ((&start_p, &start_t), (&end_p, &end_t)) in targets.iter().tuple_combinations() {
        let path = match bfs(
            &start_p,
            |&p| {
                Direction::iter()
                    .map(|d| p.move_dir(d, 1))
                    .filter(|newp| map[newp] != Tile::Wall)
                    .collect::<Vec<_>>()
            },
            |&p| p == end_p,
        ) {
            Some(p) => p,
            None => continue,
        };

        let keys_needed = path
            .iter()
            .filter_map(|p| match map[p] {
                Tile::Door(d) => Some(d.to_lowercase().next()?),
                _ => None,
            })
            .collect::<BTreeSet<_>>();

        if let Tile::Key(new_key) = end_t {
            ret.insert(
                (start_p, end_p),
                Path {
                    len: path.len() - 1,
                    keys_needed: keys_needed.to_owned(),
                    new_key,
                },
            );
        }
        if let Tile::Key(new_key) = start_t {
            ret.insert(
                (end_p, start_p),
                Path {
                    len: path.len() - 1,
                    keys_needed: keys_needed.to_owned(),
                    new_key,
                },
            );
        }
    }

    Some(ret)
}

fn find_shortest_path(map: &Map, entrances: &[Pt]) -> Option<usize> {
    let keys: BTreeSet<char> = BTreeSet::from_iter(map.iter().filter_map(|(_, &c)| match c {
        Tile::Key(k) => Some(k),
        _ => None,
    }));
    let paths = gen_path_cache(map).unwrap();

    Some(
        dijkstra(
            &(entrances.to_vec(), BTreeSet::new()), // positions, keys collected
            |(ps, keys_collected)| {
                ps.iter()
                    .flat_map(|p| {
                        paths
                            .iter()
                            .filter(|&(pp, path)| {
                                pp.0 == *p
                                    && path.keys_needed.is_subset(keys_collected)
                                    && !keys_collected.contains(&path.new_key)
                            })
                            .map(|(pp, path)| {
                                let mut new_keys = keys_collected.clone();
                                new_keys.insert(path.new_key);
                                let mut new_pos = ps
                                    .iter()
                                    .filter(|&&psp| psp != *p)
                                    .copied()
                                    .collect::<Vec<Pt>>();
                                new_pos.push(pp.1);

                                ((new_pos, new_keys), path.len)
                            })
                    })
                    .collect::<Vec<_>>()
            },
            |(_, k)| *k == keys,
        )?
        .1,
    )
}

fn part1(map: &Map) -> Option<usize> {
    let entrance = *map.iter().find(|&(_, c)| *c == Tile::Entrance).unwrap().0;

    find_shortest_path(map, &[entrance])
}

fn part2(map: &Map) -> Option<usize> {
    let entrance = *map.iter().find(|&(_, c)| *c == Tile::Entrance).unwrap().0;

    let mut modded_map = map.clone();
    let modded_entrances = entrance.adjacent_diagonal();
    modded_map.insert(entrance, Tile::Wall);
    for p in entrance.adjacent_straight() {
        modded_map.insert(p, Tile::Wall);
    }
    for &p in &modded_entrances {
        modded_map.insert(p, Tile::Entrance);
    }

    find_shortest_path(&modded_map, &modded_entrances)
}

fn main() {
    let inp = fs::read_to_string("inputs/18.txt").unwrap();
    let map = parse(&inp).unwrap();

    println!("Answer #1: {}", part1(&map).unwrap());
    println!("Answer #2: {}", part2(&map).unwrap());
}
