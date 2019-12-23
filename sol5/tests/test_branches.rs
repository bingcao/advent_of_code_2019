use sol5::process_program;

#[cfg(test)]
mod tests_branches {
    use super::*;

    #[test]
    fn test_branches_1() {
        let program_str = "3,9,8,9,10,9,4,9,99,-1,8";
        let (_p, result) = process_program(program_str, &[8]);
        assert_eq!(result, Some(1), "program did not output 1 for 8");

        let (_p, result) = process_program(program_str, &[3]);
        assert_eq!(result, Some(0), "program did not output 0 for 3");
    }

    #[test]
    fn test_branches_2() {
        let program_str = "3,9,7,9,10,9,4,9,99,-1,8";
        let (_p, result) = process_program(program_str, &[8]);
        assert_eq!(result, Some(0), "program did not output 0 for 8");

        let (_p, result) = process_program(program_str, &[3]);
        assert_eq!(result, Some(1), "program did not output 1 for 3");
    }


    #[test]
    fn test_branches_3() {
        let program_str = "3,3,1108,-1,8,3,4,3,99";
        let (_p, result) = process_program(program_str, &[8]);
        assert_eq!(result, Some(1), "program did not output 1 for 8");

        let (_p, result) = process_program(program_str, &[3]);
        assert_eq!(result, Some(0), "program did not output 0 for 3");
    }

    #[test]
    fn test_branches_4() {
        let program_str = "3,3,1107,-1,8,3,4,3,99";
        let (_p, result) = process_program(program_str, &[8]);
        assert_eq!(result, Some(0), "program did not output 0 for 8");

        let (_p, result) = process_program(program_str, &[3]);
        assert_eq!(result, Some(1), "program did not output 1 for 3");
    }


    #[test]
    fn test_branches_5() {
        let program_str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let (_p, result) = process_program(program_str, &[0]);
        assert_eq!(result, Some(0), "program did not output 0 for 0");

        let (_p, result) = process_program(program_str, &[3]);
        assert_eq!(result, Some(1), "program did not output 1 for 3");
        let (_p, result) = process_program(program_str, &[-3]);
        assert_eq!(result, Some(1), "program did not output 1 for -3");
    }
}
