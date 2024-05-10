use std::fs::read_to_string;
use itertools::Itertools;

pub fn p0022_solver() -> String {
    name_scores(&read_to_string("input/0022_names.txt").unwrap()).to_string()
}

fn name_scores(input: &str) -> u64 {
    input.split(',')
        .map(|n| n.replace('"', ""))
        .sorted()
        .enumerate()
        .map(|(i,n)| name_score(&n) * (i as u64 + 1))
        .sum()
}

fn name_score(name: &str) -> u64 {
    name.chars()
        .map(|c| c as u64 - LETTER_ZERO)
        .sum()
}

static LETTER_ZERO: u64 = 'A' as u64 - 1;

#[test]
fn computes_score_for_name() {
    assert_eq!(name_score("COLIN"), 53);
}
