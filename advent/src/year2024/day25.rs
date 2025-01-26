use crate::solver::AdventSolver;
use itertools::Itertools;

pub struct Advent2024Day25Solver {
    pins: Vec<Pins>,
}

impl Advent2024Day25Solver {
    pub fn new(input: &str) -> Self {
        Self {
            pins: input
                .lines()
                .chunks(8)
                .into_iter()
                .map(|c| c.collect_vec())
                .map(Pins::from)
                .collect(),
        }
    }

    fn keys(&self) -> Vec<Pins> {
        self.pins.iter().filter(|p| p.is_key).cloned().collect()
    }

    fn locks(&self) -> Vec<Pins> {
        self.pins.iter().filter(|p| !p.is_key).cloned().collect()
    }
}

impl AdventSolver for Advent2024Day25Solver {
    fn solve_part1(&self) -> usize {
        let keys = self.keys();
        let locks = self.locks();
        keys.into_iter().cartesian_product(locks).filter(|(k,l)| k.fits_with(l)).count()
    }
}

#[derive(Clone)]
struct Pins {
    pins: Vec<u8>,
    is_key: bool,
}

impl Pins {
    fn fits_with(&self, other: &Self) -> bool {
        self.is_key != other.is_key && (0..5).all(|p| self.pins[p] + other.pins[p] <= 5)
    }
}

impl From<Vec<&str>> for Pins {
    fn from(value: Vec<&str>) -> Self {
        let is_key = value[0] == ".....";
        let pins = (0..5)
            .map(|c| {
                value
                    .iter()
                    .skip(1)
                    .take(5)
                    .filter(|r| r.chars().nth(c).unwrap() == '#')
                    .count() as u8
            })
            .collect();
        Self { pins, is_key }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn counts_lock_key_pairs_that_could_fit() {
        let solver = Advent2024Day25Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 3);
    }
}
