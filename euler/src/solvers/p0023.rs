use std::collections::HashSet;
use crate::libs::integers::number_types::perfect_numbers::Perfect;

pub fn p0023_solver() -> String {
    non_abundant_sums().to_string()
}

fn non_abundant_sums() -> u64 {
    let abundant_numbers = (12..=ABUNDANT_SUM_LIMIT)
        .filter(|&n| n.is_abundant())
        .collect::<Vec<u64>>();
    let abundant_sums = (0..abundant_numbers.len())
        .flat_map(|a| (a..abundant_numbers.len()).map(move |b| (a,b)))
        .map(|(a,b)| abundant_numbers[a] + abundant_numbers[b])
        .filter(|&s| s <= ABUNDANT_SUM_LIMIT)
        .collect::<HashSet<u64>>();
    (1..=ABUNDANT_SUM_LIMIT)
        .filter(|n| !abundant_sums.contains(n))
        .sum()
}

static ABUNDANT_SUM_LIMIT: u64 = 28123;
