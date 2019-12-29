use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::env;
use std::io;

const ADD: i128 = 1;
const MULTIPLY: i128 = 2;
const INPUT: i128 = 3;
const OUTPUT: i128 = 4;
const JIT: i128 = 5;
const JIF: i128 = 6;
const LESS: i128 = 7;
const EQUALS: i128 = 8;
const REL: i128 = 9;
const STOP: i128 = 99;

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
    REL,
    STOP,
}

const POSITION: u32 = 0;
const IMMEDIATE: u32 = 1;
const RELATIVE: u32 = 2;

const LEN_COMMAND: usize = 2;
const OPER_NUM_PARAMS: usize = 3;
const IO_NUM_PARAMS: usize = 1;
const JUMP_NUM_PARAMS: usize = 2;
const CMP_NUM_PARAMS: usize = 3;
const REL_NUM_PARAMS: usize = 1;

const WRITE_CMDS: [Command; 5] = [
    Command::ADD,
    Command::MULTIPLY,
    Command::INPUT,
    Command::LESS,
    Command::EQUALS,
];

#[derive(Debug)]
struct Opcode {
    command: Command,
    modes: Vec<u32>,
}
impl Opcode {
    pub fn new(opcode: i128) -> Self {
        let (command, modes) = match opcode % 100 {
            ADD => (Command::ADD, Opcode::get_modes(opcode, OPER_NUM_PARAMS)),
            MULTIPLY => (
                Command::MULTIPLY,
                Opcode::get_modes(opcode, OPER_NUM_PARAMS),
            ),
            INPUT => (Command::INPUT, Opcode::get_modes(opcode, IO_NUM_PARAMS)),
            OUTPUT => (Command::OUTPUT, Opcode::get_modes(opcode, IO_NUM_PARAMS)),
            JIT => (Command::JIT, Opcode::get_modes(opcode, JUMP_NUM_PARAMS)),
            JIF => (Command::JIF, Opcode::get_modes(opcode, JUMP_NUM_PARAMS)),
            LESS => (Command::LESS, Opcode::get_modes(opcode, CMP_NUM_PARAMS)),
            EQUALS => (Command::EQUALS, Opcode::get_modes(opcode, CMP_NUM_PARAMS)),
            REL => (Command::REL, Opcode::get_modes(opcode, REL_NUM_PARAMS)),
            STOP => (Command::STOP, vec![]),
            _ => panic!("Invalid opcode: {}", opcode),
        };
        Opcode { command, modes }
    }

