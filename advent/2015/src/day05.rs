use std::collections::HashSet;
use std::fs;
use std::str::Lines;

const FILENAME: &str = "inputs/day05.txt";

fn read_input() -> String {
    fs::read_to_string(FILENAME)
        .expect("error reading")
}

pub fn part1() -> usize {
    read_input()
        .trim()
        .lines()
        .filter(|s| is_nice_part1(s))
        .count()
}

fn is_nice_part1(input: &str) -> bool {
    let mut previous: char = '0';
    let mut vowel_count: usize = 0;
    let mut contains_duplicate: bool = false;
    for c in input.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
        if previous == c {
            contains_duplicate = true;
        }
        if previous == 'a' && c == 'b' {
            return false
        }
        if previous == 'c' && c == 'd' {
            return false
        }
        if previous == 'p' && c == 'q' {
            return false
        }
        if previous == 'x' && c == 'y' {
            return false
        }
        previous = c;
    }
    return vowel_count >= 3 && contains_duplicate
}

pub fn part2() -> usize {
    read_input()
        .trim()
        .lines()
        .filter(|s| is_nice_part2(s))
        .count()
}

fn is_nice_part2(input: &str) -> bool {
    let mut pairs: HashSet<(char,char)> = HashSet::new();
    let mut previous: char = '0';
    let mut contains_duplicate: bool = false;
    let mut contains_separated: bool = false;
    let mut last_pair: (char,char) = ('0','0');
    for c in input.chars() {
        if previous == '0' {
            previous = c;
            continue;
        }
        let this_pair = (previous,c);
        contains_separated |= last_pair.0 == this_pair.1;
        contains_duplicate |= !pairs.insert(this_pair) && last_pair != ('0', '0') && last_pair != this_pair;
        last_pair = (previous,c);
        previous = c;
    }
    contains_duplicate && contains_separated
}
