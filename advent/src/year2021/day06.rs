use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2021Day06Solver {
    fish_count: [usize; 9]
}

impl AdventSolver for Advent2021Day06Solver {
    fn day(&self) -> usize { 06 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        let mut fish = self.fish_count.clone();
        for _ in 0..80 {
            fish = iterate(&fish);
        }
        fish.iter().sum()
    }

    fn solve_part2(&self) -> usize {
        let mut fish = self.fish_count.clone();
        for _ in 0..256 {
            fish = iterate(&fish);
        }
        fish.iter().sum()
    }
}

fn iterate(fish: &[usize; 9]) -> [usize; 9] {
    let mut new_fish = [0; 9];
    for n in 0..8 {
        new_fish[n] = fish[n+1];
    }
    new_fish[6] += fish[0];
    new_fish[8] = fish[0];
    new_fish
}

pub fn advent2021_day06_solver() -> Box<dyn AdventSolver> {
    let mut fish_count = [0; 9];
    read_to_string("src/year2021/day06.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .for_each(|f: usize| fish_count[f] += 1);
    Box::new(Advent2021Day06Solver {
        fish_count
    })
}
