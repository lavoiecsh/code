use std::collections::HashSet;
use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2015Day05Solver {
    lines: Vec<String>
}

impl AdventSolver for Advent2015Day05Solver {
    fn day(&self) -> usize { 05 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| is_nice_part1(l))
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| is_nice_part2(l))
            .count()
    }
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
        let follows = match (previous, c) {
            ('a', 'b') => true,
            ('c', 'd') => true,
            ('p', 'q') => true,
            ('x', 'y') => true,
            _ => false,
        };
        if follows {
            return false;
        }
        previous = c;
    }
    return vowel_count >= 3 && contains_duplicate;
}

fn is_nice_part2(input: &str) -> bool {
    let mut pairs: HashSet<(char,char)> = HashSet::new();
    let mut chars = input.chars();
    let mut previous: char = chars.next().unwrap();
    let mut contains_duplicate: bool = false;
    let mut contains_seperated: bool = false;
    let mut last_pair: (char, char) = ('0', '0');
    for c in chars {
        let this_pair = (previous, c);
        contains_seperated |= last_pair.0 == this_pair.1;
        contains_duplicate |= !pairs.insert(this_pair) && last_pair != ('0', '0');
        last_pair = (previous, c);
        previous = c;
    }
    contains_duplicate && contains_seperated
}

pub fn advent2015_day05_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day05Solver {
        lines: read_to_string("src/year2015/day05.txt")
            .unwrap()
            .trim()
            .lines()
            .map(String::from)
            .collect()
    })
}
