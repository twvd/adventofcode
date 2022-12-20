use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::fs;

const PART1_TARGET: i128 = 2022;
const PART2_TARGET: i128 = 1000000000000;

const DROP_HEIGHT: usize = 4;
const DROP_OFFSET: usize = 2;

const FIELD_WIDTH: usize = 7;

#[rustfmt::skip]
const BLOCKS: [[[bool; 4]; 4]; 5] = [
    [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [true,  true,  true,  true ],
    ],
    [
        [false, false, false, false],
        [false, true,  false, false],
        [true,  true,  true,  false],
        [false, true,  false, false],
    ],
    [
        [false, false, false, false],
        [false, false, true,  false],
        [false, false, true,  false],
        [true,  true,  true,  false],
    ],
    [
        [true,  false, false, false],
        [true,  false, false, false],
        [true,  false, false, false],
        [true,  false, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true,  true,  false, false],
        [true,  true,  false, false],
    ],
];

const BLOCKWIDTH: [usize; 5] = [4, 3, 3, 1, 2];

type BlockRow = Vec<bool>;
type Block = Vec<BlockRow>;
type FieldRow = BlockRow;

fn extend_block(block: &Block, sz: usize) -> Block {
    let mut ret = block.clone();
    for i in 0..ret.len() {
        let ext: BlockRow = vec![false; sz - block.len()];
        ret[i].extend(ext);
    }
    ret
}

fn shift_block(block: &Block, steps: i32) -> Block {
    let mut ret = block.clone();
    if steps > 0 {
        for i in 0..ret.len() {
            ret[i].rotate_right(steps as usize);
        }
    } else if steps < 0 {
        for i in 0..ret.len() {
            ret[i].rotate_left(steps.abs() as usize);
        }
    }

    ret
}

struct Field {
    // Field is upside-down: the bottom is row 0.
    blocks: Vec<FieldRow>,
    width: usize,
}

impl Field {
    fn new(width: usize) -> Field {
        Field {
            blocks: vec![],
            width,
        }
    }

    fn gen_row(&self, val: bool) -> FieldRow {
        (0..self.width).map(|_| val).collect::<FieldRow>()
    }

    fn available_space(&self) -> usize {
        self.blocks
            .iter()
            .rev()
            .take_while(|r| r.iter().all(|x| !x))
            .count()
    }

    fn stack_height(&self) -> usize {
        self.blocks
            .iter()
            .take_while(|r| r.iter().any(|x| *x))
            .count()
    }

    fn ensure_space(&mut self, space: usize) {
        while self.available_space() < space {
            self.blocks.push(self.gen_row(false));
        }
    }

    fn can_merge(&self, block: &Block, bottom_y: usize) -> bool {
        for (rel_y, blrow) in block.iter().rev().enumerate() {
            let y = bottom_y + rel_y;
            for (x, &blc) in blrow.iter().enumerate() {
                if blc && self.blocks[y][x] {
                    return false;
                }
            }
        }

        true
    }

    fn merge_block(&mut self, block: &Block, bottom_y: usize) {
        // Integrate block into field, bottom-up
        for (rel_y, blrow) in block.iter().rev().enumerate() {
            let y = bottom_y + rel_y;
            for (x, blc) in blrow.iter().enumerate() {
                if *blc {
                    assert!(!self.blocks[y][x]);
                    self.blocks[y][x] = true;
                }
            }
        }
    }

    fn drop_block(
        &mut self,
        block: &Block,
        block_width: usize,
        jets: &mut impl Iterator<Item = (usize, char)>,
    ) {
        self.ensure_space(DROP_HEIGHT + block.len());

        let mut x: i32 = DROP_OFFSET as i32;
        let mut drop_block = shift_block(&extend_block(block, self.width), x as i32);

        for y in (0..(self.stack_height() + DROP_HEIGHT)).rev() {
            let newx = min(
                (self.width - block_width) as i32,
                match jets.next().unwrap() {
                    (_, '>') => x + 1,
                    (_, '<') => max(0, x - 1),
                    _ => panic!("Unknown input"),
                },
            );

            // Try to shift block
            if newx != x {
                let sh_block = shift_block(&drop_block, newx - x);
                // Check horizontal collision
                if self.can_merge(&sh_block, y) {
                    drop_block = sh_block;
                    x = newx;
                }
            }

            // Check for vertical collision
            if y == 0 || !self.can_merge(&drop_block, y - 1) {
                self.merge_block(&drop_block, y);
                break;
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " {} ", "-".repeat(self.width))?;
        for l in self.blocks.iter().rev() {
            writeln!(
                f,
                "|{}|",
                l.iter()
                    .map(|x| if *x { "#" } else { " " })
                    .collect::<Vec<&str>>()
                    .join("")
            )?;
        }
        write!(f, " {} ", "-".repeat(self.width))?;
        Ok(())
    }
}

fn part1(inp: &str) -> usize {
    let mut jets = inp
        .trim()
        .chars()
        .into_iter()
        .enumerate()
        .cycle()
        .peekable();
    let mut f = Field::new(FIELD_WIDTH);
    let mut blk = 0;

    for _ in 0..PART1_TARGET {
        let bl = Vec::from(BLOCKS[blk].map(|a| Vec::from(a)));
        f.drop_block(&bl, BLOCKWIDTH[blk], &mut jets);
        blk = (blk + 1) % BLOCKS.len();
    }

    f.stack_height()
}

fn part2(inp: &str) -> i128 {
    let mut jets = inp
        .trim()
        .chars()
        .into_iter()
        .enumerate()
        .cycle()
        .peekable();
    let mut f = Field::new(FIELD_WIDTH);
    let mut blk = 0;
    let mut cache: HashMap<(usize, usize), (i128, i128)> = HashMap::new(); // K = (blkidx, jetidx), V = (cycles, height)

    for ticks in 0i128.. {
        let bl = Vec::from(BLOCKS[blk].map(|a| Vec::from(a)));
        f.drop_block(&bl, BLOCKWIDTH[blk], &mut jets);
        blk = (blk + 1) % BLOCKS.len();
        let (jetpos, _) = *jets.peek().unwrap();

        if ticks > 1000 {
            if let Some(cacheval) = cache.get(&(blk, jetpos)) {
                // Cycle detected..
                let (cv_ticks, cv_height) = *cacheval;
                let cycle_period = ticks - cv_ticks;
                let ticks_left = PART2_TARGET - ticks - 1;
                if (ticks_left % cycle_period) == 0 {
                    // .. and aligned.
                    return (f.stack_height() as i128 - cv_height) * (ticks_left / cycle_period)
                        + f.stack_height() as i128;
                }
            }
            cache.insert((blk, jetpos), (ticks, f.stack_height() as i128));
        }
    }

    f.stack_height() as i128
}

fn main() {
    let inp = fs::read_to_string("inputs/17.txt").unwrap();

    println!("Answer #1: {}", part1(&inp));
    println!("Answer #2: {}", part2(&inp));
}
