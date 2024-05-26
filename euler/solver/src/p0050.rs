use std::collections::VecDeque;
use itertools::Itertools;

use number_lists::prime_iterator;

pub fn p0050_solver() -> String {
    // todo slow solution
    consecutive_prime_sum(1_000_000)
        .sorted_by_key(|(_, v)| 1_000_000 - v.len())
        .next()
        .unwrap()
        .0
        .to_string()
}

fn consecutive_prime_sum(max: usize) -> impl Iterator<Item=(usize, Vec<usize>)> {
    ConsecutivePrimeSumIterator::new(max)
}

struct ConsecutivePrimeSumIterator {
    primes: Vec<usize>,
    current_index: usize,
}

impl ConsecutivePrimeSumIterator {
    fn new(max: usize) -> Self {
        Self { primes: prime_iterator().take_while(|&p| p < max).collect(), current_index: 0 }
    }
}

impl Iterator for ConsecutivePrimeSumIterator {
    type Item = (usize, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.primes.len() - 1 {
            self.current_index += 1;
            let current = self.primes[self.current_index];
            let mut sum = 0;
            let mut summed_primes = VecDeque::new();
            let mut hi = 0;
            while sum != current {
                while sum < current {
                    sum += self.primes[hi];
                    summed_primes.push_back(self.primes[hi]);
                    hi += 1;
                }
                while sum > current {
                    sum -= summed_primes.pop_front().unwrap();
                }
            }
            if summed_primes.len() > 1 {
                return Some((current, summed_primes.into_iter().collect()));
            }
        }
        None
    }
}

#[test]
fn finds_primes_which_are_sums_of_consecutive_primes() {
    let prime_41 = consecutive_prime_sum(100).find(|&(n, _)| n == 41).unwrap().1;
    assert_eq!(prime_41, vec!(2, 3, 5, 7, 11, 13));
    let prime_953 = consecutive_prime_sum(1000).find(|&(n, _)| n == 953).unwrap().1;
    assert_eq!(prime_953.len(), 21);
}