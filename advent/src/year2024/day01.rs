use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2024Day01Solver {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Advent2024Day01Solver {
    pub fn new(input: &str) -> Self {
        Self {
            left: input.lines()
                .map(|l| l.split("   ").nth(0).unwrap())
                .map(|n| n.parse().unwrap())
                .collect(),
            right: input.lines()
                .map(|l| l.split("   ").nth(1).unwrap())
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2024Day01Solver {
    fn solve_part1(&self) -> usize {
        self.left.iter()
            .sorted()
            .zip(self.right.iter().sorted())
            .map(|(&l, &r)| l.abs_diff(r) as usize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.left.iter()
            .map(|&l| self.right.iter().filter(|&&r| l == r).count() * l as usize)
            .sum()

    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn finds_total_distance_between_ordered_lists() {
        let solver = Advent2024Day01Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 11);
    }

    #[test]
    fn finds_similarity_score() {
        let solver = Advent2024Day01Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 31);
    }
}
