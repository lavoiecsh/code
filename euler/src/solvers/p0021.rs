use crate::libs::divisors::proper_divisors;

pub fn p0021_solver() -> String {
    amicable_numbers(10_000).to_string()
}

fn amicable_numbers(max: u128) -> u128 {
    (1..=max)
        .map(|n| (n, proper_divisors(n).sum()))
        .filter(|&(n,s)| n != s && proper_divisors(s).sum::<u128>() == n)
        .map(|(n,_)| n)
        .sum()
}
