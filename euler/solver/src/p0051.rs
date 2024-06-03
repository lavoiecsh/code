use itertools::Itertools;

use integers::digits::Digits;
use number_lists::prime_sieve;

pub fn p0051_solver() -> String {
    // todo slow solution
    prime_digit_replacements()
        .find(|v| v.len() == 8)
        .unwrap()
        .first()
        .unwrap()
        .to_string()
}

fn prime_digit_replacements() -> impl Iterator<Item=Vec<usize>> {
    PrimeDigitReplacementsIterator::new()
}

struct PrimeDigitReplacementsIterator {
    primes: Vec<bool>,
    current: usize,
}

impl PrimeDigitReplacementsIterator {
    fn new() -> Self {
        Self {
            primes: prime_sieve(1000),
            current: 0,
        }
    }
    
    fn update_primes(&mut self) {
        if self.current * 100 >= self.primes.len() {
            self.primes = prime_sieve(self.primes.len() * 100);
        }
    }
}

impl Iterator for PrimeDigitReplacementsIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_primes();
        self.current += 1;
        let mut digits = Digits::from_number(11, self.current);
        while !digits.iter().contains(&10) {
            self.current += 1;
            digits = Digits::from_number(11, self.current);
        }
        let mut v = vec!();
        for i in 0..=9 {
            if i == 0 && digits.get(0).unwrap() == 10 { continue; }
            let mut d2 = digits.clone();
            for j in 0..d2.len() {
                if d2.get(j).unwrap() != 10 { continue; }
                d2.set(j, i);
            }
            let n = d2.number_in_base(10);
            if self.primes[n] {
                v.push(n);
            }
        }
        Some(v)
    }
}

#[test]
fn finds_smallest_prime_family() {
    use itertools::Itertools;
    assert!(prime_digit_replacements().contains(&(vec!(13, 23, 43, 53, 73, 83))));
    assert!(prime_digit_replacements().contains(&(vec!(56003, 56113, 56333, 56443, 56663, 56773, 56993))));
}
