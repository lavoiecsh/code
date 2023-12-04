use num_traits::pow;
use crate::solver::AdventSolver;

pub struct Advent2023Day04Solver {
    cards: Vec<Card>,
}

impl Advent2023Day04Solver {
    pub fn new(input: String) -> Self {
        Self { cards: input.lines().map(Card::from).collect() }
    }
}

impl AdventSolver for Advent2023Day04Solver {
    fn solve_part1(&self) -> usize {
        self.cards.iter().map(|c| c.point_value()).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut copies: Vec<usize> = Vec::new();
        copies.resize(self.cards.len(), 1);
        for i in 0..self.cards.len() {
            let winning_count = self.cards[i].winning_count();
            for j in i + 1..=i + winning_count {
                copies[j] += copies[i];
            }
        }
        copies.iter().sum()
    }
}

struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn point_value(&self) -> usize {
        let winning_count = self.winning_count();
        if winning_count == 0 { 0 } else { pow(2, winning_count - 1) }
    }

    fn winning_count(&self) -> usize {
        self.numbers.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let to_numbers = |s: &str| -> Vec<usize> {
            s.split(" ")
                .map(|n| n.trim())
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect()
        };
        let winning_split: Vec<&str> = value.split(":").skip(1).next().unwrap().split("|").collect();
        Card {
            winning_numbers: to_numbers(winning_split[0]),
            numbers: to_numbers(winning_split[1]),
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day04Solver {
    Advent2023Day04Solver::new(String::from("\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"))
}

#[test]
fn point_values() {
    let solver = test_solver_1();
    let point_values: Vec<usize> = solver.cards.iter().map(|c| c.point_value()).collect();
    assert_eq!(point_values, vec!(8, 2, 2, 1, 0, 0));
    assert_eq!(solver.solve_part1(), 13);
}

#[test]
fn scratchcard_count() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 30);
}
