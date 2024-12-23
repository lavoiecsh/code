use std::cmp::Ordering;

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2015Day24Solver {
    input: Vec<usize>,
}

impl Advent2015Day24Solver {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2015Day24Solver {
    fn solve_part1(&self) -> usize {
        split(&self.input, 3)
            .iter()
            .sorted_by(|l, r| group_compare(l, r))
            .next()
            .unwrap()
            .entanglement as usize
    }

    fn solve_part2(&self) -> usize {
        split(&self.input, 4)
            .iter()
            .sorted_by(|l, r| group_compare(l, r))
            .next()
            .unwrap()
            .entanglement as usize
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct GroupEntanglement {
    size: usize,
    entanglement: u128,
}

fn split(numbers: &[usize], group_count: usize) -> Vec<GroupEntanglement> {
    let group_sum = numbers.iter().sum::<usize>() / group_count;
    let max_group_length = numbers.len() / group_count;
    let mut entanglements = Vec::new();
    let max: usize = 2usize.pow(numbers.len() as u32);
    for i in 0..max {
        let group = vec_extract(numbers, i);
        if group.len() > max_group_length || group.iter().sum::<usize>() != group_sum {
            continue;
        }
        let entanglement: u128 = group.iter().fold(1u128, |acc, cur| acc * (*cur as u128));
        entanglements.push(GroupEntanglement {
            size: group.len(),
            entanglement,
        });
    }
    entanglements
}

fn vec_extract(numbers: &[usize], element_index: usize) -> Vec<usize> {
    let mut group = Vec::new();
    let mut elements = element_index;
    let mut index = 0;
    while elements != 0 {
        if elements % 2 == 1 {
            group.push(numbers[index]);
        }
        elements /= 2;
        index += 1;
    }
    group
}

fn group_compare(left: &GroupEntanglement, right: &GroupEntanglement) -> Ordering {
    if left.size < right.size {
        return Ordering::Less;
    }
    if left.size > right.size {
        return Ordering::Greater;
    }
    if left.entanglement < right.entanglement {
        return Ordering::Less;
    }
    if left.entanglement > right.entanglement {
        return Ordering::Greater;
    }
    Ordering::Equal
}
