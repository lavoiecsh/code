use num_integer::lcm;

pub fn p0005_solver() -> String {
    smallest_multiple(20).to_string()
}

fn smallest_multiple(max: u64) -> u64 {
    (1..=max)
        .reduce(lcm)
        .unwrap()
}

#[test]
fn finds_smallest_number_divisible() {
    assert_eq!(smallest_multiple(10), 2520);
}
