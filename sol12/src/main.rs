use starter::get_file_string;
mod lib;
use lib::{find_reset, run_galaxy};

fn main() {
    let fs = get_file_string();
    let energy = run_galaxy(&fs[..], 100);
    println!("Total energy: {}", energy);

    let reset = find_reset(&fs[..]);
    println!("Reset step: {}", reset);
}
