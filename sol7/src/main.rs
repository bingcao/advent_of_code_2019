use std::fs;

mod lib;
use lib::get_signal;

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input_string = input_string.trim();

    let result = get_signal(&input_string);
    println!("{:?}", result);
}
