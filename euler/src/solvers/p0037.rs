use crate::libs::integers::digits::digitable::Digitable;
use crate::libs::integers::primes::{prime_iterator, PrimeIterator};

pub fn p0037_solver() -> String {
    truncatable_primes().sum::<u64>().to_string()
}

fn truncatable_primes() -> impl Iterator<Item=u64> {
    TruncatablePrimeIterator::new()
}

struct TruncatablePrimeIterator {
    primes: PrimeIterator<u64>,
    count: usize,
}

impl TruncatablePrimeIterator {
    fn new() -> Self {
        Self {
            primes: prime_iterator(),
            count: 0,
        }
    }
}

impl Iterator for TruncatablePrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 11 { return None }
        while let Some(p) = self.primes.next() {
            if p < 10 { continue; }
            let pd = p.as_decimal();
            if pd.truncate_left().all(|p2| self.primes.is_prime(p2.number())) &&
                pd.truncate_right().all(|p2| self.primes.is_prime(p2.number())) {
                self.count += 1;
                return Some(p)
            }
        }
        None
    }
}

#[test]
fn finds_truncatable_primes() {
    use itertools::Itertools;
    assert_eq!(truncatable_primes().contains(&3797), true);
}
