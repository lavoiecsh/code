use crate::solver::AdventSolver;

pub struct Advent2019Day01Solver {
    module_masses: Vec<u64>,
}

impl Advent2019Day01Solver {
    pub fn new(input: &str) -> Self {
        Self {
            module_masses: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day01Solver {
    fn solve_part1(&self) -> usize {
        self.module_masses.iter().filter_map(fuel_requirement).sum::<u64>() as usize
    }

    fn solve_part2(&self) -> usize {
        self.module_masses.iter().map(fuel_requirement_rec).sum::<u64>() as usize
    }
}

fn fuel_requirement(mass: &u64) -> Option<u64> {
    let third = mass / 3;
    if third <= 2 {
        None
    } else {
        Some(third - 2)
    }
}

fn fuel_requirement_rec(mass: &u64) -> u64 {
    let mut total = 0;
    let mut current = *mass;
    while let Some(next) = fuel_requirement(&current) {
        total += next;
        current = next;
    }
    total
}