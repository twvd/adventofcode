use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Image = Vec<u32>;
type Images = Vec<Image>;

fn part1(images: &Images) -> usize {
    let img = images
        .iter()
        .min_by_key(|img| img.iter().filter(|&&c| c == 0).count())
        .unwrap();
    img.iter().filter(|&&c| c == 1).count() * img.iter().filter(|&&c| c == 2).count()
}

fn part2(images: &Images) -> String {
    let flattened = images
        .iter()
        .rev()
        .fold(Vec::from([2; WIDTH * HEIGHT]), |a, l| {
            l.iter()
                .enumerate()
                .map(|(i, &px)| match px {
                    2 => a[i],
                    _ => px,
                })
                .collect()
        });

    flattened
        .into_iter()
        .map(|c| match c {
            0 => ' ',
            1 => '#',
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .chunks(WIDTH)
        .into_iter()
        .map(|s| String::from_iter(s.into_iter()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let inp = fs::read_to_string("inputs/8.txt").unwrap();

    let images: Images = inp
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .chunks(WIDTH * HEIGHT)
        .map(|c| Image::from(c))
        .collect();

    println!("Answer #1: {}", part1(&images));
    println!("Answer #2:\n{}", part2(&images));
}
