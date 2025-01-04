use itertools::Itertools;
use crate::solver::AdventSolver;
use regex::{Match, Regex};

pub struct Advent2024Day13Solver {
    claw_machines: Vec<ClawMachine>,
}

impl Advent2024Day13Solver {
    pub fn new(input: &str) -> Self {
        let button_a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let button_b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let mut claw_machines = Vec::new();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        let lines = input.lines().collect_vec();
        for i in (0..lines.len()).step_by(4) {
            let cap_a = button_a_re.captures(lines[i]).unwrap();
            let cap_b = button_b_re.captures(lines[i+1]).unwrap();
            let cap_prize = prize_re.captures(lines[i+2]).unwrap();
            let pos_a = Pos::new(parse(cap_a.get(1)), parse(cap_a.get(2)));
            let pos_b = Pos::new(parse(cap_b.get(1)), parse(cap_b.get(2)));
            let prize = Pos::new(parse(cap_prize.get(1)), parse(cap_prize.get(2)));
            claw_machines.push(ClawMachine::new(pos_a, pos_b, prize));
        }
        Self { claw_machines }
    }
}

impl AdventSolver for Advent2024Day13Solver {
    fn solve_part1(&self) -> usize {
        self.claw_machines
            .iter()
            .filter_map(|cm| cm.solution())
            .map(|(a,b)| a * 3 + b)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.claw_machines
            .iter()
            .map(|cm| cm.move_prize())
            .filter_map(|cm| cm.solution())
            .map(|(a, b)| a * 3 + b)
            .sum()
    }
}

struct ClawMachine {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}

impl ClawMachine {
    fn new(button_a: Pos, button_b: Pos, prize: Pos) -> Self {
        Self { button_a, button_b, prize }
    }

    fn move_prize(&self) -> Self {
        Self {
            button_a: self.button_a.clone(),
            button_b: self.button_b.clone(),
            prize: self.prize.move_away(),
        }
    }

    fn solution(&self) -> Option<(usize, usize)> {
        // px = ax * a + bx * b
        // py = ay * a + by * b
        // by * b = py - ay * a
        // b = (py - ay * a) / by
        // px = ax * a + bx * (py - ay * a) / by
        // by * px = by * ax * a + bx * (py - ay * a)
        // by * px = by * ax * a + bx * py - bx * ay * a
        // bx * ay * a - by * ax * a = bx * py - by * px
        // a * ( bx * ay - by * ax ) = (bx * py - by * px)
        // a = (bx_py - by_px) / (ay_bx - ax_by)
        let bx_py = self.button_b.x * self.prize.y;
        let by_px = self.button_b.y * self.prize.x;
        let ax_by = self.button_a.x * self.button_b.y;
        let ay_bx = self.button_a.y * self.button_b.x;
        if by_px > bx_py && ax_by < ay_bx {
            return None;
        }
        let r = bx_py.abs_diff(by_px);
        let l = ax_by.abs_diff(ay_bx);
        if r % l != 0 {
            return None;
        }
        let a = r / l;
        let b = self.prize.y - self.button_a.y * a;
        if b % self.button_b.y != 0 {
            return None;
        }
        Some((a, b / self.button_b.y))
    }
}

#[derive(Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_away(&self) -> Self {
        Self { x: self.x + 10000000000000, y : self.y + 10000000000000 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279

";

    #[test]
    fn finds_fewest_tokens_to_win_prizes() {
        let solver = Advent2024Day13Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 480);
    }
}
