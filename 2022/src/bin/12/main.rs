use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

type Grid = Vec<Vec<u8>>; // [y][x]
type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct PFWork {
    pt: Point,
    steps: usize,
}

fn parse_grid(s: String) -> Grid {
    s.lines().map(|l| l.trim().bytes().collect()).collect()
}

fn chars_to_points(haystack: &Grid, needle: u8) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];
    for (y, l) in haystack.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == needle {
                ret.push((x, y));
            }
        }
    }

    ret
}

fn find_elevation(grid: &Grid, point: &Point, elevation: u8) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];
    let rel = [
        (0, -1), // up
        (0, 1),  // down
        (-1, 0), // left
        (1, 0),  // right
    ];

    for r in rel {
        let trp: (i32, i32) = (point.0 as i32 + r.0 as i32, point.1 as i32 + r.1 as i32);
        if trp.0 < 0 || trp.1 < 0 || trp.0 >= grid[0].len() as i32 || trp.1 >= grid.len() as i32 {
            continue;
        }

        let rp = (trp.0 as usize, trp.1 as usize);

        if grid[rp.1][rp.0] <= elevation {
            ret.push(rp);
        }
    }

    ret
}

fn find_path(grid: &Grid, start: Point, dest: Point) -> Option<usize> {
    let mut queue: VecDeque<PFWork> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push_back(PFWork {
        pt: start,
        steps: 0,
    });
    while let Some(w) = queue.pop_front() {
        let elevation = grid[w.pt.1][w.pt.0];
        let next_elevation = elevation + 1;

        if visited.contains(&w.pt) {
            continue;
        } else {
            visited.insert(w.pt);
        }

        if w.pt == dest {
            return Some(w.steps);
        }

        let higher = find_elevation(&grid, &w.pt, next_elevation);
        if higher.len() > 0 {
            // Queue steps that progress in the direction we want to go
            for r in higher {
                queue.push_back(PFWork {
                    pt: r,
                    steps: w.steps + 1,
                });
            }
        } else {
            // Try to continue on the same level
            for r in find_elevation(&grid, &w.pt, elevation) {
                queue.push_back(PFWork {
                    pt: r,
                    steps: w.steps + 1,
                });
            }
        }
    }

    None
}

fn main() {
    let inp = fs::read_to_string("inputs/12.txt").unwrap();
    let mut grid = parse_grid(inp);

    let start = chars_to_points(&grid, b'S')[0];
    let dest = chars_to_points(&grid, b'E')[0];
    // Normalize the S and E points in the grid to their real
    // elevations.
    grid[start.1][start.0] = b'a';
    grid[dest.1][dest.0] = b'z';

    println!("Answer #1: {}", find_path(&grid, start, dest).unwrap());

    let candidates = chars_to_points(&grid, b'a');
    let mut results: Vec<usize> = vec![];
    for c in &candidates {
        let res = find_path(&grid, *c, dest);
        if res.is_some() {
            results.push(res.unwrap());
        }
    }
    println!("Answer #2: {}", results.iter().min().unwrap());
}
