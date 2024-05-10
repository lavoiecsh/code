use number_theory::Perfect;

pub fn p0021_solver() -> String {
    amicable_numbers(10_000).to_string()
}

fn amicable_numbers(max: u128) -> u128 {
    (1..=max)
        .filter_map(|n| n.amicable_pair())
        .sum()
}
