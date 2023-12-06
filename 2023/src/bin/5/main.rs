use std::cmp::min;
use std::fs;
use std::iter;
use std::ops::Range;
use std::thread;

use itertools::Itertools;
use num::PrimInt;

#[derive(Debug, Clone)]
struct Map {
    src: Range<u64>,
    dest: Range<u64>,
}

type Maps = Vec<Map>;
type MapDims = Vec<Maps>;
type Seeds = Vec<u64>;

fn map_range<T: PrimInt>(from_range: &Range<T>, to_range: &Range<T>, s: T) -> T {
    to_range.start
        + (s - from_range.start) * (to_range.end - to_range.start)
            / (from_range.end - from_range.start)
}

fn crunch(seed: u64, mapdims: &MapDims) -> u64 {
    let mut v = seed;
    for maps in mapdims.iter() {
        for m in maps.into_iter() {
            if m.src.contains(&v) {
                v = map_range(&m.src, &m.dest, v);
                break;
            }
        }
    }
    v
}

fn part1(seeds: &Seeds, mapdims: &MapDims) -> u64 {
    seeds
        .into_iter()
        .map(|s| crunch(*s, mapdims))
        .min()
        .unwrap()
}

fn part2(seeds: &Seeds, mapdims: &MapDims) -> u64 {
    // Not the smartest way to do it, but:
    // real    1m31.341s
    // user    5m49.499s
    // sys     0m0.723s

    let ranges = seeds
        .chunks(2)
        .map(|a| a[0]..(a[0] + a[1]))
        .collect::<Vec<_>>();

    let (snd, rcv) = crossbeam_channel::unbounded();
    for t_r in ranges.iter().cloned() {
        let t_mapdims = mapdims.clone();
        let t_snd = snd.clone();

        thread::spawn(move || {
            t_snd
                .send(t_r.fold(u64::MAX, |a, seed| min(a, crunch(seed, &t_mapdims))))
                .unwrap();
        });
    }

    iter::from_fn(|| rcv.recv().ok())
        .take(ranges.len())
        .min()
        .unwrap()
}

fn main() {
    let f = fs::read_to_string("inputs/5.txt").unwrap();

    let seeds = Seeds::from_iter(
        f.lines().next().unwrap()[("seeds: ".len())..]
            .split(' ')
            .map(|i| i.parse().unwrap()),
    );

    let mut mapdims = MapDims::new();
    for (_, g) in f
        .lines()
        .skip(2)
        .group_by(|i| i.len() == 0)
        .into_iter()
        .step_by(2)
    {
        let mut maps = Maps::new();
        for a in g.into_iter().skip(1).map(|l| {
            l.trim()
                .split(' ')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u64>>()
        }) {
            let (src, dest, len) = (a[1], a[0], a[2]);
            maps.push(Map {
                src: (src..(src + len)),
                dest: (dest..(dest + len)),
            });
        }
        mapdims.push(maps);
    }

    println!("Answer #1: {}", part1(&seeds, &mapdims));
    println!("Answer #2: {}", part2(&seeds, &mapdims));
}
