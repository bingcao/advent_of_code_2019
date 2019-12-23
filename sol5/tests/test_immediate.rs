use sol5::process_program;

#[cfg(test)]
mod tests_immediate {
    use super::*;

    #[test]
    fn test_1() {
        let program = "1002,4,3,4,33";
        let (result, _o) = process_program(program, &[]);
        assert_eq!(*result.memory(), vec![1002, 4, 3, 4, 99]);
    }
}

