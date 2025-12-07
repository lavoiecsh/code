use std::collections::{HashMap, HashSet};
use crate::solver::AdventSolver;

pub struct Advent2025Day07Solver {
    map: TachyonBeamMap,
}

impl Advent2025Day07Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: TachyonBeamMap::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2025Day07Solver {
    fn solve_part1(&self) -> usize {
        self.map.count_splits()
    }

    fn solve_part2(&self) -> usize {
        self.map.count_timelines()
    }
}

struct TachyonBeamMap {
    grid: Vec<Vec<char>>,
    start: usize,
}

impl TachyonBeamMap {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let start = grid[0].iter().position(|&c| c == 'S').unwrap();
        Self {
            grid,
            start,
        }
    }

    fn count_splits(&self) -> usize {
        let mut count = 0;
        let mut beams = HashSet::new();
        beams.insert(self.start);
        for i in 1..self.grid.len() {
            let mut next_beams = HashSet::new();
            for b in beams {
                match self.grid[i][b] {
                    '.' => {
                        next_beams.insert(b);
                    },
                    '^' => {
                        next_beams.insert(b - 1);
                        next_beams.insert(b + 1);
                        count += 1;
                    }
                    _g => unreachable!("unknown grid character {_g}"),
                }
            }
            beams = next_beams;
        }
        count
    }

    fn count_timelines(&self) -> usize {
        let mut beams = HashMap::new();
        beams.insert(self.start, 1);
        for i in 1..self.grid.len() {
            let mut next_beams = HashMap::new();
            for (b,c) in beams {
                match self.grid[i][b] {
                    '.' => {
                        next_beams.entry(b).and_modify(|v| *v += c).or_insert(c);
                    },
                    '^' => {
                        next_beams.entry(b-1).and_modify(|v| *v += c).or_insert(c);
                        next_beams.entry(b+1).and_modify(|v| *v += c).or_insert(c);
                    }
                    _g => unreachable!("unknown grid character {_g}"),
                }
            }
            beams = next_beams;
        }
        beams.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn counts_number_of_splits() {
        let solver = Advent2025Day07Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 21);
    }

    #[test]
    fn counts_number_of_timelines() {
        let solver = Advent2025Day07Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 40);
    }
}