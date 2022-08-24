mod combination;
mod primes;

use std::{collections::HashSet, process::exit};

use combination::PositionCombinations;
use primes::Primes;

type Int = u64;

// EXAMPLE 1
// const SEARCH_MIN: Int = 10;
// const SEARCH_MAX: Int = 99;
// const AMOUNT_OF_DIGITS_IN_NUMBER: usize = 2;
// const MIN_PATTERN_LENGTH: usize = 1;
// const FAMILY_SIZE: usize = 6;

// EXAMPLE 2
// const SEARCH_MIN: Int = 10_000;
// const SEARCH_MAX: Int = 99_999;
// const AMOUNT_OF_DIGITS_IN_NUMBER: usize = 5;
// const MIN_PATTERN_LENGTH: usize = 1;
// const FAMILY_SIZE: usize = 7;

// FINAL
const SEARCH_MIN: Int = 100_000;
const SEARCH_MAX: Int = 999_999;
const AMOUNT_OF_DIGITS_IN_NUMBER: usize = 6;
const MIN_PATTERN_LENGTH: usize = 2;
const FAMILY_SIZE: usize = 8;

const MAX_PATTERN_LENGTH: usize = AMOUNT_OF_DIGITS_IN_NUMBER - 1;
const NUMBER_OF_DIGITS: usize = 10;
const DIGITS: [u8; NUMBER_OF_DIGITS] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let all_primes = Primes::get_between(SEARCH_MIN, SEARCH_MAX);
    let mut processed_cases = HashSet::with_capacity(SEARCH_MIN as usize);
    let mut primes_matching_pattern = Vec::with_capacity(FAMILY_SIZE);
    let mut case_buf = String::with_capacity(AMOUNT_OF_DIGITS_IN_NUMBER);

    for pattern in PositionCombinations::new(
        MIN_PATTERN_LENGTH,
        MAX_PATTERN_LENGTH,
        AMOUNT_OF_DIGITS_IN_NUMBER,
    ) {
        for prime in &all_primes.vector {
            primes_matching_pattern.clear();

            let digits = prime.to_digits();

            get_case_string(&digits, &pattern, &mut case_buf);
            if processed_cases.contains(&case_buf) {
                continue;
            } else {
                processed_cases.insert(case_buf.clone());
            }

            for (i, digit) in DIGITS.iter().enumerate() {
                if !enough_digits_left(i, primes_matching_pattern.len()) {
                    break;
                }
                let could_be_prime =
                    replace_digits_with_new_digit_according_to_pattern(&digits, *digit, &pattern);
                if all_primes.contains(&could_be_prime) {
                    primes_matching_pattern.push(could_be_prime);
                }
            }
            if primes_matching_pattern.len() == FAMILY_SIZE {
                println!("{primes_matching_pattern:#?}");
                exit(0);
            }
        }
    }
}

fn enough_digits_left(i: usize, primes_len: usize) -> bool {
    // are there enough digits left in the loop to achieve the required FAMILY_SIZE?
    i - primes_len + FAMILY_SIZE <= NUMBER_OF_DIGITS
}

fn get_case_string(digits: &[u8], pattern: &[bool], buf: &mut String) {
    buf.clear();
    digits
        .iter()
        .zip(pattern)
        .map(|(d, b)| if *b { '*' } else { *d as char })
        .for_each(|c| buf.push(c));
}

fn replace_digits_with_new_digit_according_to_pattern(
    prime_as_digits: &[u8],
    new_digit: u8,
    pattern: &[bool],
) -> Int {
    debug_assert_eq!(prime_as_digits.len(), pattern.len());
    let mut multiplier = 1;
    let result = prime_as_digits
        // apply the pattern to prime_as_digits
        .iter()
        .zip(pattern)
        .map(|(old_digit, replace)| if *replace { new_digit } else { *old_digit })
        .map(|d| d as Int)
        // calculate Int
        .rev()
        .reduce(|accum, item| {
            multiplier *= 10;
            accum + multiplier * item
        })
        .unwrap();
    debug_assert_eq!(multiplier, 100_000);
    result
}

trait ToDigits {
    fn to_digits(&self) -> Vec<u8>;
}

impl ToDigits for Int {
    fn to_digits(&self) -> Vec<u8> {
        let oom = get_order_of_magnitude(*self);
        let length = oom as usize;
        let mut var = *self;
        let mut result = vec![0; length];

        result.iter_mut().enumerate().for_each(|(i, d)| {
            let power = 10_u64.pow(oom - i as u32 - 1);
            let digit = var / power;
            var -= digit * power;
            *d = digit as u8;
        });

        debug_assert_eq!(var, 0);
        result
    }
}

fn get_order_of_magnitude(x: Int) -> u32 {
    let mut tmp = 1;
    let mut oom = 1;
    while tmp - 1 < x {
        oom += 1;
        tmp *= 10;
    }
    oom - 1
}

#[cfg(test)]
mod test {
    use crate::{
        get_order_of_magnitude, replace_digits_with_new_digit_according_to_pattern, ToDigits,
    };

    #[test]
    fn test_num_to_digits() {
        let num = 83371;
        assert_eq!(num.to_digits(), vec![8, 3, 3, 7, 1]);
    }

    #[test]
    fn test_apply_transformation() {
        let prime_as_digits = [5, 7, 3, 8, 2, 1];
        let new_digit = 0;
        let pattern = [false, true, true, true, false, false];
        assert_eq!(
            replace_digits_with_new_digit_according_to_pattern(
                &prime_as_digits,
                new_digit,
                &pattern
            ),
            500_021
        );
    }

    #[test]
    fn test_to_digits() {
        let num = 37871_u64;
        assert_eq!(vec![3, 7, 8, 7, 1], num.to_digits());
    }

    #[test]
    fn test_oom() {
        assert_eq!(get_order_of_magnitude(3), 1);
        assert_eq!(get_order_of_magnitude(9), 1);
        assert_eq!(get_order_of_magnitude(10), 2);
        assert_eq!(get_order_of_magnitude(32), 2);
        assert_eq!(get_order_of_magnitude(10_000), 5);
        assert_eq!(get_order_of_magnitude(10_001), 5);
        assert_eq!(get_order_of_magnitude(37_871), 5);
    }
}
