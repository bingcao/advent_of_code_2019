use std::fs;
use std::time::{Instant};

mod lib;
use crate::lib::{find_closest_intersection_distance, find_steppiest_intersection_distance};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file_string.trim().split("\n").collect();
    let point_list_a: Vec<&str> = lines[0].split(",").collect();
    let point_list_b: Vec<&str> = lines[1].split(",").collect();

    let start = Instant::now();
    let result = find_closest_intersection_distance(&point_list_a, &point_list_b);
    let duration = start.elapsed();
    println!("closest_intersection: {}", result);
    println!("calculation took {:?}", duration);

    let start = Instant::now();
    let result = find_steppiest_intersection_distance(&point_list_a, &point_list_b);
    let duration = start.elapsed();
    println!("closest_intersection: {}", result);
    println!("calculation took {:?}", duration);
}
