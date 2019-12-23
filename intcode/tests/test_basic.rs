use intcode::process_program;

#[cfg(test)]
mod tests_basic {
    use super::*;

    #[test]
    fn test_basic_1() {
        let program = "1,0,0,0,99";
        let (result, _o) = process_program(program, &vec![]);
        assert_eq!(*result.memory(), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_basic_2() {
        let program = "2,3,0,3,99";
        let (result, _o) = process_program(program, &vec![]);
        assert_eq!(*result.memory(), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_basic_3() {
        let program = "2,4,4,5,99,0";
        let (result, _o) = process_program(program, &vec![]);
        assert_eq!(*result.memory(), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_basic_4() {
        let program = "1,1,1,4,2,5,6,0,99";
        let (result, _o) = process_program(program, &vec![]);
        assert_eq!(*result.memory(), vec![30,1,1,4,2,5,6,0,99]);
    }
}

