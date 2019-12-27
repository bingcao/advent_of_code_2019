use starter::get_file_string;

mod lib;
use lib::run_tractor;

fn main() {
    let fs = get_file_string();
    let result = run_tractor(&fs[..]);
    println!("{}", result);
}
