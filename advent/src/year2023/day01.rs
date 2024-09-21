use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day01Solver {
    inputs: Vec<String>,
}

impl Advent2023Day01Solver {
    pub fn new(input: String) -> Self {
        Self { inputs: input.lines().map(String::from).collect() }
    }
}

impl AdventSolver for Advent2023Day01Solver {
    fn solve_part1(&self) -> usize {
        self.inputs.iter()
            .map(calibration_value_digits)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.inputs.iter()
            .map(calibration_value_spelled)
            .sum()
    }
}

fn calibration_value_digits(input: impl Into<String>) -> usize {
    let digits: Vec<usize> = input.into().matches(char::is_numeric).map(|c| c.parse().unwrap()).collect();
    digits[0] * 10 + digits[digits.len() - 1]
}

fn calibration_value_spelled(input: impl Into<String>) -> usize {
    let input = input.into();
    let choices = vec!("1", "2", "3", "4", "5", "6", "7", "8", "9",
                       "one", "two", "three", "four", "five", "six", "seven", "eight", "nine");

    let first = choices.iter()
        .filter_map(|c| input.match_indices(c).next())
        .sorted_by_key(|(i, _)| *i)
        .next()
        .unwrap()
        .1;

    let last = choices.iter()
        .filter_map(|c| input.rmatch_indices(c).next())
        .sorted_by_key(|(i, _)| *i)
        .next_back()
        .unwrap()
        .1;

    to_digit(first) * 10 + to_digit(last)
}

fn to_digit(input: &str) -> usize {
    match input {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        x => panic!("impossible capture {x}"),
    }
}

#[test]
fn calibrates_numbers_only() {
    assert_eq!(calibration_value_digits("1abc2"), 12);
    assert_eq!(calibration_value_digits("pqr3stu8vwx"), 38);
    assert_eq!(calibration_value_digits("a1b2c3d4e5f"), 15);
    assert_eq!(calibration_value_digits("treb7uchet"), 77);
}

#[test]
fn calibrates_with_spelled() {
    assert_eq!(calibration_value_spelled("two1nine"), 29);
    assert_eq!(calibration_value_spelled("eightwothree"), 83);
    assert_eq!(calibration_value_spelled("abcone2threexyz"), 13);
    assert_eq!(calibration_value_spelled("xtwone3four"), 24);
    assert_eq!(calibration_value_spelled("4nineeightseven2"), 42);
    assert_eq!(calibration_value_spelled("zoneight234"), 14);
    assert_eq!(calibration_value_spelled("7pqrstsixteen"), 76);
}

#[test]
fn calibrates_with_spelled_from_end() {
    assert_eq!(calibration_value_spelled("28gtboneightmx"), 28);
}
