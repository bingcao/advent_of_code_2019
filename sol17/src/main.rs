use starter::get_file_string;

mod lib;
use lib::{clean_robot, find_alignments, run_robot};

fn main() {
    let fs = get_file_string();
    let robot = run_robot(&fs[..]);
    println!("{}", robot);
    println!("{}", find_alignments(&robot.camera).iter().sum::<usize>());

    println!("==========================================================");
    println!("Now cleaning!");

    let result = clean_robot(&fs[..]);
    println!("{}", result);
}
