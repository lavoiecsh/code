use crate::solver::AdventSolver;

pub struct Advent2016Day18Solver {
    first_row: Vec<bool>,
}

impl Advent2016Day18Solver {
    pub fn new(input: String) -> Self {
        Self { first_row: input.chars().map(|c| c == '.').collect() }
    }

    fn count_safe(&self, rows: usize) -> usize {
        let mut row: Vec<bool> = self.first_row.clone();
        let mut count = row.iter().filter(|s| **s).count();
        for _ in 1..rows {
            row = next_row(&row);
            count += row.iter().filter(|s| **s).count();
        }
        count
    }
}

impl AdventSolver for Advent2016Day18Solver {
    fn solve_part1(&self) -> usize {
        self.count_safe(40)
    }

    fn solve_part2(&self) -> usize {
        self.count_safe(400000)
    }
}

fn next_row(row: &Vec<bool>) -> Vec<bool> {
    (0..row.len())
        .map(|i| under(row, i))
        .collect()
}

fn under(row: &Vec<bool>, index: usize) -> bool {
    let left = if index == 0 { true } else { row[index-1] };
    let center = row[index];
    let right = if index == row.len() - 1 { true } else { row[index+1] };
    match (left, center, right) {
        (false, false, true) => false,
        (true, false, false) => false,
        (false, true, true) => false,
        (true, true, false) => false,
        _ => true,
    }
}
