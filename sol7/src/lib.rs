use intcode::Program;
use permutohedron::Heap;
use std::cell::RefCell;

const NUM_AMPS: usize = 5;

fn get_signal_for_inputs(program_str: &str, inputs: &[i32; 5]) -> i32 {
    let programs: Vec<RefCell<Program>> = (0..NUM_AMPS)
        .map(|i| RefCell::new(Program::new(program_str, &vec![inputs[i]])))
        .collect();
    let mut program_index = 0;
    let mut output = 0;
    let mut halted = vec![];

    loop {
        let mut cur_program = programs[program_index].borrow_mut();
        if halted.contains(&program_index) {
            program_index = (program_index + 1) % NUM_AMPS;
            continue;
        }

        cur_program.send_input(output);

        loop {
            let (halt, new_output) = cur_program.execute();
            if halt && program_index == NUM_AMPS - 1 {
                return output;
            } else if halt {
                halted.push(program_index);
                break;
            } else if new_output.is_some() {
                output = new_output.unwrap();
                break;
            }
        }

        program_index = (program_index + 1) % NUM_AMPS;
    }
}

pub fn get_signal(program_str: &str) -> i32 {
    let mut inputs = [0; NUM_AMPS];
    for i in 0..NUM_AMPS {
        inputs[i] = (i + 5) as i32
    }
    let permutations = Heap::new(&mut inputs);
    permutations
        .map(|permutation| get_signal_for_inputs(program_str, &permutation))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_get_signal_for_inputs {
        use super::*;

        // For Part 1
        // #[test]
        // fn test_1() {
        //     let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        //     let output = get_signal_for_inputs(program, &[4, 3, 2, 1, 0]);
        //     assert_eq!(output, 43210);
        // }

        // #[test]
        // fn test_2() {
        //     let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        //     let output = get_signal_for_inputs(program, &[0, 1, 2, 3, 4]);
        //     assert_eq!(output, 54321);
        // }

        // For Part 2
        #[test]
        fn test_1() {
            let program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
            let output = get_signal_for_inputs(program, &[9, 8, 7, 6, 5]);
            assert_eq!(output, 139629729);
        }

        #[test]
        fn test_2() {
            let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
            let output = get_signal_for_inputs(program, &[9, 7, 8, 5, 6]);
            assert_eq!(output, 18216);
        }
    }

    mod test_get_signal {
        use super::*;

        // #[test]
        // fn test_1() {
        //     let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        //     let output = get_signal(program);
        //     assert_eq!(output, 43210);
        // }

        // #[test]
        // fn test_2() {
        //     let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        //     let output = get_signal(program);
        //     assert_eq!(output, 54321);
        // }

        // For Part 2
        #[test]
        fn test_1() {
            let program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
            let output = get_signal(program);
            assert_eq!(output, 139629729);
        }

        #[test]
        fn test_2() {
            let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
            let output = get_signal(program);
            assert_eq!(output, 18216);
        }
    }
}
