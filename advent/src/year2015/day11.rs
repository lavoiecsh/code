use std::collections::HashSet;

use crate::solver::AdventSolver;

pub struct Advent2015Day11Solver {
    input: String,
}

impl Advent2015Day11Solver {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl AdventSolver for Advent2015Day11Solver {
    fn solve_part1_string(&self) -> String {
        let mut password: Vec<u8> = self.input.chars().map(|c| c as u8).collect();
        next(&mut password);
        while is_invalid(&password) {
            next(&mut password);
        }
        password.iter().map(|c| *c as char).collect()
    }

    fn solve_part2_string(&self) -> String {
        let mut password: Vec<u8> = self.input.chars().map(|c| c as u8).collect();
        next(&mut password);
        while is_invalid(&password) {
            next(&mut password);
        }
        next(&mut password);
        while is_invalid(&password) {
            next(&mut password);
        }
        password.iter().map(|c| *c as char).collect()
    }
}

fn is_invalid(password: &Vec<u8>) -> bool {
    // first rule: increasing straight of 3 letters
    let mut straight_found = false;
    for i in 2..8 {
        if password[i] == password[i - 1] + 1 && password[i] == password[i - 2] + 2 {
            straight_found = true;
            break;
        }
    }
    if !straight_found {
        return true;
    }
    // second rule: no i, o, l
    for i in 0..8 {
        if password[i] == 'i' as u8 || password[i] == 'o' as u8 || password[i] == 'l' as u8 {
            return true;
        }
    }
    // third rule: 2 different pairs
    let mut pairs: HashSet<u8> = HashSet::new();
    for i in 1..8 {
        if password[i] == password[i - 1] {
            pairs.insert(password[i]);
        }
    }
    pairs.len() < 2
}

fn next(password: &mut Vec<u8>) {
    for i in (0..8).rev() {
        password[i] += 1;
        if password[i] <= 'z' as u8 {
            break;
        }
        password[i] = 'a' as u8;
    }
}
