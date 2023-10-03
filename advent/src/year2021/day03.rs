use crate::solver::AdventSolver;

pub struct Advent2021Day03Solver {
    numbers: Vec<usize>,
    power: usize,
}

impl Advent2021Day03Solver {
    pub fn new(input: String) -> Self {
        let numbers: Vec<usize> = input
            .trim()
            .lines()
            .map(|s| usize::from_str_radix(s, 2).unwrap())
            .collect();
        let largest = numbers.iter().cloned().fold(0, usize::max);
        let mut power = 1;
        while power < largest { power *= 2 };
        power /= 2;
        Self { numbers, power }
    }
}

impl Advent2021Day03Solver {
    fn find_rating(&self, use0: fn(c0: usize, c1: usize) -> bool) -> usize {
        let mut numbers = self.numbers.clone();
        let mut power = self.power;
        while power > 0 {
            if numbers.len() == 1 {
                return numbers[0];
            }
            let c1 = count1(&numbers, power);
            let c0 = numbers.len() - c1;
            let x = if use0(c0, c1) { 0 } else { power };
            numbers = numbers.iter().cloned().filter(|n| n & power == x).collect();
            power /= 2;
        }
        numbers[0]
    }
}

fn count1(numbers: &Vec<usize>, power: usize) -> usize {
    numbers
        .iter()
        .fold(0, |acc, n| acc + if n & power != 0 { 1 } else { 0 })
}

impl AdventSolver for Advent2021Day03Solver {
    fn solve_part1(&self) -> usize {
        let mut power = self.power;
        let mut gamma: usize = 0;
        let mut epsilon: usize = 0;
        while power > 0 {
            let c1 = count1(&self.numbers, power);
            if c1 > self.numbers.len() / 2 {
                gamma += power;
            } else {
                epsilon += power;
            }
            power /= 2;
        }
        gamma * epsilon
    }

    fn solve_part2(&self) -> usize {
        let oxygen = self.find_rating(|c0, c1| c0 > c1);
        let co2 = self.find_rating(|c0, c1| c0 <= c1);
        oxygen * co2
    }
}
