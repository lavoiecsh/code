use std::fs::read_to_string;
use number_lists::Triangular;

pub fn p0042_solver() -> String {
    coded_triangle_numbers(&read_to_string("input/0042_words.txt").unwrap()).to_string()
}

fn coded_triangle_numbers(input: &str) -> usize {
    input.split(',')
        .map(|w| w.replace('"', ""))
        .map(|w| w.chars().map(|c| c as usize - LETTER_ZERO).sum::<usize>())
        .filter(|n| n.is_triangular())
        .count()
}

static LETTER_ZERO: usize = 'A' as usize - 1;

#[test]
fn returns_number_of_triangle_words() {
    assert_eq!(coded_triangle_numbers("SKY"), 1);
    assert_eq!(coded_triangle_numbers("\"SKY\",\"SPY\""), 1);
}
