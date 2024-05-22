use number_lists::{prime_iterator, PrimeIterator};

pub fn p0046_solver() -> String {
    goldbachs_other_conjecture().next().unwrap().to_string()
}

fn goldbachs_other_conjecture() -> impl Iterator<Item=u64> {
    GoldbachsOtherConjectureIterator::new()
}

struct GoldbachsOtherConjectureIterator {
    primes: PrimeIterator<u64>,
    composite: u64,
}

impl GoldbachsOtherConjectureIterator {
    fn new() -> Self {
        Self {
            primes: prime_iterator(),
            composite: 7,
        }
    }

    fn update_primes(&mut self) {
        while self.primes.next().unwrap() < self.composite {}
    }

    fn find_next_composite(&mut self) {
        self.composite += 2;
        self.update_primes();
        while self.primes.is_prime(self.composite) {
            self.composite += 2;
            self.update_primes();
        }
    }

    fn is_composite_goldbach(&self) -> bool {
        (1..)
            .map(|n| 2 * n * n)
            .take_while(|&n| n < self.composite)
            .map(|n| self.composite - n)
            .any(|n| self.primes.is_prime(n))
    }
}

impl Iterator for GoldbachsOtherConjectureIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.find_next_composite();
        while self.is_composite_goldbach() {
            self.find_next_composite();
        }
        Some(self.composite)
    }
}
