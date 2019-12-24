use intcode::process_program;
use std::fs;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let file_string = file_string.trim();

    let (_p, o) = process_program(file_string, &vec![1]);
    println!("{:?}", o);

    let (_p, o) = process_program(file_string, &vec![2]);
    println!("{:?}", o);
}
