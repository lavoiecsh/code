use number_lists::prime_sieve;

pub fn p0027_solver() -> String {
    let (a, b) = quadratic_primes();
    (a * b).to_string()
}

fn quadratic_primes() -> (i32, i32) {
    let primes = prime_sieve(1_000_000);
    (-999..=999)
        .flat_map(|a| (-1000..=1000).map(move |b| (a,b)))
        .map(|(a,b)| ((a, b), quadratic_prime_count(a, b, &primes)))
        .max_by_key(|&(_,c)| c)
        .unwrap()
        .0
}

fn quadratic_prime_count(a: i32, b: i32, primes: &[bool]) -> usize {
    (0..i32::MAX)
        .map(|n| n * n + a * n + b)
        .take_while(|&x| x > 0 && primes[x as usize])
        .count()
}

#[test]
fn counts_quadratic_primes() {
    let primes = prime_sieve(1_000_000);
    assert_eq!(quadratic_prime_count(1, 41, &primes), 40);
    assert_eq!(quadratic_prime_count(-79, 1601, &primes), 80);
}
