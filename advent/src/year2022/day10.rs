use crate::solver::AdventSolver;

trait Operation {
    fn execute(&self, cpu: &mut CPU);
}

struct Addx {
    value: isize,
}

impl Operation for Addx {
    fn execute(&self, cpu: &mut CPU) {
        cpu.advance();
        cpu.x += self.value;
        cpu.advance();
    }
}

struct Noop {}

impl Operation for Noop {
    fn execute(&self, cpu: &mut CPU) {
        cpu.advance();
    }
}

pub struct Advent2022Day10Solver {
    operations: Vec<Box<dyn Operation>>,
}

struct CPU {
    xs: Vec<isize>,
    cycle: usize,
    x: isize,
}

impl CPU {
    fn new() -> Self {
        Self { xs: vec!(1), cycle: 0, x: 1 }
    }

    fn advance(&mut self) {
        self.cycle += 1;
        self.xs.push(self.x);
    }

    fn signal_strength(&self, cycle: usize) -> isize {
        self.xs[cycle - 1] * cycle as isize
    }

    fn is_visible(&self, cycle: usize, column: usize) -> bool {
        let x = self.xs[cycle];
        x == column as isize - 1 || x == column as isize || x == column as isize + 1
    }
}

impl Advent2022Day10Solver {
    pub fn new(input: String) -> Self {
        Self {
            operations: input
                .lines()
                .map(|l| -> Box<dyn Operation> {
                    if l == "noop" { return Box::new(Noop {}); }
                    let mut s = l.split(" ");
                    s.next();
                    Box::new(Addx { value: s.next().unwrap().parse().unwrap() })
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day10Solver {
    fn day(&self) -> usize { 10 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        let mut cpu = CPU::new();
        self.operations
            .iter()
            .for_each(|o| o.execute(&mut cpu));
        (0..6).map(|i| dbg!(cpu.signal_strength(dbg!(i * 40 + 20))))
            .sum::<isize>()
            as usize
    }

    fn solve_part2_string(&self) -> String {
        let mut cpu = CPU::new();
        self.operations
            .iter()
            .for_each(|o| o.execute(&mut cpu));
        (0..6).map(|row|
            (0..40).map(|col| if cpu.is_visible(row * 40 + col, col) { '#' } else { '.' }).collect::<String>() + "\n")
            .collect::<String>()
    }
}
