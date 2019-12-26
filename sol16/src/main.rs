use starter::get_file_string;
use std::time::Instant;

mod lib;
use lib::fft;

fn main() {
    let fs = get_file_string();
    let start = Instant::now();
    let result = fft(&fs[..], 1, 0);
    println!("Part 1: {:?}", result);
    let duration = start.elapsed();
    println!("Took {:?}", duration);

    let offset = fs
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let result = fft(&fs[..], 10000, offset);
    println!("Part 2: {:?}", result);
    let duration = start.elapsed();
    println!("Took {:?}", duration);
}
