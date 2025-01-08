use crate::solver::AdventSolver;
use std::collections::HashMap;

pub struct Advent2024Day19Solver {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Advent2024Day19Solver {
    pub fn new(input: &str) -> Self {
        Self {
            patterns: input.lines().next().unwrap().split(", ").map(String::from).collect(),
            designs: input.lines().skip(2).map(String::from).collect(),
        }
    }
}

impl AdventSolver for Advent2024Day19Solver {
    fn solve_part1(&self) -> usize {
        let mut design_builder = DesignBuilder::new(&self.patterns);
        self.designs
            .iter()
            .filter(|d| design_builder.count(d) > 0)
            .count()
    }

    fn solve_part2(&self) -> usize {
        let mut design_builder = DesignBuilder::new(&self.patterns);
        self.designs
            .iter()
            .map(|d| design_builder.count(d))
            .sum()
    }
}

struct DesignBuilder<'a> {
    patterns: &'a [String],
    counts: HashMap<String, usize>,
}

impl<'a> DesignBuilder<'a> {
    fn new(patterns: &'a [String]) -> Self {
        let mut counts = HashMap::new();
        counts.insert(String::new(), 1);
        Self { patterns, counts }
    }

    fn count(&mut self, design: &str) -> usize {
        if let Some(&count) = self.counts.get(design) {
            return count;
        }
        let count = self.patterns
            .iter()
            .filter(|p| design.starts_with(*p))
            .map(|p| design.split_at(p.len()).1)
            .map(|r| self.count(r))
            .sum();
        self.counts.insert(design.to_string(), count);
        count
    }
}

//noinspection SpellCheckingInspection
#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn counts_possible_designs() {
        let solver = Advent2024Day19Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 6);
    }

    #[test]
    fn counts_ways_to_make_possible_designs() {
        let solver = Advent2024Day19Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 16);
    }
}
