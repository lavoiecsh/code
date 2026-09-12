use std::collections::HashMap;
use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};
use itertools::iproduct;
use std::ops::Range;

pub struct Advent2019Day19Solver {
    program: Vec<Value>,
}

impl Advent2019Day19Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day19Solver {
    fn solve_part1(&self) -> usize {
        let detector = TractorBeamDetector::new(self.program.clone());
        detector.count_50()
    }

    fn solve_part2(&self) -> usize {
        let detector = TractorBeamDetector::new(self.program.clone());
        let (x, y) = detector.closest_fitting();
        x * 10000 + y
    }
}

type Pos = (usize, usize);
struct TractorBeamDetector {
    program: Vec<Value>,
}

impl TractorBeamDetector {
    fn new(program: Vec<Value>) -> Self {
        Self {
            program,
        }
    }

    fn count_50(&self) -> usize {
        iproduct!(0..50, 0..50)
            .filter(|p| self.is_affected(p))
            .count()
    }

    // RustRover incorrectly marks ranges variable as not needing mutability,
    // because the mutability is used through the get_range! macro
    //noinspection RsUnusedMut
    fn closest_fitting(&self) -> Pos {
        let mut ranges: HashMap<usize, Range<usize>> = HashMap::new();
        macro_rules! get_range {
            ($x:expr) => { ranges.entry($x).or_insert_with(|| self.affected_range_x($x)) }
        }
        let range_100 = get_range!(100);
        let min_x = 200 * 100 / range_100.count();

        for x in min_x.. {
            let bottom_left = (x + 99, get_range!(x + 99).start);
            let top_right = (x, get_range!(x).end - 1);
            if top_right.1 >= bottom_left.1 + 99 {
                return (top_right.0, bottom_left.1);
            }
        }
        unreachable!("no solution found")
    }

    fn affected_range_x(&self, x: usize) -> Range<usize> {
        let mut start = None;
        for y in 0..10000 {
            let is_affected = self.is_affected(&(x, y));
            if start.is_none() && is_affected {
                start = Some(y);
            }
            if start.is_some() && !is_affected {
                return start.unwrap()..y;
            }
        }
        unreachable!("no range found for {x}")
    }

    fn is_affected(&self, pos: &Pos) -> bool {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(pos.0 as Value);
        computer.send_input(pos.1 as Value);
        computer.run();
        computer.receive_output().expect("affected not output") == 1
    }
}
