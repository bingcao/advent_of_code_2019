use starter::get_file_string;

mod lib;
use lib::get_keys;

fn main() {
    let fs = get_file_string();
    let result = get_keys(&fs[..]);
    println!("Took {} steps", result);
}
