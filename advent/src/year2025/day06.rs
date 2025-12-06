use crate::solver::AdventSolver;
use std::ops::Range;

pub struct Advent2025Day06Solver {
    homework: Homework,
}

impl Advent2025Day06Solver {
    pub fn new(input: &str) -> Self {
        Self {
            homework: Homework::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2025Day06Solver {
    fn solve_part1(&self) -> usize {
        self.homework.left_to_right()
    }

    fn solve_part2(&self) -> usize {
        self.homework.top_to_bottom()
    }
}

struct Homework {
    grid: Vec<Vec<char>>,
    column_indices: Vec<usize>,
    row_length: usize,
}

impl Homework {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let column_indices = grid[grid.len() - 1]
            .iter()
            .enumerate()
            .filter(|(_, c)| c != &&' ')
            .map(|(i, _)| i)
            .collect();
        let row_length = grid.iter().map(|r| r.len()).max().unwrap();
        Self {
            grid,
            column_indices,
            row_length,
        }
    }

    fn column_indices(&self, col: usize) -> Range<usize> {
        if col == self.column_indices.len() - 1 {
            self.column_indices[col]..self.row_length
        } else {
            self.column_indices[col]..self.column_indices[col + 1] - 1
        }
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.grid[row].get(col).cloned().unwrap_or(' ')
    }

    fn left_to_right(&self) -> usize {
        (0..self.column_indices.len())
            .map(|col| self.operation(col)(self.left_to_right_column(col)))
            .sum()
    }

    fn left_to_right_column(&self, col: usize) -> Vec<usize> {
        (0..self.grid.len() - 1)
            .map(|row| self.column_indices(col)
                .map(|col| self.get(row, col)).collect())
            .map(|s: String| s.trim().parse().unwrap())
            .collect()
    }

    fn top_to_bottom(&self) -> usize {
        (0..self.column_indices.len())
            .map(|col| self.operation(col)(self.top_to_bottom_column(col)))
            .sum()
    }

    fn top_to_bottom_column(&self, col: usize) -> Vec<usize> {
        self.column_indices(col)
            .map(|col| (0..self.grid.len() - 1)
                .map(|row| self.get(row, col)).collect())
            .map(|s: String| s.trim().parse().unwrap())
            .collect()
    }

    fn operation(&self, col: usize) -> fn(numbers: Vec<usize>) -> usize {
        match self.grid[self.grid.len() - 1][self.column_indices[col]] {
            '+' => |numbers| numbers.iter().sum(),
            '*' => |numbers| numbers.iter().product(),
            _o => unreachable!("unknown operation {_o}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn calculates_left_to_right_total() {
        let solver = Advent2025Day06Solver::new(EXAMPLE);
        assert_eq!(solver.homework.left_to_right_column(0), vec![123, 45, 6]);
        assert_eq!(solver.homework.left_to_right_column(1), vec![328, 64, 98]);
        assert_eq!(solver.homework.left_to_right_column(2), vec![51, 387, 215]);
        assert_eq!(solver.homework.left_to_right_column(3), vec![64, 23, 314]);
        assert_eq!(solver.solve_part1(), 4277556);
    }

    #[test]
    fn calculates_top_to_bottom_total() {
        let solver = Advent2025Day06Solver::new(EXAMPLE);
        assert_eq!(solver.homework.top_to_bottom_column(0), vec![1, 24, 356]);
        assert_eq!(solver.homework.top_to_bottom_column(1), vec![369, 248, 8]);
        assert_eq!(solver.homework.top_to_bottom_column(2), vec![32, 581, 175]);
        assert_eq!(solver.homework.top_to_bottom_column(3), vec![623, 431, 4]);
        assert_eq!(solver.solve_part2(), 3263827);
    }
}
