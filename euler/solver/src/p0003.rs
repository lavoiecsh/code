use integers::Integer;

pub fn p0003_solver() -> String {
    largest_prime_factor(600851475143).to_string()
}

fn largest_prime_factor(n: u64) -> u64 {
    n.factorize()
        .largest()
        .unwrap()
}

#[test]
fn computes_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195), 29);
}
