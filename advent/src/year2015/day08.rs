use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2015Day08Solver {
    lines: Vec<String>
}

impl AdventSolver for Advent2015Day08Solver {
    fn day(&self) -> usize { 08 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        self.lines
            .iter()
            .map(|l| literal_count(l) - memory_count(l))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.lines
            .iter()
            .map(|l| literal_count(&escape_string(l)) - literal_count(l))
            .sum()
    }
}

fn literal_count(l: &String) -> usize {
    l.len()
}

fn memory_count(l: &String) -> usize {
    let mut count: usize = 0;
    let mut i = 0;
    let chars: Vec<char> = l.chars().collect();
    while i < chars.len() {
        if chars[i] == '"' {
            i += 1;
            continue;
        }
        if chars[i] == '\\' {
            if chars[i+1] == '\\' || chars[i+1] == '"' {
                count +=1 ;
                i += 2;
                continue;
            }
            if chars[i+1] == 'x' {
                count += 1;
                i += 4;
                continue;
            }
        }
        count += 1;
        i += 1;
    }
    count
}

fn escape_string(l: &String) -> String {
    let mut escaped: String = l.chars()
        .map(|c| if c == '\\' { "\\\\".to_string() } else if c == '"' { "\\\"".to_string() } else { c.to_string() })
        .collect();
    escaped.insert(0, '"');
    escaped.push('"');
    escaped
}

pub fn advent2015_day08_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day08Solver {
        lines: read_to_string("src/year2015/day08.txt")
            .unwrap()
            .trim()
            .lines()
            .map(String::from)
            .collect()
    })
}
