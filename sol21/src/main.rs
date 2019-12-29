use starter::get_file_string;

mod lib;
use lib::run_droid;

fn main() {
    let fs = get_file_string();
    run_droid(&fs[..]);
}
