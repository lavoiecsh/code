use num_integer::Integer;
use number_lists::fibonacci;

pub fn p0002_solver() -> String {
    even_fibonacci_numbers(4_000_000).to_string()
}

fn even_fibonacci_numbers(under: u64) -> u64 {
    fibonacci::<u64>()
        .take_while(|&n| n < under)
        .filter(|&n| n.is_even())
        .sum()
}

#[test]
fn computes_sum_of_even_fibonacci_numbers() {
    assert_eq!(even_fibonacci_numbers(100), 2 + 8 + 34);
}
