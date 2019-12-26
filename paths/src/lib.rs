use std::fmt;
use std::hash::Hash;
use std::ops::Add;
use std::slice::Iter;

use num::Integer;

#[derive(PartialEq, Eq, Hash)]
pub struct Point<T: Integer + Add<Output = T> + Copy> {
    x: T,
    y: T,
}
impl<T: Integer + fmt::Display + Copy> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl<T: Integer + Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn move_in_dir(&self, dir: &Direction) -> Point<T> {
        match dir {
            Direction::NORTH => Point::new(self.x, self.y + T::one()),
            Direction::SOUTH => Point::new(self.x, self.y - T::one()),
            Direction::EAST => Point::new(self.x + T::one(), self.y),
            Direction::WEST => Point::new(self.x - T::one(), self.y),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ];
        DIRECTIONS.into_iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
