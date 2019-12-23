use std::cell::{Ref, RefCell};

const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JIT: i32 = 5;
const JIF: i32 = 6;
const LESS: i32 = 7;
const EQUALS: i32 = 8;
const STOP: i32 = 99;
//
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Command {
    ADD,
    MULTIPLY,
    INPUT,
    OUTPUT,
    JIT,
    JIF,
    LESS,
    EQUALS,
    STOP,
}

const POSITION: u32 = 0;
const IMMEDIATE: u32 = 1;

const LEN_COMMAND: usize = 2;
const OPER_NUM_PARAMS: usize = 3;
const IO_NUM_PARAMS: usize = 1;
const OUTPUT_NUM_PARAMS: usize = 1;
const JUMP_NUM_PARAMS: usize = 2;
const CMP_NUM_PARAMS: usize = 3;

const WRITE_CMDS: [Command; 4] = [
    Command::ADD,
    Command::MULTIPLY,
    Command::LESS,
    Command::EQUALS,
];

#[derive(Debug)]
struct Opcode {
    command: Command,
    modes: Vec<u32>,
}
impl Opcode {
    pub fn new(opcode: i32) -> Self {
        let (command, modes) = match opcode % 100 {
            ADD => (Command::ADD, Opcode::get_modes(opcode, OPER_NUM_PARAMS)),
            MULTIPLY => (
                Command::MULTIPLY,
                Opcode::get_modes(opcode, OPER_NUM_PARAMS),
            ),
            INPUT => (Command::INPUT, vec![]),
            OUTPUT => (
                Command::OUTPUT,
                Opcode::get_modes(opcode, OUTPUT_NUM_PARAMS),
            ),
            JIT => (Command::JIT, Opcode::get_modes(opcode, JUMP_NUM_PARAMS)),
            JIF => (Command::JIF, Opcode::get_modes(opcode, JUMP_NUM_PARAMS)),
            LESS => (Command::LESS, Opcode::get_modes(opcode, CMP_NUM_PARAMS)),
            EQUALS => (Command::EQUALS, Opcode::get_modes(opcode, CMP_NUM_PARAMS)),
            STOP => (Command::STOP, vec![]),
            _ => panic!("Invalid opcode: {}", opcode),
        };
        Opcode { command, modes }
    }

    fn get_modes(opcode: i32, num_params: usize) -> Vec<u32> {
        let mut digits: Vec<u32> = opcode
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .collect();
        while digits.len() < num_params + LEN_COMMAND {
            digits.push(0);
        }
        digits[LEN_COMMAND..].to_vec()
    }
}

#[derive(Debug)]
pub struct Program {
    memory: RefCell<Vec<i32>>,
    ip: usize,
    inputs: RefCell<Vec<i32>>,
    input_index: usize,
}
impl Program {
    pub fn new(program_string: &str, inputs: &Vec<i32>) -> Self {
        let memory: Vec<i32> = program_string
            .split(",")
            .map(|val| val.parse().unwrap())
            .collect();
        Program {
            memory: RefCell::new(memory),
            ip: 0,
            inputs: RefCell::new(inputs.clone()),
            input_index: 0,
        }
    }

    pub fn memory(&self) -> Ref<Vec<i32>> {
        self.memory.borrow()
    }

    fn val_at(&self, index: usize) -> i32 {
        self.memory()[index as usize]
    }

    fn set(&self, index: usize, val: i32) {
        self.memory.borrow_mut()[index] = val;
    }

    pub fn send_input(&mut self, input: i32) {
        self.inputs.borrow_mut().push(input);
    }

    fn get_params(&self, opcode: &Opcode) -> Vec<i32> {
        opcode
            .modes
            .iter()
            .enumerate()
            .map(|(offset, &mode)| {
                if offset == opcode.modes.len() - 1 && WRITE_CMDS.contains(&(opcode.command)) {
                    return self.val_at(self.ip + offset + 1);
                }
                match mode {
                    POSITION => self.val_at(self.val_at(self.ip + offset + 1) as usize),
                    IMMEDIATE => self.val_at(self.ip + offset + 1),
                    _ => panic!("Invalid modes for opcode: {:?}", opcode),
                }
            })
            .collect()
    }

    pub fn run(&mut self) -> Option<i32> {
        let mut last_output = None;
        loop {
            let (exit, output) = self.execute();
            if exit {
                break;
            }
            if output.is_some() {
                last_output = output;
            }
        }
        last_output
    }

    pub fn execute(&mut self) -> (bool, Option<i32>) {
        let opcode = Opcode::new(self.val_at(self.ip));
        let params = self.get_params(&opcode);
        match opcode.command {
            Command::ADD => {
                self.set(params[2] as usize, params[0] + params[1]);
                self.ip += OPER_NUM_PARAMS + 1;
                (false, None)
            }
            Command::MULTIPLY => {
                self.set(params[2] as usize, params[0] * params[1]);
                self.ip += OPER_NUM_PARAMS + 1;
                (false, None)
            }
            Command::INPUT => {
                self.set(
                    self.val_at(self.ip + IO_NUM_PARAMS) as usize,
                    self.inputs.borrow()[self.input_index],
                );
                self.input_index += 1;
                self.ip += IO_NUM_PARAMS + 1;
                (false, None)
            }
            Command::OUTPUT => {
                self.ip += IO_NUM_PARAMS + 1;
                (false, Some(params[0]))
            }
            Command::JIT => {
                if params[0] != 0 {
                    self.ip = params[1] as usize;
                } else {
                    self.ip += JUMP_NUM_PARAMS + 1;
                }
                (false, None)
            }
            Command::JIF => {
                if params[0] == 0 {
                    self.ip = params[1] as usize;
                } else {
                    self.ip += JUMP_NUM_PARAMS + 1;
                }
                (false, None)
            }
            Command::LESS => {
                if params[0] < params[1] {
                    self.set(params[2] as usize, 1);
                } else {
                    self.set(params[2] as usize, 0);
                }
                self.ip += CMP_NUM_PARAMS + 1;
                (false, None)
            }
            Command::EQUALS => {
                if params[0] == params[1] {
                    self.set(params[2] as usize, 1);
                } else {
                    self.set(params[2] as usize, 0);
                }
                self.ip += CMP_NUM_PARAMS + 1;
                (false, None)
            }
            Command::STOP => (true, None),
        }
    }
}

pub fn process_program(program_string: &str, inputs: &Vec<i32>) -> (Program, Option<i32>) {
    let mut program = Program::new(program_string, inputs);
    let output = program.run();
    (program, output)
}
