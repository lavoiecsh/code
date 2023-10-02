use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::solver::AdventSolver;

pub struct Advent2015Day03Solver {
    movements: String
}

impl Advent2015Day03Solver {
    pub fn new(input: String) -> Self {
        Self {
            movements: input
        }
    }

    fn compute_visited_houses(&self, visited_houses: &mut HashMap<Pos, usize>) {
        let mut santa: Pos = (0,0);
        self.movements.chars().for_each(|c| {
            match c {
                '<' => santa.0 -= 1,
                '>' => santa.0 += 1,
                '^' => santa.1 -= 1,
                'v' => santa.1 += 1,
                _ => panic!("invalid character"),
            }
            *visited_houses.entry(santa).or_insert(0) += 1;
        })
    }
}

type Pos = (i64, i64);

impl AdventSolver for Advent2015Day03Solver {
    fn day(&self) -> usize { 03 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        let mut visited_houses: HashMap<Pos, usize> = HashMap::new();
        visited_houses.insert((0,0), 1);
        self.compute_visited_houses(visited_houses.borrow_mut());
        visited_houses.len()
    }

    fn solve_part2(&self) -> usize {
        let mut visited_houses: HashMap<Pos,usize> = HashMap::new();
        visited_houses.insert((0,0), 2);
        let mut santa_movements = String::new();
        let mut robosanta_movements = String::new();
        self.movements.chars().enumerate().for_each(|e| {
            if e.0 % 2 == 0 {
                santa_movements.push(e.1);
            } else {
                robosanta_movements.push(e.1);
            }
        });
        Advent2015Day03Solver { movements: santa_movements }.compute_visited_houses(visited_houses.borrow_mut());
        Advent2015Day03Solver { movements: robosanta_movements }.compute_visited_houses(visited_houses.borrow_mut());
        visited_houses.len()
    }
}
