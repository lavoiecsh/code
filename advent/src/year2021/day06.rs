use crate::solver::AdventSolver;

pub struct Advent2021Day06Solver {
    fish_count: [usize; 9]
}

impl Advent2021Day06Solver {
    pub fn new(input: String) -> Self {
        let mut fish_count = [0; 9];
        input
            .split(",")
            .map(|s| s.parse().unwrap())
            .for_each(|f: usize| fish_count[f] += 1);
        Self { fish_count }
    }
}

impl AdventSolver for Advent2021Day06Solver {
    fn solve_part1(&self) -> usize {
        let mut fish = self.fish_count;
        for _ in 0..80 {
            fish = iterate(&fish);
        }
        fish.iter().sum()
    }

    fn solve_part2(&self) -> usize {
        let mut fish = self.fish_count;
        for _ in 0..256 {
            fish = iterate(&fish);
        }
        fish.iter().sum()
    }
}

fn iterate(fish: &[usize; 9]) -> [usize; 9] {
    let mut new_fish = [0; 9];
    new_fish[..8].copy_from_slice(&fish[1..9]);
    new_fish[6] += fish[0];
    new_fish[8] = fish[0];
    new_fish
}
