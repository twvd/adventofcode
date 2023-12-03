use itertools::Itertools;
use std::fs;

type Num = usize;
type Coord = (usize, usize);
type Coords = Vec<Coord>;

fn main() {
    let f = fs::read_to_string("inputs/3.txt").unwrap();

    let mut nums = Vec::<(Num, Coords)>::new();
    let mut symbols = Coords::new();
    let mut gears = Coords::new();
    for (y, l) in f.trim().lines().map(|l| l.trim()).enumerate() {
        let mut l_iter = l.chars().enumerate();
        while let Some((x, c)) = l_iter.next() {
            match c {
                '.' => (),
                '0'..='9' => {
                    // Collect the full number
                    let mut num_str = String::from(c);
                    num_str += &l_iter
                        .clone()
                        .take_while(|(_, i)| i.is_digit(10))
                        .map(|(_, i)| i)
                        .collect::<String>();
                    let num = num_str.parse::<usize>().unwrap();
                    l_iter = l_iter.dropping(num_str.len() - 1);

                    // Cache adjacent coordinates
                    let x_end = x + num_str.len();
                    let x_iter = (x.saturating_sub(1))..(x_end + 1);
                    let adjacent = Coords::from_iter(
                        x_iter
                            .clone()
                            .map(|x| (x, y))
                            .chain(x_iter.clone().map(|x| (x, y.saturating_sub(1))))
                            .chain(x_iter.clone().map(|x| (x, y + 1)))
                            .unique(),
                    );

                    nums.push((num, adjacent));
                }
                '*' => {
                    symbols.push((x, y));
                    gears.push((x, y));
                }
                _ => symbols.push((x, y)),
            }
        }
    }

    let part1 = nums
        .iter()
        .filter(|(_, a)| a.iter().any(|c| symbols.contains(c)))
        .fold(0, |a, (n, _)| a + n);

    println!("Answer #1: {}", part1);

    let part2 = gears
        .iter()
        .filter(|p| nums.iter().filter(|(_, a)| a.contains(p)).count() == 2)
        .map(|p| {
            nums.iter()
                .filter(|(_, a)| a.iter().contains(p))
                .fold(1, |a, (n, _)| a * n)
        })
        .sum::<usize>();
    println!("Answer #2: {}", part2);
}
