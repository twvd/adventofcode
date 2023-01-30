use num_traits::identities::{One, Zero};
use num_traits::sign::Signed;
use std::ops::Add;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point<T>(pub T, pub T);

impl<T: Signed> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Signed + Copy + One + Zero> Point<T> {
    pub fn move_dir(&self, dir: Direction, steps: T) -> Point<T> {
        match dir {
            Direction::Up => Point(self.0, self.1 + steps),
            Direction::Down => Point(self.0, self.1 - steps),
            Direction::Left => Point(self.0 - steps, self.1),
            Direction::Right => Point(self.0 + steps, self.1),
        }
    }

    pub fn manhattan(&self, other: Point<T>) -> T {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn adjacent_diagonal(&self) -> Vec<Self> {
        let one: T = T::one();
        let min_one: T = T::zero() - T::one();

        [
            Point::<T>(min_one, min_one),
            Point::<T>(one, min_one),
            Point::<T>(min_one, one),
            Point::<T>(one, one),
        ]
        .into_iter()
        .map(|p| *self + p)
        .collect()
    }

    pub fn adjacent_straight(&self) -> Vec<Self> {
        Direction::iter()
            .map(|d| self.move_dir(d, T::one()))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_ascii(c: char) -> Direction {
        match c {
            'L' | '<' => Direction::Left,
            'R' | '>' => Direction::Right,
            'U' | '^' => Direction::Up,
            'D' | 'v' => Direction::Down,
            _ => unreachable!(),
        }
    }

    pub fn rotate_90deg(&self, d: Direction) -> Direction {
        match d {
            Direction::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            Direction::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            _ => unreachable!(),
        }
    }
}
