use crate::solver::AdventSolver;

pub struct Advent2024Day02Solver {
    reports: Vec<Vec<u32>>,
}

impl Advent2024Day02Solver {
    pub fn new(input: &str) -> Self {
        Self {
            reports: input
                .lines()
                .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2024Day02Solver {
    fn solve_part1(&self) -> usize {
        self.reports.iter().filter(|r| is_safe(r)).count()
    }

    fn solve_part2(&self) -> usize {
        self.reports
            .iter()
            .filter(|r| is_safe(r) || is_safe_with_removed(r))
            .count()
    }
}

fn is_safe(report: &[u32]) -> bool {
    let increasing = report[0] < report[1];
    (1..report.len())
        .all(|i|
            if increasing { report[i - 1] < report[i] } else { report[i - 1] > report[i] } &&
                report[i-1].abs_diff(report[i]) <= 3)
}

fn is_safe_with_removed(report: &[u32]) -> bool {
    (0..report.len()).any(|i| {
        let mut clone = report.to_vec();
        clone.remove(i);
        is_safe(&clone)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn finds_safe_reports() {
        let solver = Advent2024Day02Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 2);
    }

    #[test]
    fn finds_safe_reports_after_removing_level() {
        let solver = Advent2024Day02Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 4);
    }
}
