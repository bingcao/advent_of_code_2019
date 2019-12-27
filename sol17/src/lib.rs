use std::fmt;

use intcode::Program;
use paths::{index_to_point, Direction, Point};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    OPEN,
    SCAFFOLD,
}
impl Tile {
    fn from_char(c: char) -> Self {
        if c == '.' {
            Tile::OPEN
        } else {
            Tile::SCAFFOLD
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::OPEN => '.',
            Tile::SCAFFOLD => '#',
        }
    }
}

pub struct Robot {
    program: Program,
    pub camera: Vec<Vec<Tile>>,
    position: Point<usize>,
    direction: Direction,
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars = vec![];
        for _ in 0..self.camera[0].len() + 2 {
            chars.push('-');
        }
        chars.push('\n');
        for y in (0..self.camera.len()).rev() {
            chars.push('|');
            for x in 0..self.camera[0].len() {
                let point = Point::new(x, y);
                if point == self.position {
                    chars.push(match self.direction {
                        Direction::NORTH => '^',
                        Direction::SOUTH => 'v',
                        Direction::EAST => '>',
                        Direction::WEST => '<',
                    })
                } else {
                    chars.push(self.tile_at(&point).to_char())
                }
            }
            chars.push('|');
            chars.push('\n');
        }
        for _ in 0..self.camera[0].len() + 2 {
            chars.push('-');
        }
        write!(f, "{}", chars.iter().collect::<String>())
    }
}
impl Robot {
    fn new(program_str: &str) -> Self {
        let program = Program::new(program_str, &vec![]);
        Robot {
            program,
            camera: vec![],
            position: Point::new(0, 0),
            direction: Direction::NORTH,
        }
    }

    fn tile_at(&self, point: &Point<usize>) -> Tile {
        self.camera[point.y as usize][point.x as usize].clone()
    }

    fn fill_map(&mut self) {
        let outputs = self.program.run();
        let mut rows = vec![];
        let mut row = vec![];
        let mut count = 0;
        let mut position_index = 0;
        for &output in outputs[..outputs.len() - 1].into_iter() {
            let c = output as u8 as char;
            match c {
                '\n' => {
                    rows.push(row);
                    row = vec![];
                }
                '^' => {
                    row.push(Tile::SCAFFOLD);
                    position_index = count;
                    self.direction = Direction::NORTH;
                    count += 1;
                }
                '>' => {
                    row.push(Tile::SCAFFOLD);
                    position_index = count;
                    self.direction = Direction::EAST;
                    count += 1;
                }
                'v' => {
                    row.push(Tile::SCAFFOLD);
                    position_index = count;
                    self.direction = Direction::SOUTH;
                    count += 1;
                }
                '<' => {
                    row.push(Tile::SCAFFOLD);
                    position_index = count;
                    self.direction = Direction::WEST;
                    count += 1;
                }
                _ => {
                    row.push(Tile::from_char(c));
                    count += 1;
                }
            }
        }
        rows = rows.into_iter().rev().collect();
        let width = rows[0].len();
        let height = rows.len();
        let mut position = index_to_point(position_index, width);
        position.y = height - position.y - 1;
        self.position = position;
        self.camera = rows;
    }

    fn clean(&mut self) -> i128 {
        self.program.set(0, 2);
        let f = [
            'A', ',', 'B', ',', 'A', ',', 'C', ',', 'A', ',', 'A', ',', 'C', ',', 'B', ',', 'C',
            ',', 'B', '\n',
        ];
        let f_a = [
            'L', ',', '1', '2', ',', 'L', ',', '8', ',', 'R', ',', '1', '2', '\n',
        ];
        let f_b = [
            'L', ',', '1', '0', ',', 'L', ',', '8', ',', 'L', ',', '1', '2', ',', 'R', ',', '1',
            '2', '\n',
        ];
        let f_c = [
            'R', ',', '1', '2', ',', 'L', ',', '8', ',', 'L', ',', '1', '0', '\n',
        ];

        for c in f.iter() {
            self.program.send_input(*c as i128);
        }
        for c in f_a.iter() {
            self.program.send_input(*c as i128);
        }
        for c in f_b.iter() {
            self.program.send_input(*c as i128);
        }
        for c in f_c.iter() {
            self.program.send_input(*c as i128);
        }
        self.program.send_input('n' as i128);
        self.program.send_input('\n' as i128);

        let output = self.program.run();
        *output.iter().last().unwrap()
    }
}

pub fn find_alignments(map: &Vec<Vec<Tile>>) -> Vec<usize> {
    let mut intersections = vec![];
    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] != Tile::SCAFFOLD {
                continue;
            }
            let point = Point::new(x, y);
            if Direction::iter()
                .map(|d| point.move_in_dir(&d))
                .filter(|p| map[p.y][p.x] == Tile::SCAFFOLD)
                .count()
                == 4
            {
                intersections.push(point);
            };
        }
    }
    let height = map.len();
    println!("{:?}", intersections);
    println!(
        "{:?}",
        intersections
            .iter()
            .map(|i| (height - i.y - 1, i.x))
            .collect::<Vec<(usize, usize)>>()
    );
    intersections
        .iter()
        .map(|i| (height - i.y - 1) * i.x)
        .collect()
}

pub fn run_robot(program_str: &str) -> Robot {
    let mut robot = Robot::new(program_str);
    robot.fill_map();
    robot
}

pub fn clean_robot(program_str: &str) -> i128 {
    let mut robot = Robot::new(program_str);
    robot.clean()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let map_str = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...#..\n";
        let mut rows = vec![];
        let mut row = vec![];
        for c in map_str.chars() {
            if c == '\n' {
                rows.push(row);
                row = vec![];
            } else {
                row.push(Tile::from_char(c));
            }
        }
        for row in rows.iter() {
            println!("{:?}", row);
        }
        println!("");

        let map = rows.into_iter().rev().collect::<Vec<Vec<Tile>>>();
        for row in map.iter() {
            println!("{:?}", row);
        }

        let result = find_alignments(&map);
        assert_eq!(result.iter().sum::<usize>(), 76);
    }
}
