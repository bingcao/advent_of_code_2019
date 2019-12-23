use std::fs;
use std::time::{Instant};

mod lib;
use lib::num_passwords;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let file_string = file_string.trim();

    let start = Instant::now();
    let result = num_passwords(6, file_string);
    let duration = start.elapsed();

    println!("{}", result);
    println!("calculation took {:?}", duration);
}
