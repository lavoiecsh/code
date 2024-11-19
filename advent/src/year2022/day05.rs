use regex::Regex;

use crate::solver::AdventSolver;

#[derive(Debug)]
struct CraneOperation {
    count: usize,
    from: usize,
    to: usize,
}

struct Yard {
    stacks: Vec<Vec<char>>,
}

impl Yard {
    fn new(stacks: Vec<Vec<char>>) -> Self {
        Self { stacks }
    }

    fn execute_single(&mut self, operation: &CraneOperation) {
        for _ in 0..operation.count {
            let c = self.stacks[operation.from].pop().unwrap();
            self.stacks[operation.to].push(c);
        }
    }

    fn execute_multi(&mut self, operation: &CraneOperation) {
        let mut tmp = Vec::new();
        for _ in 0..operation.count {
            tmp.push(self.stacks[operation.from].pop().unwrap());
        }
        self.stacks[operation.to].extend(tmp);
    }
}

pub struct Advent2022Day05Solver {
    starting_stacks: Vec<Vec<char>>,
    operations: Vec<CraneOperation>,
}

impl Advent2022Day05Solver {
    pub fn new(input: &str) -> Self {
        let text = input
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let mut stacks = Vec::new();
        for _ in 1..=9 {
            stacks.push(Vec::new());
        }
        for i in (0..8).rev() {
            let chars = text[i].clone().chars().collect::<Vec<char>>();
            for s in 0..9 {
                let c = s * 4 + 1;
                if chars.len() > c && chars[c] != ' ' {
                    stacks[s].push(chars[c]);
                }
            }
        }
        let operation_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        Self {
            starting_stacks: stacks,
            operations: text
                .iter()
                .skip(10)
                .map(|l| {
                    let m = operation_regex.captures(l).unwrap();
                    CraneOperation {
                        count: m.get(1).unwrap().as_str().parse().unwrap(),
                        from: m.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                        to: m.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2022Day05Solver {
    fn solve_part1_string(&self) -> String {
        let mut yard = Yard::new(self.starting_stacks.clone());
        self.operations.iter().for_each(|o| yard.execute_single(o));
        yard.stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    fn solve_part2_string(&self) -> String {
        let mut yard = Yard::new(self.starting_stacks.clone());
        self.operations.iter().for_each(|o| yard.execute_multi(o));
        yard.stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}
