use crate::solver::AdventSolver;

pub struct Advent2022Day25Solver {
    fuel_requirements: Vec<String>
}

impl Advent2022Day25Solver {
    pub fn new(input: String) -> Self {
        Self {
            fuel_requirements: input
                .lines()
                .map(String::from)
                .collect()
        }
    }
}

fn snafu_to_decimal(snafu: &String) -> isize {
    let mut decimal = 0;
    let mut power = 1;
    for c in snafu.chars().rev() {
        decimal += power * match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown snafu digit"),
        };
        power *= 5;
    }
    decimal
}

fn decimal_to_snafu(decimal: isize) -> String {
    let mut d = decimal;
    let mut snafu = String::new();
    while d != 0 {
        snafu.insert(0, match d % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => { d += 5; '=' },
            4 => { d += 5; '-' },
            _ => panic!("unknown modulo"),
        });
        d /= 5;
    }
    snafu
}

impl AdventSolver for Advent2022Day25Solver {
    fn day(&self) -> usize { 25 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1_string(&self) -> String {
        decimal_to_snafu(self.fuel_requirements.iter().map(snafu_to_decimal).sum())
    }
}
