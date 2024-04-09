use std::fs::read_to_string;
use itertools::Itertools;

pub fn p0022_solver() -> String {
    name_scores(&read_to_string("input/0022_names.txt").unwrap()).to_string()
}

fn name_scores(input: &str) -> u128 {
    input.split(',')
        .map(|n| n.replace('"', ""))
        .sorted()
        .enumerate()
        .map(|(i,n)| name_score(&n) * (i as u128 + 1))
        .sum()
}

fn name_score(name: &str) -> u128 {
    name.chars()
        .map(|c| c as u128 - LETTER_ZERO)
        .sum()
}

static LETTER_ZERO: u128 = 'A' as u128 - 1;

#[test]
fn computes_score_for_name() {
    assert_eq!(name_score("COLIN"), 53);
}
