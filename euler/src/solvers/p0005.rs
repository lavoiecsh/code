use num_integer::lcm;

pub fn p0005_solver() -> usize {
    smallest_multiple(20)
}

fn smallest_multiple(max: usize) -> usize {
    (1..=max)
        .reduce(|acc, cur| lcm(acc, cur))
        .unwrap()
}

#[test]
fn finds_smallest_number_divisible() {
    assert_eq!(smallest_multiple(10), 2520);
}
