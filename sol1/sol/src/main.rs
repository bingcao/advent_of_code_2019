use std::fs;

fn total_fuel(mass: i64) -> i64 {
    let fuel = mass /3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + total_fuel(fuel)
    }
}

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let lines = file_string.trim().split("\n");
    let masses = lines.map(|line| line.trim().parse::<i64>().unwrap());
    let total_fuel: i64 = masses.map(|mass| total_fuel(mass)).sum();
    println!("{}", total_fuel);
}

