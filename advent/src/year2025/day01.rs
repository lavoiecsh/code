use crate::solver::AdventSolver;

pub struct Advent2025Day01Solver {
    rotations: Vec<Rotation>,
}

impl Advent2025Day01Solver {
    pub fn new(input: &str) -> Self {
        Self {
            rotations: input
                .lines()
                .map(|l| l.split_at(1))
                .map(|(d, a)| Rotation {
                    right: d == "R",
                    amount: a.parse::<i32>().unwrap(),
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2025Day01Solver {
    fn solve_part1(&self) -> usize {
        let mut dial = Dial::new();
        for rotation in &self.rotations {
            dial.rotate(&rotation);
        }
        dial.finished
    }

    fn solve_part2(&self) -> usize {
        let mut dial = Dial::new();
        for rotation in &self.rotations {
            dial.rotate(&rotation);
        }
        dial.finished + dial.passed
    }
}

struct Rotation {
    right: bool,
    amount: i32,
}

impl Rotation {
    fn full(&self) -> usize {
        (self.amount / 100) as usize
    }

    fn partial(&self) -> i32 {
        self.amount % 100 * if self.right { 1 } else { -1 }
    }
}

struct Dial {
    pointing: i32,
    finished: usize,
    passed: usize,
}

impl Dial {
    fn new() -> Self {
        Self {
            pointing: 50,
            finished: 0,
            passed: 0,
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        self.passed += rotation.full();
        let partial = rotation.partial();
        if partial < 0 && self.pointing == 0 {
            self.passed -= 1;
        }
        self.pointing += partial;
        if self.pointing > 100 {
            self.pointing -= 100;
            self.passed += 1;
        }
        if self.pointing < 0 {
            self.pointing += 100;
            self.passed += 1;
        }
        if self.pointing == 100 {
            self.pointing -= 100;
        }
        if self.pointing == 0 {
            self.finished += 1;
        }
    }
}
