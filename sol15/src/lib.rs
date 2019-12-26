use intcode::Program;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    UNVISITED,
    VISITED,
    WALL,
    DEST,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
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

pub struct Droid {
    map: HashMap<(i128, i128), Tile>,
    position: (i128, i128),
    visited: HashSet<(i128, i128)>,
    program_states: HashMap<(i128, i128), Program>,
    oxygen_map: HashSet<(i128, i128)>,
    pub paths: HashMap<(i128, i128), Vec<(i128, i128)>>,
}
impl fmt::Display for Droid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = *self
            .map
            .keys()
            .map(|(x, _)| x)
            .min()
            .unwrap_or(&0)
            .min(&-10)
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
            .min(&-10)
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
        for _ in min_x..max_x + 2 {
            chars.push('-');
        }
        chars.push('\n');
        for y in (min_y..max_y).rev() {
            chars.push('|');
            // chars.extend(y.to_string().chars());
            for x in min_x..max_x {
                let position = (x, y);
                if self.oxygen_map.contains(&position) {
                    chars.push('O');
                } else if position == self.position {
                    chars.push('D');
                } else {
                    chars.push(match self.map.get(&position).unwrap_or(&Tile::UNVISITED) {
                        Tile::UNVISITED => ' ',
                        Tile::VISITED => '.',
                        Tile::WALL => '#',
                        Tile::DEST => 'o',
                    })
                }
            }
            chars.push('|');
            chars.push('\n');
        }
        for _ in min_x..max_x + 2 {
            chars.push('-');
        }
        write!(f, "{}", chars.iter().collect::<String>())
    }
}
impl Droid {
    fn new(program_str: &str) -> Self {
        let program = Program::new(program_str, &vec![]);
        let mut map = HashMap::new();
        let position = (0, 0);
        map.insert(position, Tile::VISITED);
        let mut visited = HashSet::new();
        visited.insert(position);
        let mut program_states = HashMap::new();
        program_states.insert(position, program);
        let mut paths = HashMap::new();
        paths.insert(position, vec![]);
        Droid {
            map,
            position: position,
            visited,
            program_states,
            paths,
            oxygen_map: HashSet::new(),
        }
    }

    fn get_output_position(&self, dir: &Direction) -> (i128, i128) {
        match dir {
            Direction::NORTH => (self.position.0, self.position.1 + 1),
            Direction::SOUTH => (self.position.0, self.position.1 - 1),
            Direction::EAST => (self.position.0 + 1, self.position.1),
            Direction::WEST => (self.position.0 - 1, self.position.1),
        }
    }

    fn get_tile_at(&mut self, position: (i128, i128)) -> Tile {
        self.map.entry(position).or_insert(Tile::UNVISITED).clone()
    }

    fn add_path(&mut self, old_position: (i128, i128), new_position: (i128, i128)) {
        let mut path = self.paths.get(&old_position).unwrap().clone();
        path.push(new_position);
        self.paths.insert(new_position, path);
    }

    fn explore(&mut self) {
        let mut horizon = vec![self.position];
        while horizon.len() > 0 {
            let mut new_horizon = vec![];
            for position in horizon.into_iter() {
                for direction in Direction::iter() {
                    // println!("{}", self);
                    self.position = position;
                    let next_position = self.get_output_position(&direction);
                    if self.visited.contains(&next_position)
                        || self.get_tile_at(next_position) != Tile::UNVISITED
                    {
                        continue;
                    }
                    let mut program = self.program_states.get(&position).unwrap().clone();
                    let input = match direction {
                        Direction::NORTH => 1,
                        Direction::WEST => 3,
                        Direction::SOUTH => 2,
                        Direction::EAST => 4,
                    };
                    program.send_input(input);

                    let (outputs, done) = program.run_until_blocked_or_done();
                    if done {
                        panic!("Hit unexpected halt");
                    }
                    assert_eq!(outputs.len(), 1, "Invalid outputs: {:?}", outputs);
                    let output = outputs[0];
                    if output == 0 {
                        self.map.insert(next_position, Tile::WALL);
                    } else if output == 1 {
                        self.map.insert(next_position, Tile::VISITED);
                        new_horizon.push(next_position);
                        self.program_states.insert(next_position, program.clone());
                        self.add_path(position, next_position);
                    } else if output == 2 {
                        self.map.insert(next_position, Tile::DEST);
                        new_horizon.push(next_position);
                        self.program_states.insert(next_position, program.clone());
                        self.add_path(position, next_position);
                    } else {
                        panic!("Invalid output: {}", output);
                    }
                }
            }
            horizon = new_horizon;
        }
    }

    pub fn get_dest_location(&self) -> (i128, i128) {
        self.map
            .iter()
            .filter_map(|(position, tile)| match tile {
                Tile::DEST => Some(*position),
                _ => None,
            })
            .collect::<Vec<(i128, i128)>>()[0]
    }

    pub fn fill_oxygen(&mut self) -> usize {
        let dest_location = self.get_dest_location();
        self.oxygen_map.insert(dest_location);
        let mut visited = HashSet::new();
        visited.insert(dest_location);
        let mut horizon = vec![dest_location];

        let mut i = 0;
        loop {
            let mut new_horizon = vec![];
            for position in horizon.into_iter() {
                for direction in Direction::iter() {
                    self.position = position;
                    let next_position = self.get_output_position(&direction);
                    if visited.contains(&next_position)
                        || self.get_tile_at(next_position) == Tile::WALL
                    {
                        continue;
                    }

                    self.oxygen_map.insert(next_position);
                    visited.insert(next_position);
                    new_horizon.push(next_position);
                    // println!("Step {}:\n{}", i, self);
                }
            }
            if new_horizon.len() == 0 {
                break;
            }
            horizon = new_horizon;
            i += 1;
        }
        i
    }
}

pub fn run_droid(program_str: &str) -> Droid {
    let mut droid = Droid::new(program_str);
    droid.explore();
    droid
}
