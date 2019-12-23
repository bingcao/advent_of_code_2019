use std::fs;

use sol6::{calc_checksum, calc_transfers};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let file_string = fs::read_to_string("test.txt").unwrap();
        let file_string = file_string.trim();
        let lines: Vec<&str> = file_string.split("\n").collect();

        let result = calc_checksum(&lines[..]); 
        assert_eq!(result, 54);
    }

    #[test]
    fn test_2() {
        let file_string = fs::read_to_string("test.txt").unwrap();
        let file_string = file_string.trim();
        let lines: Vec<&str> = file_string.split("\n").collect();

        let result = calc_transfers(&lines[..]); 
        assert_eq!(result, 4);
    }

}

