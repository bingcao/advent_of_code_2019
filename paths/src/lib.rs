use std::fmt;
use std::hash::Hash;
use std::ops::Add;
use std::slice::Iter;

use num::Integer;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point<T: Integer + Add<Output = T> + Copy> {
    pub x: T,
    pub y: T,
}
impl<T: Integer + fmt::Display + Copy> fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

    pub fn move_in_dir_y_rev(&self, dir: &Direction) -> Point<T> {
        match dir {
            Direction::NORTH => Point::new(self.x, self.y - T::one()),
            Direction::SOUTH => Point::new(self.x, self.y + T::one()),
            Direction::EAST => Point::new(self.x + T::one(), self.y),
            Direction::WEST => Point::new(self.x - T::one(), self.y),
        }
    }
}

pub fn index_to_point<T: Integer + Copy>(index: T, width: T) -> Point<T> {
    // Index = width * y + x
    let x = index % width;
    let y = (index - x) / width;
    Point::new(x, y)
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
    use super::*;

    mod test_index_to_point {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(index_to_point(3, 5), Point::new(3, 0));
        }

        #[test]
        fn test_2() {
            assert_eq!(index_to_point(5, 5), Point::new(0, 1));
        }

        #[test]
        fn test_3() {
            assert_eq!(index_to_point(12, 5), Point::new(2, 2));
        }
    }
}
