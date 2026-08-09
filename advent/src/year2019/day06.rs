use crate::solver::AdventSolver;
use std::collections::{HashMap, VecDeque};

pub struct Advent2019Day06Solver {
    orbits: VecDeque<(String, String)>,
}

impl Advent2019Day06Solver {
    pub fn new(input: &str) -> Self {
        Self {
            orbits: input
                .lines()
                .map(|l| l.split_once(')').unwrap())
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
        }
    }
}

impl AdventSolver for Advent2019Day06Solver {
    fn solve_part1(&self) -> usize {
        let mut counts = HashMap::new();
        let mut remaining = self.orbits.clone();
        let (starting_center, _) = self
            .orbits
            .iter()
            .find(|(a, _)| self.orbits.iter().all(|(_, b)| a != b))
            .unwrap();
        counts.insert(starting_center.clone(), 0);
        while let Some((from, to)) = remaining.pop_front() {
            if let Some(count) = counts.get(&from) {
                counts.insert(to, count + 1);
            } else {
                remaining.push_back((from, to));
            }
        }
        counts.values().sum()
    }

    fn solve_part2(&self) -> usize {
        let mut you_path = vec![String::from("YOU")];
        let mut last_you = &you_path[0];
        while let Some((previous, _)) = self.orbits.iter().find(|(_, b)| b == last_you) {
            you_path.push(previous.clone());
            last_you = previous;
        }

        let mut san_path = vec![String::from("SAN")];
        let mut last_san = &san_path[0];
        while let Some((previous, _)) = self.orbits.iter().find(|(_, b)| b == last_san) {
            san_path.push(previous.clone());
            last_san = previous;
        }

        let ancestor = you_path.iter().find(|s| san_path.contains(s)).unwrap();
        you_path.iter().position(|a| a == ancestor).unwrap()
            + san_path.iter().position(|a| a == ancestor).unwrap()
            - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
    const EXAMPLE_2: &str = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

    #[test]
    fn counts_direct_and_indirect_orbits() {
        let solver = Advent2019Day06Solver::new(EXAMPLE_1);
        assert_eq!(solver.solve_part1(), 42);
    }

    #[test]
    fn finds_minimum_number_of_orbit_transfers() {
        let solver = Advent2019Day06Solver::new(EXAMPLE_2);
        assert_eq!(solver.solve_part2(), 4);
    }
}
