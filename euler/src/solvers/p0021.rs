use crate::libs::integer_divisible::Divisible;

pub fn p0021_solver() -> String {
    amicable_numbers(10_000).to_string()
}

fn amicable_numbers(max: u128) -> u128 {
    (1..=max)
        .map(|n| (n, n.proper_divisors().sum()))
        .filter(|&(n,s)| n != s && s.proper_divisors().sum::<u128>() == n)
        .map(|(n,_)| n)
        .sum()
}
