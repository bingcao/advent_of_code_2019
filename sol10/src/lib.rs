use num_rational::Ratio;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Hash, PartialEq, Eq)]
enum Slope {
    RATIO(Ratio<i32>, usize),
    VERTICAL(bool),
    HORIZONTAL(bool),
}
impl fmt::Debug for Slope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Slope::RATIO(r, q) => write!(f, "{}/{} to {}", r.numer(), r.denom(), q),
            Slope::VERTICAL(true) => write!(f, "+inf"),
            Slope::VERTICAL(false) => write!(f, "-inf"),
            Slope::HORIZONTAL(true) => write!(f, "+0"),
            Slope::HORIZONTAL(false) => write!(f, "-0"),
        }
    }
}
impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Slope {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Slope::VERTICAL(true), Slope::VERTICAL(true)) => Ordering::Equal,
            (Slope::VERTICAL(true), _) => Ordering::Less,
            (_, Slope::VERTICAL(true)) => Ordering::Greater,
            (Slope::VERTICAL(false), Slope::HORIZONTAL(right)) => match right {
                true => Ordering::Greater,
                false => Ordering::Less,
            },
            (Slope::HORIZONTAL(right), Slope::VERTICAL(false)) => match right {
                true => Ordering::Less,
                false => Ordering::Greater,
            },
            (Slope::VERTICAL(false), Slope::RATIO(_r, q)) => {
                if *q > 2 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Slope::RATIO(_r, q), Slope::VERTICAL(false)) => {
                if *q > 2 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Slope::HORIZONTAL(right1), Slope::HORIZONTAL(right2)) => right2.cmp(right1),
            (Slope::RATIO(_r, q), Slope::HORIZONTAL(right)) => {
                if *right {
                    if *q == 1 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    if *q <= 3 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
            (Slope::HORIZONTAL(right), Slope::RATIO(_r, q)) => {
                if *right {
                    if *q == 1 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else {
                    if *q <= 3 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
            (Slope::RATIO(r1, q1), Slope::RATIO(r2, q2)) => {
                if q1 != q2 {
                    q1.cmp(q2)
                } else {
                    match q1 {
                        1 | 3 => r2.cmp(r1),
                        2 | 4 => r1.cmp(r2),
                        _ => panic!("Invalid quadrant {}", q1),
                    }
                }
            }
            (_, _) => Ordering::Equal,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64).sqrt()
    }

    fn dir_to(&self, other: &Self) -> Slope {
        if self.x == other.x {
            Slope::VERTICAL(other.y > self.y)
        } else if self.y == other.y {
            Slope::HORIZONTAL(other.x > self.x)
        } else {
            let y_diff = other.y - self.y;
            let x_diff = other.x - self.x;
            let x_sign = x_diff / x_diff.abs();
            let y_sign = y_diff / y_diff.abs();
            let quadrant = match (x_sign, y_sign) {
                (1, 1) => 1,
                (1, -1) => 2,
                (-1, -1) => 3,
                (-1, 1) => 4,
                (_, _) => panic!("Invalid signs"),
            };
            let r = Ratio::new(y_diff.abs(), x_diff.abs());
            Slope::RATIO(r, quadrant)
        }
    }
}

struct Map {
    locations: Vec<Point>,
    height: i32,
}
impl Map {
    fn new(map_string: &str) -> Self {
        let width = map_string.chars().position(|c| c == '\n').unwrap() as i32;
        let height = map_string.len() as i32 / width;
        let map_string = map_string
            .chars()
            .filter(|&c| c != '\n')
            .collect::<String>();
        let locations = map_string
            .chars()
            .enumerate()
            .filter(|(_i, c)| *c == '#')
            .map(|(index, _c)| {
                Point::new(
                    index as i32 % width,
                    height - 1 - (index as i32 - (index as i32 % width)) / width,
                )
            })
            .collect::<Vec<Point>>();
        Map { locations, height }
    }

    fn get_best(&self) -> ((i32, i32), usize) {
        let mut dirs = HashMap::new();

        for loc_a in &self.locations {
            let mut loc_a_dirs = HashSet::new();
            for loc_b in &self.locations {
                if loc_a == loc_b {
                    continue;
                }
                loc_a_dirs.insert(loc_a.dir_to(&loc_b));
            }
            dirs.insert(loc_a.to_tuple(), loc_a_dirs);
        }

        let result = dirs
            .iter()
            .map(|(l, dirs)| (l, dirs.len()))
            .max_by(|(_l_a, count_a), (_l_b, count_b)| count_a.cmp(&count_b))
            .unwrap();
        let (loc, count) = result;
        ((loc.0, self.height - 1 - loc.1), count)
    }

    fn get_vape_order(&self, loc: (i32, i32)) -> Vec<(i32, i32)> {
        let mut order = vec![];
        let mut dir_to_locs = HashMap::new();
        let mut distances: HashMap<Point, f64> = HashMap::new();

        let start = Point::new(loc.0, self.height - 1 - loc.1);
        for ast in &self.locations {
            if start == *ast {
                continue;
            }

            let dir = start.dir_to(&ast);

            if !dir_to_locs.contains_key(&dir) {
                dir_to_locs.insert(dir.clone(), HashSet::new());
            }

            dir_to_locs.get_mut(&dir).unwrap().insert(ast);
            distances.insert(ast.clone(), start.distance(&ast));
        }

        let mut sorted_dirs = dir_to_locs.keys().cloned().collect::<Vec<Slope>>();
        sorted_dirs.sort();

        let mut exit = false;
        while !exit {
            for dir in &sorted_dirs {
                let locs = match dir_to_locs.get_mut(dir) {
                    Some(locs) => locs,
                    None => continue,
                };
                let closest = locs
                    .iter()
                    .min_by(|a, b| {
                        distances
                            .get(a)
                            .unwrap()
                            .partial_cmp(&distances.get(b).unwrap())
                            .unwrap_or(Ordering::Less)
                    })
                    .unwrap()
                    .clone();
                let tup = closest.to_tuple();
                order.push((tup.0, self.height - 1 - tup.1));
                locs.remove(closest);
                if locs.len() == 0 {
                    dir_to_locs.remove(dir);
                    if dir_to_locs.len() == 0 {
                        exit = true;
                        break;
                    }
                }
            }
        }

        order
    }
}

pub fn get_best_location(map_string: &String) -> ((i32, i32), usize) {
    let map = Map::new(map_string);
    map.get_best()
}

pub fn get_vape_order(map_string: &String, loc: (i32, i32)) -> Vec<(i32, i32)> {
    let map = Map::new(map_string);
    map.get_vape_order(loc)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_location {
        use super::*;

        #[test]
        fn test_1() {
            let map_str = ".#..#
.....
#####
....#
...##";
            let result = get_best_location(&String::from(map_str));
            assert_eq!(result, ((3, 4), 8));
        }

        #[test]
        fn test_2() {
            let map_str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
            let result = get_best_location(&String::from(map_str));
            assert_eq!(result, ((5, 8), 33));
        }

        #[test]
        fn test_3() {
            let map_str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
            let result = get_best_location(&String::from(map_str));
            assert_eq!(result, ((1, 2), 35));
        }

        #[test]
        fn test_4() {
            let map_str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
            let result = get_best_location(&String::from(map_str));
            assert_eq!(result, ((6, 3), 41));
        }
    }

    mod test_vape {
        use super::*;

        #[test]
        fn test_1() {
            let map_str = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##";
            let result = get_vape_order(&String::from(map_str), (8, 3));
            assert_eq!(
                result[..10],
                [
                    (8, 1),
                    (9, 0),
                    (9, 1),
                    (10, 0),
                    (9, 2),
                    (11, 1),
                    (12, 1),
                    (11, 2),
                    (15, 1),
                    (12, 2)
                ]
            );
        }

        #[test]
        fn test_2() {
            let map_str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
            let result = get_vape_order(&String::from(map_str), (11, 13));
            assert_eq!(result[199], (8, 2));
        }
    }
}
