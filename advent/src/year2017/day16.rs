use std::collections::VecDeque;

use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2017Day16Solver {
    dance_moves: Vec<DanceMove>,
}

impl Advent2017Day16Solver {
    pub fn new(input: String) -> Self {
        let spin_re = Regex::new(r"s(\d+)").unwrap();
        let exchange_re = Regex::new(r"x(\d+)/(\d+)").unwrap();
        let partner_re = Regex::new(r"p(\w)/(\w)").unwrap();
        Self {
            dance_moves: input.split(",")
                .map(|dm| {
                    if let Some(c) = spin_re.captures(dm) {
                        DanceMove::Spin(c.get(1).unwrap().as_str().parse().unwrap())
                    } else if let Some(c) = exchange_re.captures(dm) {
                        DanceMove::Exchange(
                            c.get(1).unwrap().as_str().parse().unwrap(),
                            c.get(2).unwrap().as_str().parse().unwrap()
                        )
                    } else if let Some(c) = partner_re.captures(dm) {
                        DanceMove::Partner(
                            c.get(1).unwrap().as_str().chars().next().unwrap(),
                            c.get(2).unwrap().as_str().chars().next().unwrap()
                        )
                    } else {
                        panic!("unknown dance move")
                    }
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2017Day16Solver {
    fn solve_part1_string(&self) -> String {
        let mut group = DanceGroup::new();
        group.execute_dance(&self.dance_moves);
        group.order()
    }

    fn solve_part2_string(&self) -> String {
        let max = 1000000000;
        let mut group = DanceGroup::new();
        let mut orders: Vec<String> = Vec::new();
        for i in 0..max {
            group.execute_dance(&self.dance_moves);
            let order = group.order();
            if let Some(position) = orders.iter().position(|o| o == &order) {
                let remainder = (max - i) % (i - position);
                return orders[position + remainder - 1].clone()
            } else {
                orders.push(order);
            }
        }
        group.order()
    }
}

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

struct DanceGroup {
    programs: VecDeque<char>,
}

impl DanceGroup {
    fn new() -> Self {
        Self { programs: ('a'..='p').collect() }
    }

    fn execute_dance(&mut self, dance_moves: &[DanceMove]) {
        dance_moves.iter()
            .for_each(|dm| self.execute_move(dm))
    }

    fn execute_move(&mut self, dance_move: &DanceMove) {
        match dance_move {
            DanceMove::Spin(amount) => (0..*amount).for_each(|_| {
                let back = self.programs.pop_back().unwrap();
                self.programs.push_front(back);
            }),
            DanceMove::Exchange(i,j) => self.programs.swap(*i, *j),
            DanceMove::Partner(a,b) => {
                let i = self.programs.iter().position(|x| x == a).unwrap();
                let j = self.programs.iter().position(|y| y == b).unwrap();
                self.programs.swap(i, j);
            },
        }
    }

    fn order(&self) -> String {
        self.programs.iter().collect()
    }
}
