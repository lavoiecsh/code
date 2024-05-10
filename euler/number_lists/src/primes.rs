use std::fmt::{Debug, Formatter};
use integers::Integer;

pub fn prime_sieve(max: usize) -> Vec<bool> {
    let mut primes = vec!();
    primes.resize(max+1, true);
    for q in (4..=max).step_by(2) {
        primes[q] = false;
    }
    for p in (3..=max).step_by(2) {
        if !primes[p] { continue; }
        for q in (p+p..=max).step_by(p) {
            primes[q] = false;
        }
    }
    primes
}

pub fn prime_iterator<T: Integer>() -> PrimeIterator<T> {
    PrimeIterator::new()
}

pub struct PrimeIterator<T: Integer> {
    primes: Vec<T>,
}

impl<T: Integer> Debug for PrimeIterator<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Primes up to {}", self.primes.last().unwrap()))
    }
}

impl <T: Integer> PrimeIterator<T> {
    fn new() -> Self {
        Self { primes: vec!() }
    }

    pub fn is_prime(&self, n: T) -> bool {
        self.primes.contains(&n)
    }

    fn next_is_prime(&self, n: T) -> bool {
        self.primes.iter().take_while(|&p| p <= &n.sqrt()).all(|&p| n % p != T::zero())
    }
}

impl <T: Integer> Iterator for PrimeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.primes.len() {
            0 => { self.primes.push(T::two()); },
            1 => { self.primes.push(T::two() + T::one()); },
            n => { 
                let mut current = self.primes[n - 1] + T::two();
                while !self.next_is_prime(current) {
                    current += T::two();
                }
                self.primes.push(current);
            }
        };
        Some(*self.primes.last().unwrap())
    }
}
