use std::cmp::min;
use std::fs;

struct File<'a> {
    #[allow(dead_code)]
    name: &'a str,

    size: usize,
}

struct Dir<'a> {
    #[allow(dead_code)]
    name: &'a str,

    subdirs: Vec<Dir<'a>>,
    files: Vec<File<'a>>,
}

impl<'a> Dir<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            subdirs: vec![],
            files: vec![],
        }
    }

    fn mkdir(&mut self, name: &'a str) -> &mut Dir<'a> {
        let n = Self::new(name);
        self.subdirs.push(n);
        self.subdirs.last_mut().unwrap()
    }

    fn mkfile(&mut self, name: &'a str, size: usize) {
        self.files.push(File { name, size });
    }

    fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self.subdirs.iter().map(|d| d.total_size()).sum::<usize>()
    }
}

fn parse_dir<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    dir: &mut Dir<'a>,
) -> Result<(), &'a str> {
    while let Some(ref mut l) = lines.next() {
        let ls: Vec<&str> = l.split(' ').collect();

        match ls[0] {
            "$" => match ls[1] {
                "ls" => continue,
                "cd" => match ls[2] {
                    ".." => return Ok(()),
                    _ => parse_dir(lines, dir.mkdir(ls[2])).unwrap(),
                },
                _ => return Err(ls[1]),
            },
            "dir" => continue,
            _ => dir.mkfile(ls[1], ls[0].parse().unwrap()),
        }
    }
    Ok(())
}

fn part1<'a>(dir: &Dir<'a>) -> usize {
    let size_limit = 100000;

    dir.subdirs
        .iter()
        .filter_map(|d| {
            let s = d.total_size();
            if s <= size_limit {
                Some(part1(d) + s)
            } else {
                Some(part1(d))
            }
        })
        .sum::<usize>()
}

fn part2<'a>(root: &Dir<'a>) -> usize {
    let disk_size = 70000000;
    let required_space = 30000000;
    let free_space = disk_size - root.total_size();

    assert!(free_space < required_space);

    let space_diff = required_space - free_space;

    part2_in(root, space_diff)
}

fn part2_in<'a>(dir: &Dir<'a>, space_diff: usize) -> usize {
    match dir
        .subdirs
        .iter()
        .filter_map(|d| {
            let s = d.total_size();
            if s >= space_diff {
                Some(min(part2_in(d, space_diff), s))
            } else {
                None
            }
        })
        .min()
    {
        Some(i) => i,
        None => usize::MAX,
    }
}

fn main() {
    let inp = fs::read_to_string("inputs/7.txt").unwrap();
    let mut lines = inp.lines();
    let mut root = Dir::new("");
    parse_dir(&mut lines, &mut root).expect("Input parse error");

    println!("Answer #1: {}", part1(&root));
    println!("Answer #2: {}", part2(&root));
}
