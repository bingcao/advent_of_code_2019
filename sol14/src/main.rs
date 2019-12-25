use starter::get_file_string;

mod lib;
use lib::get_ore_and_fuel;

fn main() {
    let fs = get_file_string();
    let result = get_ore_and_fuel(&fs[..]);
    println!(
        "Need {} ore for one fuel, 1 trillon ore produces {} fuel",
        result.0, result.1
    );
}
