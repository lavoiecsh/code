use eval::Expr;
use itertools::Itertools;
use regex::Regex;

use crate::solver::AdventSolver;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    // operation: fn(old: usize) -> usize,
    operation: Expr,
    divisibility_test: usize,
    when_true: usize,
    when_false: usize,
    inspection_count: usize,
}

trait WorryManagement {
    fn manage(&self, worry: usize) -> usize;
}

struct WorryManagementDivision {
    divisor: usize,
}

impl WorryManagement for WorryManagementDivision {
    fn manage(&self, worry: usize) -> usize {
        worry / self.divisor
    }
}

struct WorryManagementModulo {
    modulo: usize,
}

impl WorryManagement for WorryManagementModulo {
    fn manage(&self, worry: usize) -> usize {
        worry % self.modulo
    }
}

struct MonkeyReader {
    index_regex: Regex,
    items_regex: Regex,
    operation_regex: Regex,
    divisibility_regex: Regex,
    when_true_regex: Regex,
    when_false_regex: Regex,
}

impl MonkeyReader {
    fn new() -> Self {
        Self {
            index_regex: Regex::new(r"Monkey (\d+):").unwrap(),
            items_regex: Regex::new(r" {2}Starting items: (.*)").unwrap(),
            operation_regex: Regex::new(r" {2}Operation: new = (.*)").unwrap(),
            divisibility_regex: Regex::new(r" {2}Test: divisible by (\d+)").unwrap(),
            when_true_regex: Regex::new(r" {4}If true: throw to monkey (\d+)").unwrap(),
            when_false_regex: Regex::new(r" {4}If false: throw to monkey (\d+)").unwrap(),
        }
    }

    fn read(&self, input: &str) -> (usize, Monkey) {
        let mut lines = input.lines();
        let index = self.index_regex.captures(lines.next().unwrap()).unwrap()
            .get(1).unwrap().as_str().parse().unwrap();
        let items = self.items_regex.captures(lines.next().unwrap()).unwrap()
            .get(1).unwrap().as_str().split(", ")
            .map(|i| i.parse().unwrap()).collect();
        let operation = Expr::new(
            self.operation_regex.captures(lines.next().unwrap()).unwrap()
                .get(1).unwrap().as_str());
        let divisibility_test = self.divisibility_regex.captures(lines.next().unwrap()).unwrap()
            .get(1).unwrap().as_str().parse().unwrap();
        let when_true = self.when_true_regex.captures(lines.next().unwrap()).unwrap()
            .get(1).unwrap().as_str().parse().unwrap();
        let when_false = self.when_false_regex.captures(lines.next().unwrap()).unwrap()
            .get(1).unwrap().as_str().parse().unwrap();
        (index, Monkey {
            items,
            operation,
            divisibility_test,
            when_true,
            when_false,
            inspection_count: 0
        })
    }
}

impl Monkey {
    fn evaluate_next(&mut self, worry_management: &Box<dyn WorryManagement>) -> (usize, usize) {
        self.inspection_count += 1;
        let item = self.items.remove(0);
        let worry = worry_management.manage(self.operation.clone().value("old", item).exec().unwrap().as_u64().unwrap() as usize);
        (worry, if worry % self.divisibility_test == 0 { self.when_true } else { self.when_false })
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn round(&mut self, worry_management: &Box<dyn WorryManagement>) {
        for i in 0..self.monkeys.len() {
            while !self.monkeys[i].items.is_empty() {
                let (worry, to) = self.monkeys[i].evaluate_next(worry_management);
                self.monkeys[to].items.push(worry);
            }
        }
    }
}

pub struct Advent2022Day11Solver {
    monkeys: Vec<Monkey>,
}

impl Advent2022Day11Solver {
    pub fn new(input: String) -> Self {
        let reader = MonkeyReader::new();
        Self {
            monkeys: input
                .split("\n\n")
                .map(|l| reader.read(l))
                .sorted_by(|a,b| usize::cmp(&a.0, &b.0))
                .map(|(_,m)| m)
                .collect()
        }
    }

    fn solve(&self, iterations: usize, worry_management: &Box<dyn WorryManagement>) -> usize {
        let mut monkeys = Monkeys { monkeys: self.monkeys.clone() };
        (0..iterations).for_each(|_| monkeys.round(worry_management));
        monkeys.monkeys
            .iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .fold(1, |acc, cur| acc * cur)
    }
}

impl AdventSolver for Advent2022Day11Solver {
    fn day(&self) -> usize { 11 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        let worry_management: Box<dyn WorryManagement> = Box::new(WorryManagementDivision { divisor: 3 });
        self.solve(20, &worry_management)
    }

    fn solve_part2(&self) -> usize {
        let worry_management: Box<dyn WorryManagement> =
            Box::new(WorryManagementModulo { modulo: self.monkeys.iter().fold(1, |a, c| a * c.divisibility_test) });
        self.solve(10000, &worry_management)
    }
}
