use starter::get_file_string;

mod lib;
use lib::run_game;

fn main() {
    let fs = get_file_string();
    let game = run_game(&fs[..]);
    println!("Num blocks left: {}", game.num_blocks());
    println!("Final score: {}", game.score);
}
