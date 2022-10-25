use std::fs;
use json::JsonValue;
use regex::Regex;

const FILENAME: &str = "inputs/day12.txt";

fn read_input() -> JsonValue {
    json::parse(fs::read_to_string(FILENAME)
        .expect("error reading").as_str())
        .unwrap()
}

pub fn part1() -> usize {
    let o = read_input();
    compute_sum(&o) as usize
}

fn compute_sum(value: &JsonValue) -> isize {
    if value.is_number() {
        value.as_isize().unwrap()
    } else if value.is_array() {
        value.members().map(|m| compute_sum(m)).sum()
    } else if value.is_object() {
        value.entries().map(|e| compute_sum(e.1)).sum()
    } else {
        0
    }
}

pub fn part2() -> usize {
    let o = read_input();
    let tmp = json::parse("{\"e\":86,\"c\":23,\"a\":{\"a\":[120,169,\"green\",\"red\",\"orange\"],\"b\":\"red\"},\"g\":\"yellow\",\"b\":[\"yellow\"],\"d\":\"red\",\"f\":-19}").unwrap();
    compute_sum_without_red(&o) as usize
}

fn compute_sum_without_red(value: &JsonValue) -> isize {
    if value.is_number() {
        value.as_isize().unwrap()
    } else if value.is_array() {
        value.members().map(|m| compute_sum_without_red(m)).sum()
    } else if value.is_object() {
        if value.entries().any(|e| e.1 == "red") {
            0
        } else {
            value.entries().map(|e| compute_sum_without_red(e.1)).sum()
        }
    } else {
        0
    }
}
