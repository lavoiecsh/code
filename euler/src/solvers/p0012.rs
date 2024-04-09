use crate::libs::divisors::proper_divisors;

pub fn p0012_solver() -> String {
    highly_divisible_triangular_number(500).to_string()
}

fn highly_divisible_triangular_number(min_divisor_count: usize) -> u128 {
    (1..u128::MAX)
        .map(|i| (i * i + i) / 2)
        .find(|&t| proper_divisors(t).count() + 1 > min_divisor_count)
        .unwrap()
}

#[test]
fn finds_first_triangular_number_with_divisors() {
    assert_eq!(highly_divisible_triangular_number(5), 28);
}
