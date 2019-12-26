use std::{char, iter};

fn num_to_digit(a: i32) -> i32 {
    a.to_string().chars().last().unwrap().to_digit(10).unwrap() as i32
}

fn slow_fft(input: &Vec<i32>) -> String {
    let mut cur = input.clone();
    let base = vec![0, 1, 0, -1];
    let half = if cur.len() % 2 == 0 {
        cur.len() / 2
    } else {
        cur.len() / 2 + 1
    };
    for i in 0..100 {
        if i % 5 == 0 {
            println!("On iteration {}", i);
        }
        let mut next = vec![];
        for j in 0..half {
            let pattern = base
                .iter()
                .flat_map(|d| iter::repeat(d).take(j + 1))
                .map(|d| *d)
                .cycle();
            let output = num_to_digit(
                cur.iter()
                    .zip(pattern.skip(1).take(cur.len()))
                    .map(|(a, b)| a * b)
                    .sum::<i32>(),
            );
            next.push(output);
        }
        // for j in cur.len() / 2 + 1..cur.len() {
        //     let output = num_to_digit(cur.iter().skip(j).sum());
        //     next.push(output);
        // }
        let mut sum = 0;
        let mut sums = vec![];
        for val in cur.iter().rev().take(cur.len() - half) {
            sum += val;
            sums.push(num_to_digit(sum));
        }
        next.extend(sums.iter().rev());
        cur = next;
    }
    cur.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect::<String>()
}

fn fast_fft(input: &Vec<i32>, offset: usize) -> String {
    let mut cur: Vec<i32> = input.clone()[offset..].into();

    for i in 0..100 {
        if i % 5 == 0 {
            println!("On iteration {}", i);
        }
        let mut sum = 0;
        let mut sums = vec![];
        for val in cur.iter().rev() {
            sum += val;
            sums.push(num_to_digit(sum));
        }
        cur = sums.iter().rev().cloned().collect();
    }
    cur.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect::<String>()
}

pub fn fft(input_str: &str, repeats: usize, offset: usize) -> String {
    let input = input_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .cycle()
        .take(input_str.len() * repeats)
        .collect::<Vec<i32>>();
    if offset < input.len() / 2 + 1 {
        slow_fft(&input)
    } else {
        fast_fft(&input, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod slow_fft {
        use super::*;

        // #[test]
        // fn test_simple() {
        //     let case = "12345678";
        //     fft(case, 1, 0);
        //     assert_eq!(0, 1);
        // }

        #[test]
        fn test_1() {
            let case = "80871224585914546619083218645595";
            assert_eq!(
                fft(case, 1, 0).chars().take(8).collect::<String>(),
                "24176176"
            );
        }

        #[test]
        fn test_2() {
            let case = "19617804207202209144916044189917";
            assert_eq!(
                fft(case, 1, 0).chars().take(8).collect::<String>(),
                "73745418"
            );
        }

        #[test]
        fn test_3() {
            let case = "69317163492948606335995924319873";
            assert_eq!(
                fft(case, 1, 0).chars().take(8).collect::<String>(),
                "52432133"
            );
        }
    }

    mod fast_fft {
        use super::*;

        #[test]
        fn test_1() {
            let case = "03036732577212944063491565474664";
            assert_eq!(
                fft(case, 10_000, 303673)
                    .chars()
                    .take(8)
                    .collect::<String>(),
                "84462026"
            );
        }

        #[test]
        fn test_2() {
            let case = "02935109699940807407585447034323";
            assert_eq!(
                fft(case, 10_000, 0293510)
                    .chars()
                    .take(8)
                    .collect::<String>(),
                "78725270"
            );
        }

        #[test]
        fn test_3() {
            let case = "03081770884921959731165446850517";
            assert_eq!(
                fft(case, 10_000, 0308177)
                    .chars()
                    .take(8)
                    .collect::<String>(),
                "53553731"
            );
        }
    }
}
