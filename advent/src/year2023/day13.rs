use crate::solver::AdventSolver;

pub struct Advent2023Day13Solver {
    patterns: Vec<Pattern>,
}

impl Advent2023Day13Solver {
    pub fn new(input: String) -> Self {
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let mut patterns = vec!();
        let mut current = vec!();
        for line in lines {
            if line.is_empty() {
                patterns.push(Pattern::new(current));
                current = vec!();
                continue;
            }
            current.push(line);
        }
        patterns.push(Pattern::new(current));
        Self { patterns }
    }
}

impl AdventSolver for Advent2023Day13Solver {
    fn solve_part1(&self) -> usize {
        self.patterns.iter()
            .map(|p| match p.find_reflection() {
                Some(Reflection::Horizontal(n)) => 100 * n,
                Some(Reflection::Vertical(n)) => n,
                None => panic!("no reflection found")
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.patterns.iter()
            .map(|p| match p.find_fixed_reflection() {
                Some(Reflection::Horizontal(n)) => 100 * n,
                Some(Reflection::Vertical(n)) => n,
                None => panic!("no fixed reflection found")
            })
            .sum()
    }
}

struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
    max_row: usize,
    max_col: usize,
}

impl Pattern {
    fn new(rows: Vec<String>) -> Self {
        let max_row = rows.len();
        let max_col = rows[0].len();


        let mut cols = vec!();
        cols.resize(max_col, String::new());
        for col in 0..max_col {
            for row in 0..max_row {
                cols[col].push(rows[row].chars().nth(col).unwrap());
            }
        }
        Self { rows, cols, max_row, max_col }
    }

    fn fix(&self, row: usize, col: usize) -> Self {
        let replace_with = if self.rows[row].chars().nth(col).unwrap() == '#' { "." } else { "#" };
        let mut rows = self.rows.clone();
        rows[row].replace_range(col..=col, replace_with);
        let mut cols = self.cols.clone();
        cols[col].replace_range(row..=row, replace_with);
        Self { rows, cols, max_row: self.max_row, max_col: self.max_col }
    }

    fn find_reflection(&self) -> Option<Reflection> {
        for row in 1..self.max_row {
            if self.reflects_horizontal(row) {
                return Some(Reflection::Horizontal(row));
            }
        }
        for col in 1..self.max_col {
            if self.reflects_vertical(col) {
                return Some(Reflection::Vertical(col));
            }
        }
        None
    }

    fn find_reflection_excluding(&self, excluded: &Reflection) -> Option<Reflection> {
        let excluded_row = match excluded { Reflection::Horizontal(r) => *r, _ => self.max_row };
        let excluded_col = match excluded { Reflection::Vertical(r) => *r, _ => self.max_col };
        for row in 1..self.max_row {
            if row == excluded_row { continue; }
            if self.reflects_horizontal(row) {
                return Some(Reflection::Horizontal(row));
            }
        }
        for col in 1..self.max_col {
            if col == excluded_col { continue; }
            if self.reflects_vertical(col) {
                return Some(Reflection::Vertical(col));
            }
        }
        None
    }

    fn find_fixed_reflection(&self) -> Option<Reflection> {
        let original_reflection = self.find_reflection().unwrap();
        for row in 0..self.max_row {
            for col in 0..self.max_col {
                let reflection = self.fix(row, col).find_reflection_excluding(&original_reflection);
                if reflection.is_some() {
                    return reflection;
                }
            }
        }
        None
    }

    fn reflects_horizontal(&self, row: usize) -> bool {
        let max = if row > self.max_row / 2 { self.max_row - row } else { row };
        for r in 0..max {
            if self.rows[row - r - 1] != self.rows[row + r] { return false; }
        }
        true
    }

    fn reflects_vertical(&self, col: usize) -> bool {
        let max = if col > self.max_col / 2 { self.max_col - col } else { col };
        for c in 0..max {
            if self.cols[col - c - 1] != self.cols[col + c] { return false; }
        }
        true
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day13Solver {
    Advent2023Day13Solver::new(String::from("\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"))
}

#[test]
fn finds_reflection_plane() {
    let solver = test_solver_1();
    assert_eq!(solver.patterns[0].find_reflection(), Some(Reflection::Vertical(5)));
    assert_eq!(solver.patterns[1].find_reflection(), Some(Reflection::Horizontal(4)));
    assert_eq!(solver.solve_part1(), 405);
}

#[test]
fn finds_fixed_reflection_plane() {
    let solver = test_solver_1();
    assert_eq!(solver.patterns[0].find_fixed_reflection(), Some(Reflection::Horizontal(3)));
    assert_eq!(solver.patterns[1].find_fixed_reflection(), Some(Reflection::Horizontal(1)));
    assert_eq!(solver.solve_part2(), 400);
}
