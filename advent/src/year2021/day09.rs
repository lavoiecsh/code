use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2021Day09Solver {
    height_map: Vec<Vec<u8>>,
    imax: usize,
    jmax: usize,
}

impl Advent2021Day09Solver {
    fn is_low_point(&self, i: usize, j: usize) -> bool {
        let value = self.height_map[i][j];
        if i > 0 && self.height_map[i-1][j] <= value {
            return false;
        }
        if i < self.imax-1 && self.height_map[i+1][j] <= value {
            return false;
        }
        if j > 0 && self.height_map[i][j-1] <= value {
            return false;
        }
        if j < self.jmax-1 && self.height_map[i][j+1] <= value {
            return false;
        }
        true
    }

    fn find_basin(&self, i: usize, j: usize) -> usize {
        let mut marked: Vec<(usize, usize)> = vec![(i,j)];
        let mut added: bool = true;
        while added {
            added = false;
            for (i,j) in marked.to_vec() {
                let current = self.height_map[i][j];
                if i > 0 && self.is_in_basin(current, i-1, j, &marked) {
                    marked.push((i-1,j));
                    added = true;
                }
                if i < self.imax-1 && self.is_in_basin(current, i+1, j, &marked) {
                    marked.push((i+1, j));
                    added = true;
                }
                if j > 0 && self.is_in_basin(current, i, j-1, &marked) {
                    marked.push((i, j-1));
                    added = true;
                }
                if j < self.jmax-1 && self.is_in_basin(current, i, j+1, &marked) {
                    marked.push((i, j+1));
                    added = true;
                }
            }
        }
        marked.len()
    }

    fn is_in_basin(&self, current: u8, i: usize, j: usize, marked: &Vec<(usize, usize)>) -> bool {
        let check = self.height_map[i][j];
        check != 9 && check > current && !marked.contains(&(i,j))
    }
}

impl AdventSolver for Advent2021Day09Solver {
    fn day(&self) -> usize { 09 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        let mut sum: usize = 0;
        let imax = self.height_map.len();
        let jmax = self.height_map[0].len();
        for i in 0..imax {
            for j in 0..jmax {
                if self.is_low_point(i, j) {
                    sum += self.height_map[i][j] as usize;
                }
            }
        }
        sum
    }

    fn solve_part2(&self) -> usize {
        let mut basins: Vec<usize> = Vec::new();
        let imax = self.height_map.len();
        let jmax = self.height_map[0].len();
        for i in 0..imax {
            for j in 0..jmax {
                if !self.is_low_point(i, j) {
                    continue;
                }

                basins.push(self.find_basin(i, j));
            }
        }
        basins.sort();
        basins.reverse();
        basins[0] * basins[1] * basins[2]
    }
}

pub fn advent2021_day09_solver() -> Box<dyn AdventSolver> {
    let height_map: Vec<Vec<u8>> = read_to_string("src/year2021/day09.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| String::from(c).parse().unwrap()).collect())
        .collect();
    let imax = height_map.len();
    let jmax = height_map[0].len();
    Box::new(Advent2021Day09Solver {
        height_map,
        imax,
        jmax,
    })
}
