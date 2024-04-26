use crate::libs::integers::digitable::Digitable;

pub fn p0036_solver() -> String {
    double_base_palindromes(1_000_000).sum::<u64>().to_string()
}

fn double_base_palindromes(max: u64) -> impl Iterator<Item=u64> {
    (1..max)
        .filter(|n| n.as_decimal().is_palindromic() && n.as_binary().is_palindromic())
}

#[test]
fn finds_palindromes_in_decimal_and_binary() {
    use itertools::Itertools;
    assert_eq!(double_base_palindromes(1000).contains(&585), true);
}
