use integers::Integer;
use number_lists::triangulars;

pub fn p0012_solver() -> String {
    highly_divisible_triangular_number(500).to_string()
}

fn highly_divisible_triangular_number(min_divisor_count: usize) -> u64 {
    triangulars::<u64>()
        .find(|t| t.proper_divisors().len() >= min_divisor_count)
        .unwrap()
}

#[test]
fn finds_first_triangular_number_with_divisors() {
    assert_eq!(highly_divisible_triangular_number(5), 28);
}
