use crate::solver::AdventSolver;

pub struct Advent2019Day04Solver {
    lower_bound: usize,
    upper_bound: usize,
}

impl Advent2019Day04Solver {
    pub fn new(input: &str) -> Self {
        let split = input.split_once('-').unwrap();
        Self {
            lower_bound: split.0.parse().unwrap(),
            upper_bound: split.1.parse().unwrap(),
        }
    }
}

impl AdventSolver for Advent2019Day04Solver {
    fn solve_part1(&self) -> usize {
        (self.lower_bound..=self.upper_bound)
            .filter(|&n| is_valid(n, false))
            .count()
    }

    fn solve_part2(&self) -> usize {
        (self.lower_bound..=self.upper_bound)
            .filter(|&n| is_valid(n, true))
            .count()
    }
}

fn is_valid(number: usize, pair_only: bool) -> bool {
    let mut digit_counts = vec![0; 10];
    let mut previous = 10;
    let mut number = number;
    while number > 0 {
        let d = number % 10;
        if d > previous {
            return false;
        }
        digit_counts[d] += 1;
        number /= 10;
        previous = d;
    }
    digit_counts.iter().sum::<usize>() == 6
        && digit_counts.iter().any(|&d| if pair_only { d == 2 } else { d >= 2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_valid_passwords() {
        assert!(is_valid(111111, false));
        assert!(!is_valid(223450, false));
        assert!(!is_valid(123789, false));

        assert!(is_valid(112233, true));
        assert!(!is_valid(123444, true));
        assert!(is_valid(111122, true));
        assert!(is_valid(111223, true));

        assert!(!is_valid(11112, false));
        assert!(!is_valid(1123456, false));
    }
}
