use std::fs;

mod lib;
use lib::get_image;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let file_string = file_string.trim();

    let image = get_image(file_string, 25, 6);
    let least_layer = image.layer_with_least('0');
    println!(
        "{}",
        least_layer.num_digits('1') * least_layer.num_digits('2')
    );

    println!("{:?}", image);
    println!("{}", image.decode());

    let file_string = "201222221022210021222012102102011010";
    let image = get_image(file_string, 4, 3);
    let least_layer = image.layer_with_least('0');
    println!(
        "{}",
        least_layer.num_digits('1') * least_layer.num_digits('2')
    );

    println!("{:?}", image);
    println!("{}", image.decode());
}
