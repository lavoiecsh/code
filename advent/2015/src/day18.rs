use std::fs;
use std::ops::Range;
use crate::day19::Problem19Solver;
use crate::problem_solver::ProblemSolver;

const MAX: usize = 100;
type LM = Vec<Vec<bool>>;

pub struct Problem18Solver {
    light_map: LM,
}
impl Problem18Solver {
    pub fn new() -> Self {
        Self {
            light_map: fs::read_to_string("inputs/day18.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        }
    }
}

impl ProblemSolver for Problem18Solver {
    fn solve_part1(&self) -> usize {
        let mut map = self.light_map.clone();
        for _ in 0..100 {
            map = iterate(&map);
        }
        let mut count = 0;
        for r in 0..MAX {
            for c in 0..MAX {
                if map[r][c] {
                    count += 1;
                }
            }
        }
        count
    }

    fn solve_part2(&self) -> usize {
        let mut map = self.light_map.clone();
        fix_corners(&mut map);
        for _ in 0..100 {
            map = iterate(&map);
            fix_corners(&mut map);
        }
        let mut count = 0;
        for r in 0..MAX {
            for c in 0..MAX {
                if map[r][c] {
                    count += 1;
                }
            }
        }
        count
    }
}

fn fix_corners(map: &mut LM) {
    map[0][0] = true;
    map[0][MAX-1] = true;
    map[MAX-1][0] = true;
    map[MAX-1][MAX-1] = true;
}

fn iterate(map: &LM) -> LM {
    let mut next: LM = Vec::new();
    for r in 0..MAX {
        let mut row = Vec::new();
        for c in 0..MAX {
            let count = count_neighbours(map, r, c);
            row.push(if map[r][c] { count == 2 || count == 3 } else { count == 3 });
        }
        next.push(row);
    }
    next
}

fn count_neighbours(map: &LM, row: usize, col: usize) -> usize {
    let mut count = 0;
    for r in neighbour_range(row) {
        for c in neighbour_range(col) {
            if r == row && c == col { continue; }
            if map[r][c] { count += 1; }
        }
    }
    count
}

fn neighbour_range(a: usize) -> Range<usize> {
    (if a == 0 { 0 } else { a - 1 })..(if a == MAX - 1 { MAX } else { a + 2 })
}
