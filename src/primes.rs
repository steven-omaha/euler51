extern crate num;

use num::FromPrimitive;
use num::Num;
use num::ToPrimitive;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct Primes<T> {
    vector: Vec<T>,
    set: HashSet<T>,
}

impl<T> Primes<T>
where
    T: Num + ToPrimitive + FromPrimitive + Hash + Eq + PartialEq + Copy,
{
    pub fn get_between(min_value: T, max_value: T) -> Self {
        let max_value = max_value.to_usize().unwrap();
        let min_value = min_value.to_usize().unwrap();
        assert!(min_value > 1);
        assert!(max_value > min_value);
        Self::from_array(
            &Self::get_prime_numbers_array(min_value, max_value),
            min_value,
            max_value,
        )
    }

    fn from_array(array: &[bool], min_value: usize, max_value: usize) -> Self {
        let mut vector = Vec::new();
        let mut set = HashSet::new();
        for number in min_value..max_value {
            if array[(number - min_value) as usize] {
                let prime = FromPrimitive::from_usize(number).unwrap();
                vector.push(prime);
                set.insert(prime);
            }
        }
        vector.reverse(); // so that self.pop removes the smallest prime
        Primes { vector, set }
    }

    fn get_prime_numbers_array(min_value: usize, max_value: usize) -> Vec<bool> {
        // Sieve of Eratosthenes
        let length = max_value - min_value;
        let mut result = vec![true; length];

        for number_to_check in 2..(max_value / 2) {
            let mut last_number = number_to_check;
            loop {
                let current_number = last_number + number_to_check;
                if current_number < min_value {
                    last_number = current_number;
                    continue;
                }
                if current_number >= max_value {
                    break;
                }
                result[current_number - min_value] = false;
                last_number = current_number;
            }
        }
        result
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = self.vector.pop();
        if let Some(value) = result {
            self.set.remove(&value);
        }
        result
    }

    pub fn contains(&self, val: &T) -> bool {
        self.set.contains(val)
    }
}

#[cfg(test)]
mod test {
    use crate::primes::Primes;

    #[test]
    fn test_example_prime() {
        let primes = Primes::get_between(2, 99_999);
        assert!(primes.set.contains(&56003));
        assert!(primes.set.contains(&56993));
        assert!(!primes.set.contains(&56002));
    }

    #[test]
    fn test_pop() {
        let mut primes = Primes::get_between(10, 20);
        assert!(primes.contains(&11));
        primes.pop();
        assert!(!primes.contains(&11));
    }
}
