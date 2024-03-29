use num_integer::sqrt;

pub fn p0003_solver() -> usize {
    largest_prime_factor(600851475143)
}

fn largest_prime_factor(n: usize) -> usize {
    (1..=sqrt(n)).rev()
        .filter(|&p| is_prime(p))
        .find(|p| n % p == 0)
        .unwrap()
}

fn is_prime(n: usize) -> bool {
    (2..=sqrt(n))
        .all(|p| n % p != 0)
}

#[test]
fn computes_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195), 29);
}
