use crate::solver::AdventSolver;

pub struct Advent2025Day03Solver {
    banks: Vec<Bank>,
}

impl Advent2025Day03Solver {
    pub fn new(input: &str) -> Self {
        Self {
            banks: input.lines()
                .map(Bank::new)
                .collect()
        }
    }
}

impl AdventSolver for Advent2025Day03Solver {
    fn solve_part1(&self) -> usize {
        self.banks.iter()
            .map(|b| b.largest_joltage(2))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.banks.iter()
            .map(|b| b.largest_joltage(12))
            .sum()
    }
}

struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn new(bank: &str) -> Self {
        Self {
            batteries: bank.chars().map(|c| c as u8 - '0' as u8).collect(),
        }
    }

    fn largest_joltage(&self, count: usize) -> usize {
        let mut indices = Vec::new();
        let mut last = 0;
        while indices.len() != count {
            last = self.largest_joltage_index(last, self.batteries.len() - (count - 1 - indices.len()));
            indices.push(last);
            last += 1;
        }
        indices.into_iter()
            .fold(0, |a, c| a * 10 + self.batteries[c] as usize)
    }

    fn largest_joltage_index(&self, start: usize, end: usize) -> usize {
        let mut max = start;
        for i in start+1..end {
            if self.batteries[i] > self.batteries[max] {
                max = i;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn finds_largest_joltage_for_2_batteries() {
        let solver = Advent2025Day03Solver::new(EXAMPLE);
        assert_eq!(solver.banks[0].largest_joltage(2), 98);
        assert_eq!(solver.banks[1].largest_joltage(2), 89);
        assert_eq!(solver.banks[2].largest_joltage(2), 78);
        assert_eq!(solver.banks[3].largest_joltage(2), 92);
        assert_eq!(solver.solve_part1(), 357);
    }

    #[test]
    fn finds_largest_joltage_for_12_batteries() {
        let solver = Advent2025Day03Solver::new(EXAMPLE);
        assert_eq!(solver.banks[0].largest_joltage(12), 987654321111);
        assert_eq!(solver.banks[1].largest_joltage(12), 811111111119);
        assert_eq!(solver.banks[2].largest_joltage(12), 434234234278);
        assert_eq!(solver.banks[3].largest_joltage(12), 888911112111);
        assert_eq!(solver.solve_part2(), 3121910778619);
    }
}
