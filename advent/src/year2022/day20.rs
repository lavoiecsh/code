use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::VecDeque;

pub struct Advent2022Day20Solver {
    numbers: Vec<isize>,
}

impl Advent2022Day20Solver {
    pub fn new(input: &str) -> Self {
        Self {
            numbers: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }
}

fn mix(input: &[isize], times: usize) -> Vec<isize> {
    let mut output: VecDeque<(usize, isize)> = input.iter().copied().enumerate().collect();
    let modulo = input.len() as isize - 1;
    for _ in 0..times {
        for index in 0..input.len() {
            let current_number = input[index];
            let current_index = output.iter().position(|(i, _)| i == &index).unwrap();
            let mut moving_number = current_number % modulo;
            let mut moving_index = current_index;
            while moving_number != 0 {
                if moving_number < 0 {
                    if moving_index == 0 {
                        output.pop_front();
                        output.push_back((index, current_number));
                        moving_index = output.len() - 1;
                    } else {
                        output.swap(moving_index, moving_index - 1);
                        moving_index -= 1;
                        moving_number += 1;
                    }
                } else if moving_index == output.len() - 1 {
                    output.pop_back();
                    output.push_front((index, current_number));
                    moving_index = 0;
                } else {
                    output.swap(moving_index, moving_index + 1);
                    moving_index += 1;
                    moving_number -= 1;
                }
            }
        }
    }
    output.iter().map(|(_, n)| *n).collect()
}

fn get_coord(numbers: &[isize], n: usize) -> isize {
    let mut n2 = n + numbers.iter().position(|x| x == &0).unwrap();
    while n2 >= numbers.len() {
        n2 -= numbers.len();
    }
    numbers[n2]
}

impl AdventSolver for Advent2022Day20Solver {
    fn solve_part1(&self) -> usize {
        let output = mix(&self.numbers, 1);
        [1000, 2000, 3000]
            .iter()
            .map(|n| get_coord(&output, *n))
            .sum::<isize>() as usize
    }

    fn solve_part2(&self) -> usize {
        let output = mix(
            &self.numbers.iter().map(|n| n * 811589153).collect_vec(),
            10,
        );
        [1000, 2000, 3000]
            .iter()
            .map(|n| get_coord(&output, *n))
            .sum::<isize>() as usize
    }
}
