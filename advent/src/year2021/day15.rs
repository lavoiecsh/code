use std::fs::read_to_string;
use crate::solver::AdventSolver;

type RisksMatrix = Vec<Vec<usize>>;

struct RisksMap {
    map: RisksMatrix,
    max_y: usize,
    max_x: usize,
}

impl RisksMap {
    fn compute_total_risks(&self) -> Self {
        let mut total_risks: RisksMap = RisksMap {
            map: vec![vec![usize::MAX; self.max_x + 1]; self.max_y + 1],
            max_y: self.max_y,
            max_x: self.max_x,
        };
        let mut modified: bool = true;
        total_risks.map[0][0] = 0;
        while modified {
            modified = false;
            for y in 0..=total_risks.max_y {
                for x in 0..=total_risks.max_x {
                    let b = total_risks.best_around(y, x);
                    if b != usize::MAX && b + self.map[y][x] < total_risks.map[y][x] {
                        total_risks.map[y][x] = b + self.map[y][x];
                        modified = true;
                    }
                }
            }
        }
        total_risks
    }

    fn best_around(&self, y: usize, x: usize) -> usize {
        let up = if y > 0 { self.map[y - 1][x] } else { usize::MAX };
        let down = if y < self.max_y { self.map[y + 1][x] } else { usize::MAX };
        let left = if x > 0 { self.map[y][x - 1] } else { usize::MAX };
        let right = if x < self.max_x { self.map[y][x + 1] } else { usize::MAX };
        *(vec![up, down, left, right]).iter().min().unwrap()
    }

    fn increase(&self) -> Self {
        let true_max_y = self.max_y + 1;
        let true_max_x = self.max_x + 1;
        let mut full = RisksMap {
            map: vec![vec![0; true_max_x * 5]; true_max_y * 5],
            max_y: true_max_y * 5 - 1,
            max_x: true_max_x * 5 - 1,
        };
        for ry in 0..5 {
            for rx in 0..5 {
                for y in 0..=self.max_y {
                    for x in 0..=self.max_x {
                        let next = self.map[y][x] + ry + rx;
                        full.map[ry * true_max_y + y][rx * true_max_x + x] = if next < 10 { next } else { next - 9 };
                    }
                }
            }
        }
        full
    }
}

pub struct Advent2021Day15Solver {
    map: RisksMap,
}

impl AdventSolver for Advent2021Day15Solver {
    fn day(&self) -> usize { 15 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        let total_risks = self.map.compute_total_risks();
        total_risks.map[total_risks.max_y][total_risks.max_x]
    }

    fn solve_part2(&self) -> usize {
        let full = self.map.increase();
        let total_risks = full.compute_total_risks();
        total_risks.map[total_risks.max_y][total_risks.max_x]
    }
}

pub fn advent2021_day15_solver() -> Box<dyn AdventSolver> {
    let map: RisksMatrix = read_to_string("src/year2021/day15.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| String::from(c).parse().unwrap()).collect())
        .collect();
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    Box::new(Advent2021Day15Solver {
        map: RisksMap { map, max_x, max_y }
    })
}
