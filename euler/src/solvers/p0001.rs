pub fn p0001_solver() -> usize {
    sum_of_multiples_of_3_or_5(1000)
}

fn sum_of_multiples_of_3_or_5(under: usize) -> usize {
    (1..under)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

#[test]
fn computes_sum_of_multiples_of_3_or_5() {
    assert_eq!(sum_of_multiples_of_3_or_5(10), 23);
}
