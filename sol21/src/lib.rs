use intcode::Computer;

pub struct Droid {
    computer: Computer,
}

impl Droid {
    fn new(program_str: &str) -> Self {
        let computer = Computer::new(program_str);
        Droid {
            computer
        }
    }


    fn run_interactive(&mut self) {
        loop {
            self.computer.run_interactive();
            self.computer.reset();
        }
    }
    // Can jump if is ground 4 tiles away, aka D = 1
    // Don't jump if unneccesary, aka A..D=1
    // T is F, A..D=ground
    // OR A T
    // AND B T
    // AND C T
    // AND D T -> T is True if A..D are ground
    // NOT T J -> jump if not A..D are ground
    // AND D J -> jump it not A..D are ground and D is ground

    // Jump if can jump and then make a move:
    // OR E T
    // OR H T
    // AND D T
    // OR T J -> two cases: if can jump, T is true, otherwise false
    // want: T true if one tile is pit, false otherwise -> AND T J
    // NOT T T

    // When I can jump: T is now false
    // OR A T -> T is true if A is ground
    // AND B..D T -> T is if A..D is ground
    // NOT T T -> T is ~(A..D is ground) = one of them is pit
    // AND T J -> J is (can jump && one of A..D is pit)

    // When I can't jump: T is true
    // OR A T -> true
    // AND B..D T -> T is if B..D is ground
    // NOT T T -> T is ~(B..D) is grouond = one of them is pit
    // AND T J -> J is (can jump && one of B..D is pit)

    // NOT A T -> T is A is pit
    // OR T J -> J is (can jump && one of B..D is pit) or A is pit


    // OR A T -> if can jump, T is ground, if can't jump T stays true
    // AND B T... -> if can jump, T says all tiles are ground , if can't jump
    // Up to this point, program knows it is safe to jump and there is an upcoming hole
    // What about waiting for last possible hole?
    // J is false if can move and one upcoming spot has jumpable spots
    // or J is true if can't move or no upcoming spots are jumpable
    // can move encoded by OR A X, X is false
    // can't move by NOT A X
    // upcoming jumpable if AND i, i+4 -> AND {B,C,D} X, AND {F,G,H} X
//    B = true, F = true -> false
//    NOT B X -> false
//    ADN F X -> false
//    B = false, F = true -> false
//    NOT B X -> true
//    AND F X -> true
    // upcoming not jumpable if OR !i !i+4 -> NOT B X
    // AND B T
    // AND F T
    // If next spot is hole, jump
    // Or jump if 4 spots away is safe and 5 isn't
    // NOT A T -> T
    // AND H T -> T says if D is safe to jump
    // NOT T J -> J says don't jump because D is safe
}

pub fn run_droid(program_str: &str) -> Droid {
    let mut droid = Droid::new(program_str);
    droid.run_interactive();
    droid
}
