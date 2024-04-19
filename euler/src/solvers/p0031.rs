use crate::libs::algorithms::dynamic_programming::dynamic_programming;

pub fn p0031_solver() -> String {
    coin_sums(200).to_string()
}

fn coin_sums(target: usize) -> usize {
    let coin_values = vec!(1, 2, 5, 10, 20, 50, 100, 200);
    dynamic_programming(&coin_values, target)
}

#[test]
fn counts_possible_ways_to_sum_to_target() {
    assert_eq!(coin_sums(1), 1);
    assert_eq!(coin_sums(2), 2);
    assert_eq!(coin_sums(5), 4);
    assert_eq!(coin_sums(10), 11);
}
