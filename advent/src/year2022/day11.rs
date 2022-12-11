use crate::solver::AdventSolver;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(old: usize) -> usize,
    divisibility_test: usize,
    when_true: usize,
    when_false: usize,
    inspection_count: usize,
}

impl Monkey {
    pub fn new(
        items: Vec<usize>,
        operation: fn (usize) -> usize,
        divisibility_test: usize,
        when_true: usize,
        when_false: usize,
    ) -> Self {
        Self { items, operation, divisibility_test, when_true, when_false, inspection_count: 0 }
    }

    fn evaluate_next(&mut self, worry_management: fn (usize) -> usize) -> (usize, usize) {
        self.inspection_count += 1;
        let item = self.items.remove(0);
        let worry = worry_management((self.operation)(item));
        (worry, if worry % self.divisibility_test == 0 { self.when_true } else { self.when_false })
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn round(&mut self, worry_management: fn (usize) -> usize) {
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
    pub fn new() -> Self {
        Self {
            monkeys: vec!(
                Monkey::new(vec!(54, 61, 97, 63, 74), |old| old * 7, 17, 5, 3),
                Monkey::new(vec!(61, 70, 97, 64, 99, 83, 52, 87), |old| old + 8, 2, 7, 6),
                Monkey::new(vec!(60, 67, 80, 65), |old| old * 13, 5, 1, 6),
                Monkey::new(vec!(61, 70, 76, 69, 82, 56), |old| old + 7, 3, 5, 2),
                Monkey::new(vec!(79, 98), |old| old + 2, 7, 0, 3),
                Monkey::new(vec!(72, 79, 55), |old| old + 1, 13, 2, 1),
                Monkey::new(vec!(63), |old| old + 4, 19, 7, 4),
                Monkey::new(vec!(72, 51, 93, 63, 80, 86, 81), |old| old * old, 11, 0, 4),
            )
        }
    }
}

impl AdventSolver for Advent2022Day11Solver {
    fn day(&self) -> usize { 11 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        let worry_management: fn (usize) -> usize = |worry| worry / 3;
        let mut monkeys = Monkeys { monkeys: self.monkeys.clone() };
        (0..20).for_each(|_| monkeys.round(worry_management));
        let mut inspection_counts: Vec<usize> = monkeys.monkeys
            .iter()
            .map(|m| m.inspection_count)
            .collect();
        inspection_counts.sort();
        inspection_counts[inspection_counts.len() - 1] * inspection_counts[inspection_counts.len() - 2]
    }

    fn solve_part2(&self) -> usize {
        // product of all divisibility_tests
        let worry_management: fn (usize) -> usize = |worry| worry % 9699690;
        let mut monkeys = Monkeys { monkeys: self.monkeys.clone() };
        (0..10000).for_each(|_| monkeys.round(worry_management));
        let mut inspection_counts: Vec<usize> = monkeys.monkeys
            .iter()
            .map(|m| m.inspection_count)
            .collect();
        inspection_counts.sort();
        inspection_counts[inspection_counts.len() - 1] * inspection_counts[inspection_counts.len() - 2]
    }
}
