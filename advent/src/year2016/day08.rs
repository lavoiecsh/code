use crate::solver::AdventSolver;
use regex::Regex;

pub struct Advent2016Day08Solver {
    operations: Vec<Operation>,
}

impl Advent2016Day08Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        let rr = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
        let rc = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
        Self {
            operations: input
                .lines()
                .map(|l| {
                    if let Some(captures) = re.captures(l) {
                        let values: Vec<&str> = captures
                            .iter()
                            .skip(1)
                            .map(|c| c.unwrap().as_str())
                            .collect();
                        Operation::Rectangle(values[0].parse().unwrap(), values[1].parse().unwrap())
                    } else if let Some(captures) = rr.captures(l) {
                        let values: Vec<&str> = captures
                            .iter()
                            .skip(1)
                            .map(|c| c.unwrap().as_str())
                            .collect();
                        Operation::RotateRow(values[0].parse().unwrap(), values[1].parse().unwrap())
                    } else if let Some(captures) = rc.captures(l) {
                        let values: Vec<&str> = captures
                            .iter()
                            .skip(1)
                            .map(|c| c.unwrap().as_str())
                            .collect();
                        Operation::RotateCol(values[0].parse().unwrap(), values[1].parse().unwrap())
                    } else {
                        panic!("unknown operation: {l}")
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2016Day08Solver {
    fn solve_part1(&self) -> usize {
        let mut screen = Screen::new();
        self.operations.iter().for_each(|o| screen.execute(o));
        screen.lit_pixels()
    }

    fn solve_part2_string(&self) -> String {
        let mut screen = Screen::new();
        self.operations.iter().for_each(|o| screen.execute(o));
        format!("\n{}", screen.pretty_print())
    }
}

enum Operation {
    Rectangle(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: (0..6).map(|_| (0..50).map(|_| false).collect()).collect(),
        }
    }

    fn execute(&mut self, operation: &Operation) {
        match operation {
            Operation::Rectangle(x, y) => {
                (0..*y).for_each(|iy| (0..*x).for_each(|ix| self.pixels[iy][ix] = true));
            }
            Operation::RotateRow(y, a) => {
                self.pixels[*y].rotate_right(*a);
            }
            Operation::RotateCol(x, a) => {
                let mut values: Vec<bool> = (0..self.pixels.len())
                    .map(|iy| self.pixels[iy][*x])
                    .collect();
                values.rotate_right(*a);
                (0..self.pixels.len()).for_each(|iy| self.pixels[iy][*x] = values[iy]);
            }
        }
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|r| r.iter().filter(|v| **v).count())
            .sum()
    }

    fn pretty_print(&self) -> String {
        self.pixels
            .iter()
            .map(|r| r.iter().map(|p| if *p { '#' } else { ' ' }).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
