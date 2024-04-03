use std::collections::HashMap;
use itertools::Itertools;

pub fn p0014_solver() -> String {
    longest_collatz_sequence(1_000_000).to_string()
}

fn longest_collatz_sequence(max: u128) -> u128 {
    let mut map: HashMap<u128, u128> = HashMap::new();
    map.insert(1, 1);
    for i in 2..max {
        calculate_collatz(i, &mut map);
    }
    map.into_iter()
        .filter(|&(k,_)| k < max)
        .sorted_by_key(|&(_,v)| v)
        .last()
        .unwrap()
        .0
}

fn calculate_collatz(value: u128, map: &mut HashMap<u128, u128>) -> u128 {
    if let Some(c) = map.get(&value) { return *c; }
    let n = calculate_collatz(if value % 2 == 0 { value / 2 } else { value * 3 + 1 }, map) + 1;
    map.insert(value, n);
    n
}

#[test]
fn finds_longest_collatz_sequence_starting_under() {
    assert_eq!(longest_collatz_sequence(15), 9);
}
