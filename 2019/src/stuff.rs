use num_traits::sign::Signed;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point<T>(pub T, pub T);

impl<T: Signed + Copy> Point<T> {
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
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_udlr(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!(),
        }
    }
}
