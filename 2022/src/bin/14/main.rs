use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

type Point = (i32, i32);
type LineSegment = (Point, Point);
type Grid = HashMap<Point, GridVal>;

#[derive(Clone, Eq, PartialEq)]
enum GridVal {
    Rock,
    Sand,
}

const DROP_POINT: Point = (500, 0);

fn parse_to_lineseg(line: &str) -> Vec<LineSegment> {
    let lb = line
        .trim()
        .split(" -> ")
        .map(|a| {
            a.split(",")
                .map(|i| i.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<Point>>();

    zip(lb.to_owned().into_iter(), lb[1..].to_owned().into_iter()).collect::<Vec<LineSegment>>()
}

fn draw_line(grid: &mut Grid, start: Point, end: Point) {
    if start.0 == end.0 {
        // Vertical
        for y in min(start.1, end.1)..=max(start.1, end.1) {
            grid.insert((start.0, y), GridVal::Rock);
        }
    } else {
        assert!(start.1 == end.1);

        // Horizontal
        for x in min(start.0, end.0)..=max(start.0, end.0) {
            grid.insert((x, start.1), GridVal::Rock);
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let xs = grid.keys().map(|(x, _)| x);
    let ys = grid.keys().map(|(_, y)| y);
    let minx: i32 = *xs.clone().min().unwrap();
    let maxx: i32 = *xs.max().unwrap();
    let miny: i32 = *ys.clone().min().unwrap();
    let maxy: i32 = *ys.max().unwrap();

    for y in miny..=maxy {
        let mut s = String::new();
        for x in minx..=maxx {
            s += match grid.get(&(x, y)) {
                Some(GridVal::Rock) => "#",
                Some(GridVal::Sand) => "o",
                _ => " ",
            };
        }
        println!("{}", s);
    }
}

fn calc_sand_paths(pt: &Point) -> [Point; 3] {
    [
        (pt.0, pt.1 + 1),     // down straight
        (pt.0 - 1, pt.1 + 1), // down left
        (pt.0 + 1, pt.1 + 1), // down right
    ]
}

fn part1(grid: &Grid) -> usize {
    fn drop_sand(grid: &mut Grid, point: &Point, abyss_y: i32) -> bool {
        let mut pt = *point;

        assert!(grid.get(&pt).is_none());

        'drop: loop {
            if pt.1 > abyss_y {
                return false;
            }

            for path in calc_sand_paths(&pt) {
                if grid.get(&path).is_none() {
                    pt = path;
                    continue 'drop;
                }
            }

            grid.insert(pt, GridVal::Sand);
            return true;
        }
    }

    let mut mygrid: Grid = grid.clone();
    let abyss_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    while drop_sand(&mut mygrid, &DROP_POINT, abyss_y) {}

    mygrid.values().filter(|&t| *t == GridVal::Sand).count()
}

fn part2(grid: &Grid) -> usize {
    fn drop_sand(grid: &mut Grid, point: &Point, floor_y: i32) -> bool {
        let mut pt = *point;

        assert!(grid.get(&pt).is_none());

        'drop: loop {
            for path in calc_sand_paths(&pt) {
                if grid.get(&path).is_none() && pt.1 < (floor_y - 1) {
                    pt = path;
                    continue 'drop;
                }
            }

            grid.insert(pt, GridVal::Sand);
            return true;
        }
    }

    let mut mygrid: Grid = grid.clone();
    let floor_y = *grid.keys().map(|(_, y)| y).max().unwrap() + 2;

    while mygrid.get(&DROP_POINT) != Some(&GridVal::Sand) {
        drop_sand(&mut mygrid, &DROP_POINT, floor_y);
    }

    mygrid.values().filter(|&t| *t == GridVal::Sand).count()
}

fn main() {
    let inp = fs::read_to_string("inputs/14.txt").unwrap();

    let mut linesegs: Vec<LineSegment> = vec![];
    for l in inp.lines() {
        linesegs.append(&mut parse_to_lineseg(l));
    }

    let mut grid: Grid = HashMap::new();

    for (start, end) in &linesegs {
        draw_line(&mut grid, *start, *end);
    }

    println!("Answer #1: {}", part1(&grid));
    println!("Answer #2: {}", part2(&grid));
}
