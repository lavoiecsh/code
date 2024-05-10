use number_lists::prime_iterator;

pub fn p0007_solver() -> String {
    nth_prime(10_001).to_string()
}

fn nth_prime(n: usize) -> u64 {
    prime_iterator::<u64>().nth(n-1).unwrap()
}

#[test]
fn finds_nth_prime() {
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(10), 29);
}
