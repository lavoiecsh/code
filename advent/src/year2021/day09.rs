use crate::solver::AdventSolver;

pub struct Advent2021Day09Solver {
    height_map: Vec<Vec<u8>>,
    imax: usize,
    jmax: usize,
}

impl Advent2021Day09Solver {
    pub fn new(input: &str) -> Self {
        let height_map: Vec<Vec<u8>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| String::from(c).parse().unwrap())
                    .collect()
            })
            .collect();
        let imax = height_map.len();
        let jmax = height_map[0].len();
        Self {
            height_map,
            imax,
            jmax,
        }
    }

    fn is_low_point(&self, i: usize, j: usize) -> bool {
        let value = self.height_map[i][j];
        if i > 0 && self.height_map[i - 1][j] <= value {
            return false;
        }
        if i < self.imax - 1 && self.height_map[i + 1][j] <= value {
            return false;
        }
        if j > 0 && self.height_map[i][j - 1] <= value {
            return false;
        }
        if j < self.jmax - 1 && self.height_map[i][j + 1] <= value {
            return false;
        }
        true
    }

    fn find_basin(&self, i: usize, j: usize) -> usize {
        let mut marked: Vec<(usize, usize)> = vec![];
        let mut new_marked: Option<(usize, usize)> = Some((i, j));
        while let Some(nm) = new_marked {
            marked.push(nm);
            new_marked = None;
            for (i, j) in marked.iter().copied() {
                let current = self.height_map[i][j];
                if i > 0 && self.is_in_basin(current, i - 1, j, &marked) {
                    new_marked = Some((i - 1, j));
                    break;
                }
                if i < self.imax - 1 && self.is_in_basin(current, i + 1, j, &marked) {
                    new_marked = Some((i + 1, j));
                    break;
                }
                if j > 0 && self.is_in_basin(current, i, j - 1, &marked) {
                    new_marked = Some((i, j - 1));
                    break;
                }
                if j < self.jmax - 1 && self.is_in_basin(current, i, j + 1, &marked) {
                    new_marked = Some((i, j + 1));
                    break;
                }
            }
        }
        marked.len()
    }

    fn is_in_basin(&self, current: u8, i: usize, j: usize, marked: &[(usize, usize)]) -> bool {
        let check = self.height_map[i][j];
        check != 9 && check > current && !marked.contains(&(i, j))
    }
}

impl AdventSolver for Advent2021Day09Solver {
    fn solve_part1(&self) -> usize {
        let mut sum: usize = 0;
        let imax = self.height_map.len();
        let jmax = self.height_map[0].len();
        for i in 0..imax {
            for j in 0..jmax {
                if self.is_low_point(i, j) {
                    sum += self.height_map[i][j] as usize + 1;
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
