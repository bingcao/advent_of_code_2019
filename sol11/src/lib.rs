use intcode::Program;
use std::collections::HashMap;
use std::env;
use std::fmt;

#[derive(Eq, PartialEq)]
enum PaintStatus {
    UNPAINTED,
    WHITE,
    BLACK,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn turn(&self, direction: i128) -> Direction {
        use Direction::{DOWN, LEFT, RIGHT, UP};
        match (self, direction) {
            (UP, 0) => LEFT,
            (LEFT, 0) => DOWN,
            (DOWN, 0) => RIGHT,
            (RIGHT, 0) => UP,
            (UP, 1) => RIGHT,
            (RIGHT, 1) => DOWN,
            (DOWN, 1) => LEFT,
            (LEFT, 1) => UP,
            (_, _) => panic!("Invalid direction to turn: {}", direction),
        }
    }
}

pub struct Robot {
    program: Program,
    position: (i32, i32),
    direction: Direction,
    map: HashMap<(i32, i32), PaintStatus>,
    debug: bool,
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = *self
            .map
            .keys()
            .map(|(x, _)| x)
            .min()
            .unwrap_or(&0)
            .min(&self.position.0);
        let max_x = *self
            .map
            .keys()
            .map(|(x, _)| x)
            .max()
            .unwrap_or(&0)
            .max(&10)
            .max(&self.position.0);
        let min_y = *self
            .map
            .keys()
            .map(|(_, y)| y)
            .min()
            .unwrap_or(&0)
            .min(&self.position.1);
        let max_y = *self
            .map
            .keys()
            .map(|(_, y)| y)
            .max()
            .unwrap_or(&0)
            .max(&10)
            .max(&self.position.1);
        let mut chars = vec![];
        for y in (min_y..max_y).rev() {
            for x in min_x..max_x {
                if (x, y) == self.position {
                    chars.push(match self.direction {
                        Direction::UP => '^',
                        Direction::LEFT => '<',
                        Direction::DOWN => 'v',
                        Direction::RIGHT => '>',
                    });
                } else {
                    chars.push(
                        match self.map.get(&(x, y)).unwrap_or(&PaintStatus::UNPAINTED) {
                            PaintStatus::UNPAINTED => '_',
                            PaintStatus::WHITE => '#',
                            PaintStatus::BLACK => '.',
                        },
                    )
                }
            }
            chars.push('\n');
        }
        write!(f, "{}", chars.iter().collect::<String>())
    }
}
impl Robot {
    fn new(program: Program) -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), PaintStatus::WHITE);
        Robot {
            program: program,
            position: (0, 0),
            direction: Direction::UP,
            map,
            debug: env::var_os("DEBUG").is_some(),
        }
    }

    fn move_dir(&mut self, direction: i128) {
        self.direction = self.direction.turn(direction);
        match self.direction {
            Direction::UP => self.position = (self.position.0, self.position.1 + 1),
            Direction::LEFT => self.position = (self.position.0 - 1, self.position.1),
            Direction::RIGHT => self.position = (self.position.0 + 1, self.position.1),
            Direction::DOWN => self.position = (self.position.0, self.position.1 - 1),
        };
    }

    fn send_input(&mut self) {
        let input = match self
            .map
            .get(&self.position)
            .unwrap_or(&PaintStatus::UNPAINTED)
        {
            PaintStatus::WHITE => 1,
            _ => 0,
        };
        self.program.send_input(input);
    }

    fn paint(&mut self) {
        self.send_input();

        let mut output_is_paint = true;
        loop {
            let (halt, output) = self.program.execute();
            if halt {
                break;
            }
            if output.is_some() {
                let output = output.unwrap();
                if output_is_paint {
                    let paint = match output {
                        0 => PaintStatus::BLACK,
                        1 => PaintStatus::WHITE,
                        _ => panic!("Invalid output {}", output),
                    };
                    self.map.insert(self.position, paint);
                    output_is_paint = false;
                } else {
                    self.move_dir(output);
                    output_is_paint = true;
                    if self.debug {
                        println!("{}", self);
                    }
                    self.send_input();
                }
            }
        }
    }

    pub fn num_painted(&self) -> usize {
        self.map
            .values()
            .filter(|&val| *val != PaintStatus::UNPAINTED)
            .count()
    }
}

pub fn paint(program_str: String) -> Robot {
    let program = Program::new(&program_str[..], &vec![]);
    let mut robot = Robot::new(program);
    robot.paint();
    robot
}
