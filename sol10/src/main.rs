use starter::get_file_string;

mod lib;
use lib::{get_best_location, get_vape_order};

fn main() {
    let file_string = get_file_string();
    let best = get_best_location(&file_string);
    println!("{:?}", best);

    let (best_loc, _) = best;
    let order = get_vape_order(&file_string, best_loc);
    println!("{:?}", order[199]);
}
