use crate::solver::AdventSolver;
use std::collections::HashMap;
use std::ops::AddAssign;

pub struct Advent2024Day11Solver {
    stones: Vec<usize>,
}

impl Advent2024Day11Solver {
    pub fn new(input: &str) -> Self {
        Self {
            stones: input.split(' ').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2024Day11Solver {
    fn solve_part1(&self) -> usize {
        // let mut stones = self.stones.clone();
        // for _ in 0..25 {
        //     stones = blink(&stones);
        // }
        // stones.len()
        let mut stones = HashMap::new();
        self.stones.iter().for_each(|&s| stones.entry(s).or_insert(0).add_assign(1));
        for _ in 0..25 {
            stones = blink(&stones);
        }
        stones.values().sum()
    }

    fn solve_part2(&self) -> usize {
        let mut stones = HashMap::new();
        self.stones.iter().for_each(|&s| stones.entry(s).or_insert(0).add_assign(1));
        for _ in 0..75 {
            stones = blink(&stones);
        }
        stones.values().sum()
    }
}

fn blink(input: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut output = HashMap::new();
    for (k, &v) in input {
        rule(k)
            .iter()
            .for_each(|&n| output.entry(n).or_insert(0).add_assign(v));
    }
    output
}

fn rule(input: &usize) -> Vec<usize> {
    if *input == 0 {
        return vec![1];
    }
    let digit_count = input.ilog10() + 1;
    if digit_count % 2 != 0 {
        return vec![input * 2024];
    }
    let power = 10usize.pow(digit_count / 2);
    vec![input / power, input % power]
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn counts_stones_after_25_blinks() {
        let solver = Advent2024Day11Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 55312);
    }
}
