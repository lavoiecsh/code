use number_lists::prime_iterator;

pub fn p0010_solver() -> String {
    summation_of_primes(2_000_000).to_string()
}

fn summation_of_primes(max: u64) -> u64 {
    prime_iterator::<u64>()
        .take_while(|&n| n < max)
        .sum()
}

#[test]
fn finds_sum_of_primes_under_value() {
    assert_eq!(summation_of_primes(10), 17);
}
