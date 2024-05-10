use integers::Integer;
use number_lists::primes::prime_sieve;

pub fn p0035_solver() -> String {
    circular_primes(1_000_000).count().to_string()
}

fn circular_primes(max: usize) -> impl Iterator<Item=usize> {
    CircularPrimeIterator::new(max)
}

struct CircularPrimeIterator {
    prime_sieve: Vec<bool>,
    current_index: usize,
}

impl CircularPrimeIterator {
    fn new(max: usize) -> Self {
        Self {
            prime_sieve: prime_sieve(max),
            current_index: 1,
        }
    }

    fn is_circular_prime(&self, prime: usize) -> bool {
        let mut digits = prime.as_decimal().rotate_left();
        while digits.number() != prime {
            if !self.prime_sieve[digits.number()] { return false; }
            digits = digits.rotate_left();
        }
        self.prime_sieve[prime]
    }
}

impl Iterator for CircularPrimeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        while self.current_index < self.prime_sieve.len() && !self.is_circular_prime(self.current_index) {
            self.current_index += 1;
        }
        if self.current_index == self.prime_sieve.len() {
            None
        } else {
            Some(self.current_index)
        }
    }
}

#[test]
fn finds_circular_primes() {
    let cp = circular_primes(100).collect::<Vec<usize>>();
    assert_eq!(cp, vec!(2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, 97));
}
