use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Failed to read file").split("\n");
    for line in lines {
        println!("{}", line);
    }
}
