use std::ops::Range;

use crate::solver::AdventSolver;

const MAX: usize = 100;
type LM = Vec<Vec<bool>>;

pub struct Advent2015Day18Solver {
    light_map: LM
}

impl Advent2015Day18Solver {
    pub fn new(input: String) -> Self {
        Self {
            light_map: input
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        }
    }
}

impl AdventSolver for Advent2015Day18Solver {
    fn solve_part1(&self) -> usize {
        let mut map = self.light_map.clone();
        for _ in 0..100 {
            map = iterate(&map);
        }
        map.iter().map(|row| row.iter().filter(|x| **x).count()).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut map = self.light_map.clone();
        fix_corners(&mut map);
        for _ in 0..100 {
            map = iterate(&map);
            fix_corners(&mut map);
        }
        map.iter().map(|row| row.iter().filter(|x| **x).count()).sum()
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
