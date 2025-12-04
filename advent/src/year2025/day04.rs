use crate::solver::AdventSolver;

pub struct Advent2025Day04Solver {
    map: PaperRollMap,
}

impl Advent2025Day04Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: PaperRollMap {
                map: input
                    .lines()
                    .map(|l| l.chars().map(|c| c == '@').collect())
                    .collect(),
            },
        }
    }
}

impl AdventSolver for Advent2025Day04Solver {
    fn solve_part1(&self) -> usize {
        self.map.accessible_paper_rolls().len()
    }

    fn solve_part2(&self) -> usize {
        self.map.clone().remove_all_accessible_paper_rolls()
    }
}

#[derive(Clone)]
struct PaperRollMap {
    map: Vec<Vec<bool>>,
}

type Pos = (usize, usize);

impl PaperRollMap {
    fn remove_all_accessible_paper_rolls(&mut self) -> usize {
        let mut accessible = self.accessible_paper_rolls();
        let mut count = accessible.len();
        while !accessible.is_empty() {
            self.remove_accessible_paper_rolls(&accessible);
            accessible = self.accessible_paper_rolls();
            count += accessible.len();
        }
        count
    }

    fn accessible_paper_rolls(&self) -> Vec<Pos> {
        let mut accessible = Vec::new();
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if !self.map[row][col] {
                    continue;
                }
                let pos = (row, col);
                let count = self.count_surrounding(pos);
                if count < 4 {
                    accessible.push((row, col));
                }
            }
        }
        accessible
    }

    fn remove_accessible_paper_rolls(&mut self, rolls: &[Pos]) {
        for pos in rolls {
            self.map[pos.0][pos.1] = false;
        }
    }

    fn count_surrounding(&self, pos: Pos) -> usize {
        let mut count = 0;
        if pos.0 > 0 {
            let row = &self.map[pos.0 - 1];
            if pos.1 > 0 && row[pos.1 - 1] {
                count += 1;
            }
            if row[pos.1] {
                count += 1;
            }
            if pos.1 < row.len() - 1 && row[pos.1 + 1] {
                count += 1;
            }
        }
        if pos.0 < self.map.len() - 1 {
            let row = &self.map[pos.0 + 1];
            if pos.1 > 0 && row[pos.1 - 1] {
                count += 1;
            }
            if row[pos.1] {
                count += 1;
            }
            if pos.1 < row.len() - 1 && row[pos.1 + 1] {
                count += 1;
            }
        }
        let row = &self.map[pos.0];
        if pos.1 > 0 && row[pos.1 - 1] {
            count += 1;
        }
        if pos.1 < row.len() - 1 && row[pos.1 + 1] {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn counts_accessible_paper_rolls() {
        let solver = Advent2025Day04Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 13);
    }

    #[test]
    fn counts_total_removable_paper_rolls() {
        let solver = Advent2025Day04Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 43);
    }
}
