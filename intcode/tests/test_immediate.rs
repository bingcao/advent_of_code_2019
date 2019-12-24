use intcode::process_program;
use std::collections::HashMap;

#[cfg(test)]
mod tests_immediate {
    use super::*;

    fn memory_to_vec(memory: &HashMap<usize, i128>) -> Vec<i128> {
        let mut vals = vec![];
        for i in 0..memory.len() {
            vals.push(*memory.get(&i).unwrap());
        }
        vals
    }

    #[test]
    fn test_1() {
        let program = "1002,4,3,4,33";
        let (result, _o) = process_program(program, &vec![]);
        assert_eq!(memory_to_vec(&result.memory()), vec![1002, 4, 3, 4, 99]);
    }
}
