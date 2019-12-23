use std::fs;

fn process_program(program: &mut [u32]) {
    let mut index = 0;
    loop {
        if program[index] == 99 {
            break
        } else if program[index] == 1 {
            program[program[index + 3] as usize] = program[program[index + 1] as usize] + program[program[index + 2] as usize];
            index += 4;
        } else if program[index] == 2 {
            program[program[index + 3] as usize] = program[program[index + 1] as usize] * program[program[index + 2] as usize];
            index += 4;
        } else {
            panic!("Found invalid opcode: {} at index {}", program[index], index);
        }
    }
}

fn main() {
    // let mut program1 = vec![1,0,0,0,99];
    // let mut program2 = vec![2,3,0,3,99];
    // let mut program3 = vec![2,4,4,5,99,0];
    // let mut program4 = vec![1,1,1,4,99,5,6,0,99];
    // process_program(&mut program1); 
    // process_program(&mut program2); 
    // process_program(&mut program3); 
    // process_program(&mut program4); 
    // println!("{:?}", program1);
    // println!("{:?}", program2);
    // println!("{:?}", program3);
    // println!("{:?}", program4);

    let file_string = fs::read_to_string("input.txt").unwrap();
    let file_string = file_string.trim();

    let original_program: Vec<u32> = file_string.split(",").map(|val| val.parse().unwrap()).collect();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = original_program.clone();
            program[1] = noun;
            program[2] = verb;

            process_program(&mut program); 
            if program[0] == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
            }
        }
    }
}
