use num_integer::sqrt;

pub fn p0012_solver() -> usize {
    highly_divisible_triangular_number(500)
}

fn highly_divisible_triangular_number(min_divisor_count: usize) -> usize {
    (1..usize::MAX)
        .map(|i| (i * i + i) / 2)
        .find(|&t| divisor_count(t) > min_divisor_count)
        .unwrap()
}

fn divisor_count(n: usize) -> usize {
    (1..=sqrt(n))
        .filter(|d| n % d == 0)
        .count() * 2
}

#[test]
fn finds_first_triangular_number_with_divisors() {
    assert_eq!(highly_divisible_triangular_number(5), 28);
}
