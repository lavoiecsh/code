use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day06Solver {
    races: Vec<Race>,
}

impl Advent2023Day06Solver {
    pub fn new(input: String) -> Self {
        let mut lines = input.lines();
        let times: Vec<usize> = lines.next().unwrap().split_ascii_whitespace().skip(1).map(|t| t.parse().unwrap()).collect();
        let distances: Vec<usize> = lines.next().unwrap().split_ascii_whitespace().skip(1).map(|d| d.parse().unwrap()).collect();
        Self {
            races: times.into_iter()
                .zip(distances)
                .map(|(time,distance)| Race { time, distance })
                .collect()
        }
    }
}

impl AdventSolver for Advent2023Day06Solver {
    fn solve_part1(&self) -> usize {
        self.races.iter()
            .map(|r| r.ways_to_beat())
            .product::<usize>()
    }

    fn solve_part2(&self) -> usize {
        Race {
            time: self.races.iter().map(|r| r.time).join("").parse().unwrap(),
            distance: self.races.iter().map(|r| r.distance).join("").parse().unwrap(),
        }
            .ways_to_beat()
    }
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn ways_to_beat(&self) -> usize {
        (0..self.time)
            .map(|h| h * (self.time - h))
            .filter(|d| d > &self.distance)
            .count()
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day06Solver {
    Advent2023Day06Solver::new(String::from("\
Time:      7  15   30
Distance:  9  40  200
"))
}

#[test]
fn ways_to_beat_split_records() {
    let solver = test_solver_1();
    let ways: Vec<usize> = solver.races.iter().map(|r| r.ways_to_beat()).collect();
    assert_eq!(ways, vec!(4, 8, 9));
    assert_eq!(solver.solve_part1(), 288);
}

#[test]
fn ways_to_beat_single_record() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 71503);
}
