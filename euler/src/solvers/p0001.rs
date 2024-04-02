pub fn p0001_solver() -> String {
    multiples_of_3_or_5(1_000).to_string()
}

fn multiples_of_3_or_5(under: usize) -> usize {
    (1..under)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

#[test]
fn computes_sum_of_multiples_of_3_or_5() {
    assert_eq!(multiples_of_3_or_5(10), 23);
}
