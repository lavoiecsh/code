use std::fs;

const FILENAME: &str = "inputs/day01.txt";

fn read_input() -> String {
    fs::read_to_string(FILENAME)
        .expect("error reading")
}

pub fn part1() -> usize {
    read_input()
        .chars()
        .fold(0, |acc, cur| acc + eval(cur))
    as usize
}

pub fn part2() -> usize {
    read_input()
        .chars()
        .fold(Fold { i: 0, c: 0, found: false }, accumulate_fold)
        .i
}

fn eval(c: char) -> i64 {
    if c == '(' { 1 } else { -1 }
}

struct Fold {
    i: usize,
    c: i64,
    found: bool,
}

fn accumulate_fold(acc: Fold, cur: char) -> Fold {
    if acc.found { return acc }
    let c = acc.c + eval(cur);
    Fold {
        i: acc.i + 1,
        c,
        found: c < 0
    }
}
