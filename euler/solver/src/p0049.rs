use std::collections::HashSet;
use itertools::Itertools;
use integers::Integer;
use number_lists::prime_sieve;

pub fn p0049_solver() -> String {
    prime_permutations()
        .find(|&(a, _, _)| a != 1487)
        .map(|(a, b, c)| format!("{a}{b}{c}"))
        .unwrap()
}

fn prime_permutations() -> impl Iterator<Item=(usize, usize, usize)> {
    let primes = prime_sieve(10_000);
    let mut v: HashSet<(usize, usize, usize)> = HashSet::new();
    for i in 1000..10_000 {
        if !primes[i] { continue; }
        let all_primes: Vec<usize> = i.as_decimal()
            .permutations()
            .into_iter()
            .map(|n| n.number())
            .filter(|&c| (1000..10000).contains(&c))
            .filter(|&c| primes[c])
            .sorted()
            .collect();
        if all_primes.len() < 3 { continue; }
        for ai in 0..all_primes.len() {
            let a = all_primes[ai];
            for bi in ai+1..all_primes.len() {
                let b = all_primes[bi];
                if a == b { continue; }
                let diff = b - a;
                for &c in all_primes.iter().skip(bi+1) {
                    if b == c { continue; }
                    if c - diff == b {
                        v.insert((a,b,c));
                    }
                }
            }
        }
    }
    v.into_iter()
}

#[test]
fn finds_sequence_of_prime_permutations() {
    use itertools::Itertools;
    assert!(prime_permutations().contains(&(1487, 4817, 8147)));
}