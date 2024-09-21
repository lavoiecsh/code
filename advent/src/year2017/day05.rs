use crate::solver::AdventSolver;

pub struct Advent2017Day05Solver {
    offsets: Vec<i32>,
}

impl Advent2017Day05Solver {
    pub fn new(input: String) -> Self {
        Self { offsets: input.lines().map(|l| l.parse().unwrap()).collect() }
    }
}

impl AdventSolver for Advent2017Day05Solver {
    fn solve_part1(&self) -> usize {
        Maze::new(&self.offsets).run(|n| n + 1)
    }

    fn solve_part2(&self) -> usize {
        Maze::new(&self.offsets).run(|n| if n >= 3 { n - 1 } else { n + 1 })
    }
}

struct Maze {
    jumps: Vec<i32>,
}

impl Maze {
    fn new(offsets: &[i32]) -> Self {
        Self { jumps: offsets.to_owned() }
    }

    fn run(&mut self, incrementer: fn (i32) -> i32) -> usize {
        let mut pointer: i32 = 0;
        let mut steps = 0;
        while pointer >= 0 && (pointer as usize) < self.jumps.len() {
            let jump = self.jumps[pointer as usize];
            self.jumps[pointer as usize] = incrementer(self.jumps[pointer as usize]);
            pointer += jump;
            steps += 1;
        }
        steps
    }
}
