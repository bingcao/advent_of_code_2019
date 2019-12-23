use std::fmt;

struct Password {
    digits: Vec<u32>,
    last_digit: u32,
    has_dup: bool,
    dup_digit: Option<u32>,
    last_is_dup: bool,
}
impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.to_num(), self.has_dup)
    }
}

impl Password {
    fn new(digit: u32) -> Self {
        Password {digits: vec![digit], last_digit: digit, has_dup: false, dup_digit: None, last_is_dup: false}
    }

    fn add_digit(&self, digit: u32) -> Self {
        let mut new_digits = self.digits.clone();
        new_digits.extend(vec![digit]);

        let mut num_times = 0;
        let last_is_dup = self.last_digit == digit;
        if self.has_dup && self.dup_digit != Some(digit) {
            return Password {
                digits: new_digits, 
                last_digit: digit,
                has_dup: self.has_dup,
                dup_digit: self.dup_digit,
                last_is_dup
            }
        }

        let mut has_dup = self.has_dup || last_is_dup;
        let mut dup_digit = self.dup_digit;
        if digit == self.last_digit {
            num_times = self.digits.iter().filter(|&d| *d == digit).count()
        }
        if num_times > 1 {
            has_dup = false;
            if dup_digit == Some(digit) {
                dup_digit = None;
            }
        } else if num_times == 1 {
            dup_digit = self.dup_digit.or(Some(digit));
        }

        Password {
            digits: new_digits, 
            last_digit: digit,
            has_dup: has_dup,
            dup_digit: dup_digit,
            last_is_dup
        }
    }

    fn next_passwords(&self, on_end_digit: bool) -> Vec<Password> {
        if !on_end_digit {
            (self.last_digit..10).map(|digit| self.add_digit(digit)).collect()
        } else {
            if !self.has_dup {
                if !self.last_is_dup {
                    vec![self.add_digit(self.last_digit)]
                } else {
                    (self.last_digit + 1..10).map(|digit| self.add_digit(digit)).filter(|password| password.has_dup).collect()
                }
            } else if self.dup_digit == Some(self.last_digit) {
                (self.last_digit + 1..10).map(|digit| self.add_digit(digit)).filter(|password| password.has_dup).collect()
            } else {
                (self.last_digit..10).map(|digit| self.add_digit(digit)).collect()
            }
        }
        // if on_end_digit && !self.has_dup {
        //     let next = self.add_digit(self.last_digit);
        //     if !next.has_dup {
        //         vec![]
        //     } else {
        //         vec![next]
        //     }
        // } else if on_end_digit && !self.has_dup && self.last_is_dup && self.dup_digit == Some(self.last_digit) {
        //     (self.last_digit + 1..10).map(|digit| self.add_digit(digit)).filter(|password| password.has_dup).collect()
        // } else {
        //     (self.last_digit..10).map(|digit| self.add_digit(digit)).collect()
        // }
    }

    fn to_num(&self) -> u32 {
        self.digits.iter().rev().enumerate().fold(0, |acc, (index, digit)| acc + digit * 10u32.pow(index as u32)) 
    }
}

fn passes_conditions(num: u32) -> bool {
    let digits: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut prev_digit = digits.first().cloned().unwrap();
    let mut has_dup = false;
    let mut cur_digit_counter = 1;
    for &digit in digits[1..].iter() {
        if digit < prev_digit {
            return false;
        }
        if digit == prev_digit {
            cur_digit_counter += 1;
        } else {
            if cur_digit_counter == 2 {
                has_dup = true;
            }
            cur_digit_counter = 1;
        }
        prev_digit = digit;
    }
    if cur_digit_counter == 2 {
        has_dup = true;
    }
    has_dup
}

fn generate_passwords(num_digits: u32) -> Vec<u32> {
    let mut passwords: Vec<Password> = (1..10).map(|digit| Password::new(digit)).collect();

    for i in 2..num_digits+1 {
        let mut new_passwords = vec![];
        for password in &passwords {
            new_passwords.extend(password.next_passwords(i == num_digits));
        }
        passwords = new_passwords;
    }

    passwords.iter().map(|password| password.to_num()).collect()
}

fn generate_2(min: u32, max: u32) -> Vec<u32> {
    (min..max).filter(|&num| {
        passes_conditions(num)
    }).collect()
}


pub fn num_passwords(num_digits: u32, range_str: &str) -> usize {
    let range: Vec<&str> = range_str.split("-").collect();
    let min_num: u32 = range[0].parse().unwrap();
    let max_num: u32 = range[1].parse().unwrap();
    
    let candidates = generate_passwords(num_digits);
    let candidates: Vec<&u32> = candidates.iter().filter(|&num| {
        *num >= min_num && *num <= max_num
    }).collect();
    candidates.len()
    // generate_2(min_num, max_num).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_add_password() {
        let password = Password::new(1);
        let result: Vec<u32> = password.next_passwords(false).iter().map(|pwd| pwd.to_num()).collect();
        let range: Vec<u32> = (11..20).collect();
        assert_eq!(result, range);
    }

    #[test]
    fn test_password_add_password_last_digit_no_dup() {
        let password = Password::new(1);
        let result: Vec<u32> = password.next_passwords(true).iter().map(|pwd| pwd.to_num()).collect();
        assert_eq!(result, vec![11]);
    }

    #[test]
    fn test_password_num_passwords() {
        let result = num_passwords(3, "200-210");
        assert_eq!(result, 0, "for {}, {}, {}", 3, 200, 210);

        let result = num_passwords(3, "200-239");
        assert_eq!(result, 8, "for {}, {}, {}", 3, 200, 239);


        let result = num_passwords(3, "990-1000");
        assert_eq!(result, 0, "for {}, {}, {}", 3, 990, 1000);
    }

    #[test]
    fn test_generates() {
        let result1 = generate_passwords(6);
        let result2 = generate_2(100000, 1000000);
        for p in &result1 {
            assert_eq!(true, result2.contains(&p), "second way missing {}", p);
        }
        for p in &result2 {
            assert_eq!(true, result1.contains(&p), "first way missing {}", p);
        }
    }


    #[test]
    fn test_passes_conditions() {
        assert_eq!(true, passes_conditions(112233), "failed for 112233");
        assert_eq!(false, passes_conditions(111123), "failed for 111123");
        assert_eq!(true, passes_conditions(111122), "failed for 111122");
    }
}

