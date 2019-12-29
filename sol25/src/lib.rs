use intcode::Computer;

pub struct Droid {
    computer: Computer,
}

impl Droid {
    fn new(program_str: &str) -> Self {
        let computer = Computer::new(program_str);
        Droid {
            computer
        }
    }

    fn run_interactive(&mut self) {
        loop {
            self.computer.run_interactive();
            self.computer.reset();
        }
    }
}

pub fn run_droid(program_str: &str) -> Droid {
    let mut droid = Droid::new(program_str);
    droid.run_interactive();
    droid
}
