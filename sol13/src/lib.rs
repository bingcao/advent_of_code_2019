use intcode::Program;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::io;

#[derive(Eq, PartialEq)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

pub struct Game {
    program: Program,
    screen: HashMap<(i128, i128), Tile>,
    pub score: i128,
    interactive: bool,
    paddle_x: i128,
    ball_x: i128,
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = *self
            .screen
            .keys()
            .map(|(x, _)| x)
            .max()
            .unwrap_or(&0)
            .max(&10);
        let height = *self
            .screen
            .keys()
            .map(|(_, y)| y)
            .max()
            .unwrap_or(&0)
            .max(&10);
        let mut chars = vec![];
        for y in 0..height {
            for x in 0..width {
                chars.push(match self.screen.get(&(x, y)).unwrap_or(&Tile::EMPTY) {
                    Tile::EMPTY => ' ',
                    Tile::WALL => 'x',
                    Tile::BLOCK => 'o',
                    Tile::PADDLE => '-',
                    Tile::BALL => '+',
                })
            }
            chars.push('\n');
        }
        write!(
            f,
            "Score: {}\n{}",
            self.score,
            chars.iter().collect::<String>()
        )
    }
}
impl Game {
    fn new(program_str: &str) -> Self {
        let program = Program::new(program_str, &vec![]);
        let screen = HashMap::new();
        Game {
            program,
            screen,
            score: 0,
            interactive: env::var_os("INTERACTIVE").is_some(),
            paddle_x: -1,
            ball_x: -1,
        }
    }

    fn get_input(&self) -> i128 {
        if self.interactive {
            loop {
                println!("Enter -1 (left), 0 (stay), or 1 (right)!");
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).unwrap();
                if let Ok(input) = user_input.trim().parse::<i128>() {
                    if input == 1 || input == 0 || input == -1 {
                        return input;
                    }
                } else if user_input == "\n" {
                    return 0;
                }
            }
        }

        if self.paddle_x < self.ball_x {
            1
        } else if self.paddle_x == self.ball_x {
            0
        } else {
            -1
        }
    }

    fn run(&mut self) {
        loop {
            let (outputs, done) = self.program.run_until_blocked_or_done();

            let mut i = 0;
            while i < outputs.len() {
                let output_set = &outputs[i..i + 3];
                let x = output_set[0];
                let y = output_set[1];
                if x == -1 {
                    self.score = output_set[2];
                } else {
                    let tile = match output_set[2] {
                        0 => Tile::EMPTY,
                        1 => Tile::WALL,
                        2 => Tile::BLOCK,
                        3 => {
                            self.paddle_x = x;
                            Tile::PADDLE
                        }
                        4 => {
                            self.ball_x = x;
                            Tile::BALL
                        }
                        _ => panic!("Invalid output {:?}", output_set),
                    };
                    self.screen.insert((x, y), tile);
                }
                i += 3;
            }
            println!("{}", self);

            if done {
                break;
            }

            self.program.send_input(self.get_input());
        }
    }

    pub fn num_blocks(&self) -> usize {
        self.screen.values().filter(|&t| *t == Tile::BLOCK).count()
    }
}

pub fn run_game(program_str: &str) -> Game {
    let mut game = Game::new(program_str);
    game.run();
    game
}
