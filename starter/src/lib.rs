use std::fs;

pub fn get_file_string() -> String {
    match fs::read_to_string("input.txt") {
        Ok(s) => String::from(s.trim()),
        _ => panic!("Make sure to paste in input!"),
    }
}
