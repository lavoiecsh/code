use crate::libs::integers::digits::digitable::Digitable;
use crate::libs::integers::primes::prime_sieve;

pub fn p0041_solver() -> String {
    // 2, 3, 5, 6, 8, 9 pandigital numbers are all divisible by 3 by sum of digits
    pandigital_primes(7)
        .into_iter()
        .max()
        .or_else(|| pandigital_primes(4).into_iter().max())
        .unwrap()
        .to_string()
}

fn pandigital_primes(n: u32) -> Vec<usize> {
    let primes = prime_sieve(10usize.pow(n));
    let mut digits = 1usize.as_decimal();
    (2..=n as usize).for_each(|d| digits.concatenate(d.as_decimal()));
    digits.permutations().iter()
        .map(|p| p.number())
        .filter(|&p| primes[p])
        .collect()
}

#[test]
fn finds_pandigital_primes() {
    assert_eq!(pandigital_primes(4).contains(&2143), true);
}
