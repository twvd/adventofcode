use itertools::iproduct;
use std::fs;
use std::iter::repeat;

type Grid = Vec<Vec<u8>>; // [y][x]
type Point = (usize, usize);
const NUM_DIRS: usize = 4;

fn parse_grid(s: String) -> Grid {
    s.lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_lines(x: usize, y: usize, x_end: usize, y_end: usize) -> [Vec<Point>; NUM_DIRS] {
    // Returns sight lines as points moving from the starting point outwards
    // for every direction.
    [
        (0..x).rev().zip(repeat(y)).collect(),       // left
        repeat(x).zip((0..y).rev()).collect(),       // up
        (x + 1..x_end + 1).zip(repeat(y)).collect(), // right
        repeat(x).zip(y + 1..y_end + 1).collect(),   // down
    ]
}

fn is_visible(grid: &Grid, x: usize, y: usize) -> bool {
    let x_end = grid[0].len() - 1;
    let y_end = grid.len() - 1;
    if x > 0 && x < x_end && y > 0 && y < y_end {
        get_lines(x, y, x_end, y_end).iter().any(|line| {
            line.iter()
                .all(|(ix, iy)| grid[*iy][*ix] < grid[y][x])
        })
    } else {
        // Outside border
        true
    }
}

fn scenic_score(grid: &Grid, x: usize, y: usize) -> usize {
    let x_end = grid[0].len() - 1;
    let y_end = grid.len() - 1;
    let mut scores = 1;

    for line in get_lines(x, y, x_end, y_end) {
        let mut linescore: usize = 0;
        for (ix, iy) in line.iter() {
            linescore += 1;
            if grid[*iy][*ix] >= grid[y][x] {
                break;
            }
        }
        scores *= linescore;
    }
    scores
}

fn part1(grid: &Grid) -> usize {
    iproduct!(0..grid[0].len(), 0..grid.len())
        .filter(|(x, y)| is_visible(grid, *x, *y))
        .count()
}

fn part2(grid: &Grid) -> usize {
    iproduct!(0..grid[0].len(), 0..grid.len())
        .map(|(x, y)| scenic_score(grid, x, y))
        .max()
        .unwrap()
}

fn main() {
    let inp = fs::read_to_string("inputs/8.txt").unwrap();
    let grid = parse_grid(inp);

    println!("Answer #1: {}", part1(&grid));
    println!("Answer #2: {}", part2(&grid));
}
