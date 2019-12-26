use starter::get_file_string;

mod lib;
use lib::run_droid;

fn main() {
    let fs = get_file_string();
    let mut droid = run_droid(&fs[..]);
    println!("{}", droid);
    println!(
        "Steps to dest: {}",
        droid.paths.get(&droid.get_dest_location()).unwrap().len()
    );
    println!("===========================================================================");
    println!("Filling with oxygen");
    let steps = droid.fill_oxygen();
    println!("Filling took {} steps", steps);
}
