use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2025Day02Solver {
    product_ranges: Vec<ProductRange>,
}

impl Advent2025Day02Solver {
    pub fn new(input: &str) -> Self {
        Self {
            product_ranges: input
                .split(',')
                .map(|r| r.split('-'))
                .map(|mut s| ProductRange {
                    first: s.next().unwrap().parse::<usize>().unwrap(),
                    last: s.next().unwrap().parse::<usize>().unwrap(),
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2025Day02Solver {
    fn solve_part1(&self) -> usize {
        self.product_ranges
            .iter()
            .flat_map(ProductRange::half_invalid_ids)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.product_ranges
            .iter()
            .flat_map(ProductRange::full_invalid_ids)
            .sum()
    }
}

struct ProductRange {
    first: usize,
    last: usize,
}

impl ProductRange {
    fn half_invalid_ids(&self) -> impl Iterator<Item = usize> {
        (self.first..=self.last).filter(is_half_invalid)
    }

    fn full_invalid_ids(&self) -> impl Iterator<Item = usize> {
        (self.first..=self.last).filter(is_full_invalid)
    }
}

fn is_half_invalid(id: &usize) -> bool {
    let full_size = id.ilog10();
    if full_size % 2 == 0 {
        return false;
    }
    let half_size = full_size / 2 + 1;
    let half_power = 10usize.pow(half_size);
    let first_half = id / half_power;
    let second_half = id % half_power;
    first_half == second_half
}

fn is_full_invalid(id: &usize) -> bool {
    let mut size = 10;
    while size < *id {
        if is_size_invalid(id, size) {
            return true;
        }
        size *= 10;
    }
    false
}

fn is_size_invalid(id: &usize, size: usize) -> bool {
    let mut splits = Vec::new();
    let mut id = *id;
    let expected_size = size.ilog10() - 1;
    while id > 0 {
        let split = id % size;
        if split == 0 || split.ilog10() != expected_size {
            return false;
        }
        splits.push(split);
        id /= size;
    }
    splits.iter().all_equal()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    macro_rules! half_invalid_ids {
        ($solver: expr, $i: tt) => {
            $solver
                .product_ranges
                .get($i)
                .unwrap()
                .half_invalid_ids()
                .collect::<Vec<_>>()
        };
    }

    macro_rules! full_invalid_ids {
        ($solver: expr, $i: tt) => {
            $solver
                .product_ranges
                .get($i)
                .unwrap()
                .full_invalid_ids()
                .collect::<Vec<_>>()
        };
    }

    #[test]
    fn finds_half_invalid_ids() {
        let solver = Advent2025Day02Solver::new(EXAMPLE);
        assert_eq!(half_invalid_ids!(solver, 0), vec![11, 22]);
        assert_eq!(half_invalid_ids!(solver, 1), vec![99]);
        assert_eq!(solver.solve_part1(), 1227775554);
    }

    #[test]
    fn finds_all_invalid_ids() {
        let solver = Advent2025Day02Solver::new(EXAMPLE);
        assert_eq!(full_invalid_ids!(solver, 0), vec![11, 22]);
        assert_eq!(full_invalid_ids!(solver, 1), vec![99, 111]);
        assert_eq!(solver.solve_part2(), 4174379265);
    }
}
