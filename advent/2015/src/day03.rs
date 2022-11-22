use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs;
use std::str::Chars;
use crate::problem_solver::ProblemSolver;

pub struct Problem03Solver {
    movements: String
}

impl Problem03Solver {
    pub fn new() -> Self {
        Self { 
            movements: fs::read_to_string("inputs/day03.txt")
                .expect("error reading")
                .trim()
                .to_string()
        }
    }
}

impl ProblemSolver for Problem03Solver {
    fn solve_part1(&self) -> usize {
        let mut visited_houses: HashMap<Pos,usize> = HashMap::new();
        visited_houses.insert((0,0), 1);
        compute_visited_houses(visited_houses.borrow_mut(), self.movements.chars());
        visited_houses.len()
    }

    fn solve_part2(&self) -> usize {
        let mut visited_houses: HashMap<Pos,usize> = HashMap::new();
        visited_houses.insert((0,0), 2);
        let mut santa_movements: String = String::new();
        let mut robosanta_movements: String = String::new();
        self.movements.chars().enumerate().for_each(|e| {
            if e.0 % 2 == 0 {
                santa_movements.push(e.1);
            } else {
                robosanta_movements.push(e.1);
            }
        });
        compute_visited_houses(visited_houses.borrow_mut(), santa_movements.chars());
        compute_visited_houses(visited_houses.borrow_mut(), robosanta_movements.chars());
        visited_houses.len()
    }
}

type Pos = (i64,i64);

fn compute_visited_houses(visited_houses: &mut HashMap<Pos,usize>, movements: Chars) {
    let mut santa: Pos = (0,0);
    movements.for_each(|c| {
        if c == '<' {
            santa.0 -= 1;
        } else if c == '>' {
            santa.0 += 1;
        } else if c == '^' {
            santa.1 -= 1;
        } else if c == 'v' {
            santa.1 += 1;
        }
        *visited_houses.entry(santa).or_insert(0) += 1;
    });
}
