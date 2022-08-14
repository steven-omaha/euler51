mod combination;
mod primes;

use combination::PositionCombinations;
use primes::Primes;

type Int = u64;

// EXAMPLE 1
// const MIN_VALUE: Int = 10;
// const MAX_VALUE: Int = 99;
// const LENGTH: usize = 2;
// const MIN_LENGTH: usize = 1;

// EXAMPLE 2
// const MIN_VALUE: Int = 10_000;
// const MAX_VALUE: Int = 99_999;
// const LENGTH: usize = 5;
// const MIN_LENGTH: usize = 2;

// FINAL
const MIN_VALUE: Int = 100_000;
const MAX_VALUE: Int = 999_999;
const LENGTH: usize = 6;
const MIN_LENGTH: usize = 2;

const MAX_LENGTH: usize = LENGTH - 1;
const DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let all_primes = Primes::get_between(MIN_VALUE, MAX_VALUE);
    let mut best_of_all_combinations = vec![];
    for pattern in PositionCombinations::new(MIN_LENGTH, MAX_LENGTH, LENGTH) {
        let mut best_this_combination = vec![];
        let mut primes_to_check = all_primes.clone();
        while let Some(first) = primes_to_check.pop() {
            let mut primes_matching_pattern = vec![];
            let digits = first.to_digits();
            for digit in DIGITS {
                let could_be_prime = apply_transformation(&digits, digit, &pattern);
                if primes_to_check.contains(&could_be_prime) || could_be_prime == first {
                    primes_matching_pattern.push(could_be_prime);
                }
            }
            if primes_matching_pattern.len() > 2 {
                best_this_combination.push(primes_matching_pattern);
            }
        }
        if !best_this_combination.is_empty() {
            best_of_all_combinations.push(get_longest(best_this_combination));
        }
    }
    println!("{:#?}", get_longest(best_of_all_combinations));
}

fn apply_transformation(prime_as_digits: &[u8], new_digit: u8, pattern: &[bool]) -> Int {
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

fn get_longest(mut vec: Vec<Vec<Int>>) -> Vec<Int> {
    assert!(
        !vec.is_empty(),
        "cannot get the longest element of an empty vector"
    );
    vec.sort_by_key(|c| c.len());
    let length = vec.last().unwrap().len();
    vec.into_iter().find(|v| v.len() == length).unwrap()
}

trait ToDigits {
    fn to_digits(&self) -> Vec<u8>;
}

impl ToDigits for Int {
    fn to_digits(&self) -> Vec<u8> {
        self.to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{apply_transformation, ToDigits};

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
            apply_transformation(&prime_as_digits, new_digit, &pattern),
            500_021
        );
    }
}
