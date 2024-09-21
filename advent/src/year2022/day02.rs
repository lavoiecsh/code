use crate::solver::AdventSolver;

pub struct Advent2022Day02Solver {
    rounds: Vec<(char, char)>
}

impl Advent2022Day02Solver {
    pub fn new(input: String) -> Self {
        Self {
            rounds: input
                .trim()
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .map(|c| (c[0], c[2]))
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day02Solver {
    fn solve_part1(&self) -> usize {
        self.rounds
            .iter()
            .map(convert_part1)
            .map(score)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.rounds
            .iter()
            .map(convert_part2)
            .map(score)
            .sum()
    }
}

fn convert_part1(round: &(char, char)) -> (char, char) {
    (round.0, match round.1 { 'X' => 'A', 'Y' => 'B', 'Z' => 'C', _ => panic!("unknown") })
}

fn convert_part2(round: &(char, char)) -> (char, char) {
    (round.0, match round {
        ('A','X') => 'C',
        ('A','Y') => 'A',
        ('A','Z') => 'B',
        ('B','X') => 'A',
        ('B','Y') => 'B',
        ('B','Z') => 'C',
        ('C','X') => 'B',
        ('C','Y') => 'C',
        ('C','Z') => 'A',
        _ => panic!("unknown")
    })
}

fn score(round: (char, char)) -> usize {
    match round {
        ('A','A') => 1 + 3,
        ('A','B') => 2 + 6,
        ('A','C') => 3,
        ('B','A') => 1,
        ('B','B') => 2 + 3,
        ('B','C') => 3 + 6,
        ('C','A') => 1 + 6,
        ('C','B') => 2,
        ('C','C') => 3 + 3,
        _ => 0
    }
}
