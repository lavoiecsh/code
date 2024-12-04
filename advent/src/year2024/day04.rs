use crate::solver::AdventSolver;

pub struct Advent2024Day04Solver {
    grid: Grid,
}

impl Advent2024Day04Solver {
    pub fn new(input: &str) -> Self {
        Self {
            grid: Grid::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2024Day04Solver {
    fn solve_part1(&self) -> usize {
        self.grid.xmas_search()
    }

    fn solve_part2(&self) -> usize {
        self.grid.mas_search()
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self {
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn xmas_search(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 3..self.width {
                if self.xmas_matches([y, y, y, y].into_iter().zip(x - 3..=x)) {
                    count += 1;
                }
            }
        }
        for y in 3..self.height {
            for x in 0..self.width {
                if self.xmas_matches((y - 3..=y).zip([x, x, x, x].into_iter())) {
                    count += 1;
                }
            }
        }
        for y in 3..self.height {
            for x in 0..self.width - 3 {
                if self.xmas_matches((y - 3..=y).rev().zip(x..=x + 3)) {
                    count += 1;
                }
            }
        }
        for y in 3..self.height {
            for x in 3..self.width {
                if self.xmas_matches((y - 3..=y).zip(x - 3..=x)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn xmas_matches(&self, pos: impl Iterator<Item = (usize, usize)>) -> bool {
        let window: Vec<char> = pos.map(|(y, x)| self.grid[y][x]).collect();
        window == ['X', 'M', 'A', 'S'] || window == ['S', 'A', 'M', 'X']
    }

    fn mas_search(&self) -> usize {
        let mut count = 0;
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if self.mas_matches(y, x) {
                    count += 1;
                }
            }
        }
        count
    }

    fn mas_matches(&self, y: usize, x: usize) -> bool {
        let forward = [
            self.grid[y - 1][x - 1],
            self.grid[y][x],
            self.grid[y + 1][x + 1],
        ];
        let backward = [
            self.grid[y - 1][x + 1],
            self.grid[y][x],
            self.grid[y + 1][x - 1],
        ];
        (forward == ['M', 'A', 'S'] || forward == ['S', 'A', 'M'])
            && (backward == ['M', 'A', 'S'] || backward == ['S', 'A', 'M'])
    }
}

#[cfg(test)]
//noinspection SpellCheckingInspection
mod test {
    use super::*;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn finds_xmas_count() {
        let solver = Advent2024Day04Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 18);
    }

    #[test]
    fn finds_cross_mas_count() {
        let solver = Advent2024Day04Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 9);
    }
}
