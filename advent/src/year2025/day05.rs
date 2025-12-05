use crate::solver::AdventSolver;
use std::ops::RangeInclusive;
use itertools::Itertools;

pub struct Advent2025Day05Solver {
    fresh_ingredients: FreshIngredients,
    available_ingredients: Vec<u64>,
}

impl Advent2025Day05Solver {
    pub fn new(input: &str) -> Self {
        let mut fresh_ingredient_ranges = Vec::new();
        let mut available_ingredients = Vec::new();
        let mut reading_fresh = true;
        for line in input.lines() {
            if line.is_empty() {
                reading_fresh = false;
                continue;
            }
            if reading_fresh {
                let range = line.split('-').collect::<Vec<&str>>();
                fresh_ingredient_ranges.push(range[0].parse().unwrap()..=range[1].parse().unwrap());
            } else {
                available_ingredients.push(line.parse().unwrap());
            }
        }
        let mut fresh_ingredients = FreshIngredients::new(&fresh_ingredient_ranges);
        let mut next_fresh_ingredients = FreshIngredients::from(&fresh_ingredients);
        while next_fresh_ingredients.len() < fresh_ingredients.len() {
            fresh_ingredients = next_fresh_ingredients;
            next_fresh_ingredients = FreshIngredients::from(&fresh_ingredients);
        }
        Self {
            fresh_ingredients,
            available_ingredients,
        }
    }
}

impl AdventSolver for Advent2025Day05Solver {
    fn solve_part1(&self) -> usize {
        self.available_ingredients
            .iter()
            .filter(|i| self.fresh_ingredients.contains(i))
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.fresh_ingredients.count() as usize
    }
}

struct FreshIngredients {
    ranges: Vec<RangeInclusive<u64>>,
}

impl FreshIngredients {
    fn new(ranges: &Vec<RangeInclusive<u64>>) -> Self {
        Self { ranges: ranges.clone() }
    }

    fn from(ingredients: &FreshIngredients) -> Self {
        let mut s = Self { ranges: Vec::new() };
        ingredients.ranges.iter().for_each(|r| s.insert(r));
        s
    }

    fn contains(&self, value: &u64) -> bool {
        self.ranges.iter().any(|r| r.contains(value))
    }

    fn len(&self) -> usize {
        self.ranges.len()
    }

    fn count(&self) -> u64 {
        self.ranges
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum()
    }

    fn insert(&mut self, range: &RangeInclusive<u64>) {
        if self.ranges.iter().any(|r| r.encompasses(range)) {
            return;
        }
        if let Some((i, _)) = self.ranges.iter().find_position(|r| range.encompasses(r)) {
            self.ranges[i] = range.clone();
            return;
        }
        if let Some((i, Some(o))) = self.ranges.iter().map(|r| r.overlaps(&range)).find_position(|o| o.is_some()) {
            self.ranges[i] = o;
            return;
        }
        self.ranges.push(range.clone());
    }
}

trait RangeInclusiveComparisons where Self: Sized {
    fn encompasses(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> Option<Self>;
}

impl RangeInclusiveComparisons for RangeInclusive<u64> {
    fn encompasses(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &Self) -> Option<Self> {
        if self.contains(other.start()) {
            Some(*self.start()..=*other.end())
        } else if self.contains(other.end()) {
            Some(*other.start()..=*self.end())
        } else if other.contains(self.start()) {
            Some(*other.start()..=*self.end())
        } else if other.contains(self.end()) {
            Some(*self.start()..=*other.end())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn finds_fresh_ingredients_from_list() {
        let solver = Advent2025Day05Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 3);
    }

    #[test]
    fn counts_all_fresh_ingredients() {
        let solver = Advent2025Day05Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 14);
    }
}