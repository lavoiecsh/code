use std::collections::HashMap;

use crate::solver::AdventSolver;

pub struct Advent2017Day03Solver {
    data: usize,
}

impl Advent2017Day03Solver {
    pub fn new(input: String) -> Self {
        Self { data: input.parse().unwrap() }
    }
}

impl AdventSolver for Advent2017Day03Solver {
    fn solve_part1(&self) -> usize {
        let mut ring_number = 0;
        let mut ring_first = 0;
        let mut ring_last = 1;
        while ring_last < self.data {
            ring_number += 1;
            ring_first = ring_last + 1;
            ring_last += ring_number * 8;
        }
        let mut diff = self.data - ring_first;
        while diff > ring_number * 2 {
            diff -= ring_number * 2;
        }
        let center = ring_number - 1;
        let to_center = if diff > center { diff - center } else { center - diff };
        to_center + ring_number
    }

    fn solve_part2(&self) -> usize {
        let mut numbers: HashMap<(i32,i32), usize> = HashMap::new();
        let mut number = 1;
        let mut pos = (0, 0);
        numbers.insert((0,0), number);
        let mut ring_number = 0;
        let mut ring_last = 1;
        let mut ring_first = 0;
        let mut ring_current = 1;
        while number < self.data {
            if ring_current == ring_last {
                ring_first = ring_last + 1;
                ring_number += 1;
                ring_last += ring_number * 8;
                pos = (pos.0 + 1, pos.1);
                ring_current = ring_first;
            } else {
                if (ring_current - ring_first) < ring_number * 2 - 1 {
                    pos = (pos.0, pos.1 - 1);
                } else if (ring_current - ring_first) < ring_number * 4 - 1 {
                    pos = (pos.0 - 1, pos.1);
                } else if (ring_current - ring_first) < ring_number * 6 - 1 {
                    pos = (pos.0, pos.1 + 1);
                } else {
                    pos = (pos.0 + 1, pos.1);
                }
                ring_current += 1;
            }
            number = [-1, 0, 1].iter()
                .map(|y| [-1, 0, 1].iter().map(|x| numbers.get(&(pos.0 + x, pos.1 + y)).unwrap_or(&0)).sum::<usize>())
                .sum::<usize>();
            numbers.insert(pos, number);
        }
        number
    }
}
