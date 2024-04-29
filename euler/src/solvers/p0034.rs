use std::iter::Iterator;
use crate::libs::integers::digits::digitable::Digitable;

pub fn p0034_solver() -> String {
    digit_factorials().sum::<u64>().to_string()
}

fn digit_factorials() -> impl Iterator<Item=u64> {
    DigitFactorialIterator::new()
}

struct DigitFactorialIterator {
    current: u64,
}

impl DigitFactorialIterator {
    fn new() -> Self {
        DigitFactorialIterator {
            current: 2
        }
    }

    fn current_is_digit_factorial_sum(&self) -> bool {
        self.current.as_decimal().into_iter().map(|d| (1u64..=d).product::<u64>()).sum::<u64>() == self.current
    }

    fn has_ended(&self) -> bool {
        static MAX_DIGIT_FACTORIAL: u64 = 100000;
        self.current >= MAX_DIGIT_FACTORIAL
    }
}

impl Iterator for DigitFactorialIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        while !self.current_is_digit_factorial_sum() && !self.has_ended() {
            self.current += 1;
        }
        if self.has_ended() { None } else { Some(self.current) }
    }
}

#[test]
fn finds_numbers_with_digit_factorials_summing_to_number() {
    use itertools::Itertools;

    assert_eq!(digit_factorials().contains(&145), true);
}
