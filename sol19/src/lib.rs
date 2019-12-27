use intcode::Program;
use std::collections::HashMap;
use std::iter;

fn set_char_at(chars: &mut Vec<Vec<char>>, x: usize, y: usize, val: char) {
    chars[y + 1][x + 1] = val;
}

fn get_output(
    x: usize,
    y: usize,
    program_str: &str,
    cache: &mut HashMap<(usize, usize), i128>,
) -> i128 {
    if let Some(o) = cache.get(&(x, y)) {
        *o
    } else {
        let mut program = Program::new(program_str, &vec![x as i128, y as i128]);
        let o = program.run_until_blocked_or_done().0[0];
        cache.insert((x, y), o);
        o
    }
}

pub fn run_tractor(program_str: &str) -> usize {
    let mut count = 0;
    let mut chars = vec![];

    const MAX_X: usize = 100;
    const MAX_Y: usize = 100;
    for y in 0..MAX_Y + 2 {
        let mut row = vec![];
        if y == 0 || y == MAX_Y + 1 {
            row.extend(iter::repeat('-').take(MAX_X + 2));
        } else {
            row.push('|');
            row.extend(iter::repeat('.').take(MAX_X));
            row.push('|');
        }
        chars.push(row);
    }

    let mut x = 0;
    let mut y = 0;
    let mut cache = HashMap::new();
    while get_output(x + 99, y, program_str, &mut cache) != 1 {
        y += 1;
        while get_output(x, y + 99, program_str, &mut cache) != 1 {
            x += 1;
        }
    }
    return count;
}
