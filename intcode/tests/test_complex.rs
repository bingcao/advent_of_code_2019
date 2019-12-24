use intcode::process_program;

#[cfg(test)]
mod tests_complex {
    use super::*;

    #[test]
    fn test_1() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let (_p, o) = process_program(program, &vec![]);
        assert_eq!(
            o,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_2() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let (_p, o) = process_program(program, &vec![]);
        assert_eq!(o[0].to_string().chars().count(), 16);
    }

    #[test]
    fn test_3() {
        let program = "104,1125899906842624,99";
        let (_p, o) = process_program(program, &vec![]);
        assert_eq!(o[0], 1125899906842624);
    }
}
