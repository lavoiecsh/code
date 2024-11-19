use crate::solver::AdventSolver;
use num_traits::pow;

pub struct Advent2017Day15Solver {
    generator_a_start: usize,
    generator_b_start: usize,
}

impl Advent2017Day15Solver {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        Self {
            generator_a_start: lines
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            generator_b_start: lines
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}

impl AdventSolver for Advent2017Day15Solver {
    fn solve_part1(&self) -> usize {
        Judge::new(
            Generator::new(self.generator_a_start, 16807, 1),
            Generator::new(self.generator_b_start, 48271, 1),
        )
        .count_matches(40000000)
    }

    fn solve_part2(&self) -> usize {
        Judge::new(
            Generator::new(self.generator_a_start, 16807, 4),
            Generator::new(self.generator_b_start, 48271, 8),
        )
        .count_matches(5000000)
    }
}

struct Judge {
    generator_a: Generator,
    generator_b: Generator,
    match_size: usize,
}

impl Judge {
    fn new(generator_a: Generator, generator_b: Generator) -> Self {
        Self {
            generator_a,
            generator_b,
            match_size: pow(2, 16),
        }
    }

    fn count_matches(&mut self, amount: usize) -> usize {
        (0..amount).filter(|_| self.does_next_match()).count()
    }

    fn does_next_match(&mut self) -> bool {
        let a = self.generator_a.next();
        let b = self.generator_b.next();
        (a % self.match_size) == (b % self.match_size)
    }
}

struct Generator {
    current: usize,
    factor: usize,
    dividend: usize,
    criteria: usize,
}

impl Generator {
    fn new(start: usize, factor: usize, criteria: usize) -> Self {
        Self {
            current: start,
            factor,
            dividend: 2147483647,
            criteria,
        }
    }

    fn next(&mut self) -> usize {
        self.current = (self.current * self.factor) % self.dividend;
        while self.current % self.criteria != 0 {
            self.current = (self.current * self.factor) % self.dividend;
        }
        self.current
    }
}
