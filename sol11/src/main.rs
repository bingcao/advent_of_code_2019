use starter::get_file_string;

mod lib;
use lib::paint;

fn main() {
    let fs = get_file_string();
    let robot = paint(fs);

    println!("{:?}", robot.num_painted());
    println!("{}", robot);
}
