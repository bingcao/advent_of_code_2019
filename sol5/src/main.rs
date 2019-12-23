use std::fs;

use intcode::process_program;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let file_string = file_string.trim();

    process_program(file_string, &[5]); 
}
