use std::collections::VecDeque;

use crate::solver::AdventSolver;

pub struct Advent2017Day17Solver {
    steps: usize,
}

impl Advent2017Day17Solver {
    pub fn new(input: String) -> Self {
        Self { steps: input.parse().unwrap() }
    }
}

impl AdventSolver for Advent2017Day17Solver {
    fn solve_part1(&self) -> usize {
        let mut spin_lock = SpinLock::new(self.steps);
        spin_lock.insert(2017);
        spin_lock.buffer[1]
    }

    fn solve_part2(&self) -> usize {
        let mut spin_lock = SpinLockCounter::new(self.steps);
        spin_lock.insert(50000000);
        spin_lock.after_zero
    }
}

struct SpinLockCounter {
    count: usize,
    position: usize,
    steps: usize,
    after_zero: usize,
}

impl SpinLockCounter {
    fn new(steps: usize) -> Self {
        Self { count: 1, steps, position: 0, after_zero: 0 }
    }

    fn insert(&mut self, times: usize) {
        (1..=times)
            .for_each(|value| self.insert_one(value));
    }

    fn insert_one(&mut self, value: usize) {
        self.position = (self.position + self.steps) % self.count;
        self.count += 1;
        self.position += 1;
        if self.position == 1 {
            self.after_zero = value;
        }
    }
}

struct SpinLock {
    buffer: VecDeque<usize>,
    steps: usize,
}

impl SpinLock {
    fn new(steps: usize) -> Self {
        Self { buffer: vec!(0).iter().cloned().collect(), steps }
    }

    fn insert(&mut self, times: usize) {
        (1..=times)
            .for_each(|value| self.insert_one(value));
    }

    fn insert_one(&mut self, value: usize) {
        (0..=self.steps)
            .for_each(|_| {
                let tmp = self.buffer.pop_front().unwrap();
                self.buffer.push_back(tmp);
            });
        self.buffer.push_front(value);
    }
}
