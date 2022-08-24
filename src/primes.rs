extern crate num;

use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use num::FromPrimitive;
use num::Num;
use num::ToPrimitive;

#[derive(Clone)]
pub struct Primes<T> {
    pub vector: Vec<T>,
    set: HashSet<T>,
}

impl<T> Primes<T>
where
    T: Num + ToPrimitive + FromPrimitive + Hash + Eq + PartialEq + Copy + Display,
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
        let number_of_primes = array.iter().filter(|b| **b).count();
        let mut vector = Vec::with_capacity(number_of_primes);
        let mut set = HashSet::with_capacity(number_of_primes);

        for number in min_value..max_value {
            let idx = number - min_value;
            // SAFETY: see considerations in get_prime_numbers_array
            if unsafe { *array.get_unchecked(idx) } {
                let prime = FromPrimitive::from_usize(number).unwrap();
                vector.push(prime);
                set.insert(prime);
            }
        }

        debug_assert_eq!(number_of_primes, vector.len());
        Primes { vector, set }
    }

    fn get_prime_numbers_array(min_value: usize, max_value: usize) -> Vec<bool> {
        // Sieve of Eratosthenes
        let length = max_value - min_value;
        let mut result = vec![true; length];

        let upper_limit = Self::calc_upper_limit(max_value);
        for i in 2..upper_limit {
            let skip = Self::find_number_to_skip_until_min_value(min_value, i);

            for multiple in (i * i..max_value).step_by(i).skip(skip) {
                let idx = multiple - min_value;
                // SAFETY: `multiple` will always be between min_value and max_value
                // (which define the length of the array), therfore idx is always smaller than the
                // length of result, therefore this access is always in bounds.
                // This is slightly faster than the bounds-checked write.
                unsafe { *result.get_unchecked_mut(idx) = false };
            }
        }
        result
    }

    fn calc_upper_limit(max_value: usize) -> usize {
        f64::sqrt(max_value as f64).ceil() as usize
    }

    fn find_number_to_skip_until_min_value(min_value: usize, i: usize) -> usize {
        ((min_value.saturating_sub(i * i)) as f64 / i as f64).ceil() as usize
    }

    pub fn contains(&self, val: &T) -> bool {
        self.set.contains(val)
    }
}

#[cfg(test)]
mod test {
    use super::Primes;

    #[test]
    fn test_example_prime() {
        let primes = Primes::get_between(2, 99_999);
        assert!(primes.set.contains(&56003));
        assert!(primes.set.contains(&56993));
        assert!(!primes.set.contains(&56002));
    }
}
