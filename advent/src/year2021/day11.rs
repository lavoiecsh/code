use crate::solver::AdventSolver;

pub struct Advent2021Day11Solver {
    levels: Vec<Vec<u8>>,
}

impl Advent2021Day11Solver {
    pub fn new(input: &str) -> Self {
        Self {
            levels: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| String::from(c).parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2021Day11Solver {
    fn solve_part1(&self) -> usize {
        let mut levels = self.levels.clone();
        let imax: usize = levels.len();
        let jmax: usize = levels[0].len();
        let mut total: usize = 0;
        for _ in 0..100 {
            total += step(&mut levels, imax, jmax);
        }
        total
    }

    fn solve_part2(&self) -> usize {
        let mut levels = self.levels.clone();
        let imax = levels.len();
        let jmax = levels[0].len();
        let mut iter: usize = 0;
        while !all_flashed(&levels) {
            step(&mut levels, imax, jmax);
            iter += 1;
        }
        iter
    }
}

fn all_flashed(levels: &[Vec<u8>]) -> bool {
    levels.iter().all(|row| row.iter().all(|c| *c == 0))
}

fn step(levels: &mut [Vec<u8>], imax: usize, jmax: usize) -> usize {
    let mut tmp: Vec<Vec<(u8, bool)>> = levels
        .iter()
        .map(|row| row.iter().map(|col| (col + 1, false)).collect())
        .collect();
    let mut modified = true;
    while modified {
        modified = false;
        for i in 0..imax {
            for j in 0..jmax {
                let (v, f) = tmp[i][j];
                if v > 9 && !f {
                    modified = true;
                    tmp[i][j] = (v, true);
                    increase_around(&mut tmp, imax, jmax, i, j);
                }
            }
        }
    }
    let mut count: usize = 0;
    for i in 0..imax {
        for j in 0..jmax {
            let (v, f) = tmp[i][j];
            if f {
                count += 1;
            }
            levels[i][j] = if f { 0 } else { v };
        }
    }
    count
}

fn increase_around(levels: &mut [Vec<(u8, bool)>], imax: usize, jmax: usize, i: usize, j: usize) {
    let min_x = if i == 0 { 0 } else { i - 1 };
    let max_x = if i == imax - 1 { i } else { i + 1 };
    let min_y = if j == 0 { 0 } else { j - 1 };
    let max_y = if j == jmax - 1 { j } else { j + 1 };
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if x == i && y == j {
                continue;
            }
            let (v, f) = levels[x][y];
            levels[x][y] = (v + 1, f);
        }
    }
}
