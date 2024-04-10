pub fn p0006_solver() -> String {
    sum_square_difference(100).to_string()
}

fn sum_square_difference(max: u64) -> u64 {
    u64::pow((1..=max).sum(), 2) - (1..=max).map(|x| x * x).sum::<u64>()
}

#[test]
fn computes_difference_between_sum_of_squares_and_square_of_sums() {
    assert_eq!(sum_square_difference(10), 2640);
}