    fn get_modes(opcode: i128, num_params: usize) -> Vec<u32> {
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

    fn to_command_str(&self, params: &Vec<i128>) -> String {
        match self.command {
            Command::ADD => format!("Storing {} to address {}", params[0] + params[1], params[2]),
            Command::MULTIPLY => {
                format!("Storing {} to address {}", params[0] * params[1], params[2])
            }
            Command::INPUT => format!("Storing input to address {}", params[0]),
            Command::OUTPUT => format!("Outputting {}", params[0]),
            Command::JIT => {
                if params[0] != 0 {
                    format!("Jumping to instruction at address {}", params[1])
                } else {
                    format!("Not jumping")
                }
            }
            Command::JIF => {
                if params[0] == 0 {
                    format!("Jumping to instruction at address {}", params[1])
                } else {
                    format!("Not jumping")
                }
            }
            Command::LESS => {
                if params[0] < params[1] {
                    format!("Storing 1 to address {}", params[2])
                } else {
                    format!("Storing 0 to address {}", params[2])
                }
            }
            Command::EQUALS => {
                if params[0] == params[1] {
                    format!("Storing 1 to address {}", params[2])
                } else {
                    format!("Storing 0 to address {}", params[2])
                }
            }
            Command::REL => format!("Increasing relative base by {}", params[0]),
            Command::STOP => format!("Stopping"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    memory: RefCell<HashMap<usize, i128>>,
    ip: usize,
    inputs: RefCell<Vec<i128>>,
    input_index: usize,
    relative_base: usize,
    debug: bool,
}
impl Program {
    pub fn new(program_string: &str, inputs: &Vec<i128>) -> Self {
        let memory = program_string
            .split(",")
            .enumerate()
            .map(|(i, val)| (i, val.parse().unwrap()))
            .collect::<HashMap<usize, i128>>();
        Program {
            memory: RefCell::new(memory),
            ip: 0,
            inputs: RefCell::new(inputs.clone()),
            input_index: 0,
            relative_base: 0,
            debug: env::var_os("DEBUG").is_some(),
        }
    }

    pub fn memory(&self) -> Ref<HashMap<usize, i128>> {
        self.memory.borrow()
    }

    fn val_at(&self, index: usize) -> i128 {
        match self.memory().get(&index) {
            Some(&val) => val,
            None => 0,
        }
    }

    pub fn set(&self, index: usize, val: i128) {
        self.memory.borrow_mut().insert(index, val);
    }

    pub fn send_input(&mut self, input: i128) {
        self.inputs.borrow_mut().push(input);
    }

    fn next_opcode(&self) -> Opcode {
        Opcode::new(self.val_at(self.ip))
    }

    fn get_params(&self, opcode: &Opcode) -> Vec<i128> {
        opcode
            .modes
            .iter()
            .enumerate()
            .map(|(offset, &mode)| {
                let is_write =
                    offset == opcode.modes.len() - 1 && WRITE_CMDS.contains(&(opcode.command));
                match mode {
                    POSITION => {
                        if is_write {
                            self.val_at(self.ip + offset + 1)
                        } else {
                            self.val_at(self.val_at(self.ip + offset + 1) as usize)
                        }
                    }
                    IMMEDIATE => self.val_at(self.ip + offset + 1),
                    RELATIVE => {
                        if is_write {
                            self.val_at(self.ip + offset + 1) + self.relative_base as i128
                        } else {
                            self.val_at(
                                (self.val_at(self.ip + offset + 1) + self.relative_base as i128)
                                    as usize,
                            )
                        }
                    }
                    _ => panic!("Invalid modes for opcode: {:?}", opcode),
                }
            })
            .collect()
    }

    pub fn num_inputs(&self) -> usize {
        self.inputs.borrow().len() - self.input_index
    }

    pub fn needs_input(&self) -> bool {
         self.next_opcode().command == Command::INPUT
    }

    pub fn run_until_blocked_or_done(&mut self) -> (Vec<i128>, bool) {
        let mut outputs = vec![];
        let mut done = false;
        loop {
            if self.needs_input() && self.num_inputs() == 0
            {
                break;
            }

            let (exit, output) = self.execute();
            if exit {
                done = true;
                break;
            }
            if output.is_some() {
                outputs.push(output.unwrap());
            }
        }
        (outputs, done)
    }

    pub fn run(&mut self) -> Vec<i128> {
        let mut outputs = vec![];
        loop {
            let (exit, output) = self.execute();
            if exit {
                break;
            }
            if output.is_some() {
                outputs.push(output.unwrap());
            }
        }
        outputs
    }

    pub fn execute(&mut self) -> (bool, Option<i128>) {
        let opcode = self.next_opcode();
        let params = self.get_params(&opcode);
        if self.debug {
            println!(
                "{}: Running command {:?} with params {:?}",
                self.ip, opcode.command, params
            );
            println!("\t{}", opcode.to_command_str(&params));
        }
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
                if self.input_index >= self.inputs.borrow().len() {
                    panic!("Need an input but could not find one");
                }
                if self.debug {
                    println!("\tInput is {}", self.inputs.borrow()[self.input_index]);
                }
                self.set(params[0] as usize, self.inputs.borrow()[self.input_index]);
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
            Command::REL => {
                self.relative_base = (self.relative_base as i128 + params[0]) as usize;
                if self.debug {
                    println!("\tRel base set to {}", self.relative_base);
                }
                self.ip += REL_NUM_PARAMS + 1;
                (false, None)
            }
            Command::STOP => (true, None),
        }
    }
}

pub fn process_program(program_string: &str, inputs: &Vec<i128>) -> (Program, Vec<i128>) {
    let mut program = Program::new(program_string, inputs);
    let outputs = program.run();
    (program, outputs)
}

pub struct Computer {
    program: Program,
    saved_program_str: String,
    pub saved_output: Option<i128>,
}
impl Computer {
    pub fn new(program_str: &str) -> Self {
        Computer {
            program: Program::new(program_str, &vec![]),
            saved_program_str: String::from(program_str),
            saved_output: None
        }
    }

    pub fn reset(&mut self) {
        self.program = Program::new(&self.saved_program_str[..], &vec![]);
    }

    pub fn send_ascii(&mut self, ascii: &str) {
        ascii.chars().for_each(|c| self.program.send_input(c as i128))
    }

    pub fn run_until_blocked_or_done(&mut self) -> (String, bool) {
        let (mut outputs, done) = self.program.run_until_blocked_or_done();
        let num_outputs = outputs.len();
        if outputs.len() > 0 && outputs[outputs.len() - 1] > u8::max_value() as i128 {
            self.saved_output = Some(outputs[num_outputs - 1]);
            outputs = outputs.into_iter().take(num_outputs - 1).collect();
        }
        let output_str = outputs.into_iter().map(|i| i as u8 as char).collect::<String>();
        print!("{}", output_str);
        (output_str, done)
    }

    pub fn run_interactive(&mut self) {
        loop {
            let (display_str, done) = self.run_until_blocked_or_done();
            if let Some(val) = self.saved_output {
                println!("Output is {}", val)
            }
            if done {
                break;
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            self.send_ascii(&input[..]);
        }
    }
}
