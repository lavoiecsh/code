use itertools::Itertools;

pub fn p0024_solver() -> String {
    lexicographic_permutations("0123456789", 1_000_000)
}

fn lexicographic_permutations(digits: &str, nth: usize) -> String {
    digits.chars()
        .permutations(digits.len())
        .nth(nth-1)
        .unwrap()
        .iter()
        .join("")
}

#[test]
fn finds_nth_lexicographic_permutation() {
    assert_eq!(lexicographic_permutations("012", 3), "102");
}
