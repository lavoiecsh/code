pub fn p0006_solver() -> usize {
    sum_square_difference(100)
}

fn sum_square_difference(max: usize) -> usize {
    usize::pow((1..=max).sum(), 2) - (1..=max).map(|x| x * x).sum::<usize>()
}

#[test]
fn computes_difference_between_sum_of_squares_and_square_of_sums() {
    assert_eq!(sum_square_difference(10), 2640);
}
