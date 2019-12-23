use std::fs;
use std::time::{Instant};

mod lib;
use lib::{calc_checksum, calc_transfers};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let file_string = file_string.trim();
    let lines: Vec<&str> = file_string.split("\n").collect();

    let start = Instant::now();
    println!("num orbits: {}", calc_checksum(&lines[..])); 
    let duration = start.elapsed();
    println!("calculation took {:?}", duration);

    let start = Instant::now();
    println!("num transfers: {}", calc_transfers(&lines[..])); 
    let duration = start.elapsed();
    println!("calculation took {:?}", duration);
}
