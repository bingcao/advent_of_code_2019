use std::cmp::{min, Ordering};

#[derive(Debug)]
#[derive(Hash)]
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x.abs() + self.y.abs()).cmp(&(other.x.abs() + other.y.abs()))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }

    fn add_dir_and_mag(&self, direction: &str, magnitude: i32) -> Point {
        match direction {
            "R" => Point {x: self.x + magnitude, y: self.y},
            "L" => Point {x: self.x - magnitude, y: self.y},
            "U" => Point {x: self.x, y: self.y + magnitude},
            "D" => Point {x: self.x, y: self.y - magnitude},
            _  => panic!("Invalid direction, got {}", direction)
        }
    }

    fn is_between(&self, line: &Line) -> bool {
        match line.dir {
            Direction::HORIZONTAL => self.x >= line.a.x && self.x <= line.b.x,
            Direction::VERTICAL => self.y >= line.a.y && self.y <= line.b.y,
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    HORIZONTAL, VERTICAL
}

#[derive(Debug)]
pub struct Line {
    a: Point, 
    b: Point,
    dir: Direction
}

impl Line {
    fn new(a: &Point, b: &Point) -> Line {
        if a.x == b.x {
            if a.y < b.y {
                Line {a: a.clone(), b: b.clone(), dir: Direction::VERTICAL}
            } else {
                Line {a: b.clone(), b: a.clone(), dir: Direction::VERTICAL}
            }
        } else if a.y == b.y {
            if a.x < b.x {
                Line {a: a.clone(), b: b.clone(), dir: Direction::HORIZONTAL}
            } else {
                Line {a: a.clone(), b: b.clone(), dir: Direction::HORIZONTAL}
            }
        } else {
            panic!("Points must share one axis: {:?}, {:?}", a, b);
        }
    }

    fn find_intersect(&self, other: &Self) -> Option<Point> {
        if self.dir == other.dir {
            if self.dir == Direction::HORIZONTAL {
                if (self.a.y != other.a.y) || (self.b.x < other.a.x) || (self.a.x > other.b.x) {
                    None
                } else {
                    panic!("Need better intersection");
                }
            } else {
                if (self.a.x != other.a.x) || (self.b.y < other.a.y) || (self.a.y > other.b.y) {
                    None
                } else {
                    panic!("Need better intersection");
                }
            }
        } else if self.a.is_between(other) && other.a.is_between(self) {
            match self.dir {
                Direction::HORIZONTAL => Some(Point{x: other.a.x, y: self.a.y}),
                Direction::VERTICAL => Some(Point{x: self.a.x, y: other.a.y}),
            }
        } else {
            None
        }
    }
}

fn get_line_list(path_string: &Vec<&str>) -> Vec<Line> {
    let mut lines = vec![];
    let mut prev = Point::new(0, 0);
    for segment in path_string {
        let direction = &segment[..1];
        let magnitude: i32 = segment[1..].parse().unwrap();
        let next = prev.add_dir_and_mag(direction, magnitude);
        lines.push(Line::new(&prev, &next));
        prev = next;
    }
    lines
}

pub fn find_closest_intersection_distance(path_string_a: &Vec<&str>, path_string_b: &Vec<&str>) -> i32  {
    let line_list_a = get_line_list(path_string_a);
    let line_list_b = get_line_list(path_string_b);
    let mut min_distance = i32::max_value();
    for line_a in line_list_a.iter() {
        for line_b in line_list_b.iter() {
            if let Some(point) = line_a.find_intersect(line_b) {
                if point.x == 0 && point.y == 0 {
                    continue
                }
                println!("{:?} intersected {:?} for {:?}", line_a, line_b, point);
                min_distance = min(min_distance, point.distance());
            }
        }
    }
    min_distance
}

pub fn find_steppiest_intersection_distance(path_string_a: &Vec<&str>, path_string_b: &Vec<&str>) -> usize  {
    0
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        let result = find_closest_intersection_distance(
            &vec!["R3", "U3"],
            &vec!["U2", "R4"],
        );
        assert_eq!(result, 5);
    }


    #[test]
    fn test_case_2() {
        let result = find_closest_intersection_distance(
            &vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"],
            &vec!["U62","R66","U55","R34","D71","R55","D58","R83"],
        );
        assert_eq!(result, 159);

        let result = find_steppiest_intersection_distance(
            &vec!["R75","D30","R83","U83","L12","D49","R71","U7","L72"],
            &vec!["U62","R66","U55","R34","D71","R55","D58","R83"],
        );
        assert_eq!(result, 610, "failed steppiest");
    }

    #[test]
    fn test_case_3() {
        let result = find_closest_intersection_distance(
            &vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
            &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]
        );
        assert_eq!(result, 135);

        let result = find_steppiest_intersection_distance(
            &vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
            &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]
        );
        assert_eq!(result, 410, "failed steppiest");
    }
}
