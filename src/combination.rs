pub struct PositionCombinations {
    min_length: usize,
    max_length: usize,
    state: Vec<bool>,
    finished: bool,
}

impl Iterator for PositionCombinations {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = self.state.clone();
            self.increment();
            if self.finished {
                return None;
            }
            if (self.min_length..=self.max_length).contains(&result.iter().filter(|x| **x).count())
            {
                return Some(result);
            }
        }
    }
}

impl PositionCombinations {
    pub fn new(min_length: usize, max_length: usize, length: usize) -> Self {
        let state = vec![false; length];
        let finished = false;
        Self {
            min_length,
            max_length,
            state,
            finished,
        }
    }

    fn increment(&mut self) {
        let mut overflow = self.state[0];
        self.state[0] ^= true;
        for position in self.state[1..].iter_mut() {
            let new_overflow = *position & overflow;
            *position ^= overflow;
            overflow = new_overflow;
        }
        if overflow {
            self.finished = true;
        }
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Vec<bool> {
    fn to_string(&self) -> String {
        self.iter().map(|b| if *b { 'x' } else { '.' }).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::combination::PositionCombinations;
    use std::collections::HashSet;

    #[test]
    fn test_combination_2_3_4() {
        let combinator = PositionCombinations::new(2, 3, 4);
        let result: Vec<_> = combinator.into_iter().collect();
        let mut number_solutions = result.len();
        let mut solution = HashSet::new();
        solution.insert(vec![false, false, true, true]);
        solution.insert(vec![false, true, false, true]);
        solution.insert(vec![false, true, true, false]);
        solution.insert(vec![false, true, true, true]);
        solution.insert(vec![true, true, false, false]);
        solution.insert(vec![true, false, true, false]);
        solution.insert(vec![true, false, false, true]);
        solution.insert(vec![true, false, true, true]);
        solution.insert(vec![true, true, false, true]);
        solution.insert(vec![true, true, true, false]);
        for item in solution {
            assert!(result.contains(&item));
            number_solutions -= 1;
        }
        assert_eq!(number_solutions, 0);
    }

    #[test]
    fn test_combination_2_2_3() {
        let combinator = PositionCombinations::new(2, 2, 3);
        let result: Vec<_> = combinator.into_iter().collect();
        let mut number_solutions = result.len();
        let mut solution = HashSet::new();
        solution.insert(vec![false, true, true]);
        solution.insert(vec![true, false, true]);
        solution.insert(vec![true, true, false]);
        for item in solution {
            assert!(result.contains(&item));
            number_solutions -= 1;
        }
        assert_eq!(number_solutions, 0);
    }
}
