use crate::solver::AdventSolver;
use std::collections::{HashMap, HashSet};

pub struct Advent2024Day08Solver {
    grid: Grid,
}

impl Advent2024Day08Solver {
    pub fn new(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut antennae = HashMap::new();
        input.lines().enumerate().for_each(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .for_each(|(x, c)| {
                    antennae
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((y as isize, x as isize))
                })
        });
        Self {
            grid: Grid {
                height,
                width,
                antennae,
            },
        }
    }
}

impl AdventSolver for Advent2024Day08Solver {
    fn solve_part1(&self) -> usize {
        self.grid.antinodes(false).len()
    }

    fn solve_part2(&self) -> usize {
        self.grid.antinodes(true).len()
    }
}

type Pos = (isize, isize);

struct Grid {
    height: usize,
    width: usize,
    antennae: HashMap<char, Vec<Pos>>,
}

impl Grid {
    fn antinodes(&self, all: bool) -> HashSet<Pos> {
        let mut antinodes = HashSet::new();
        for group in self.antennae.values() {
            if all {
                antinodes.extend(group.iter());
            }
            for i in 0..group.len() {
                for j in 0..group.len() {
                    if i == j {
                        continue;
                    }
                    antinodes.extend(self.antinodes_for(all, &group[i], &group[j]));
                }
            }
        }
        antinodes
    }

    fn antinodes_for(&self, all: bool, a: &Pos, b: &Pos) -> Vec<Pos> {
        let mut ans = Vec::new();
        let h = self.height as isize;
        let w = self.width as isize;
        let diff_y = b.0 - a.0;
        let diff_x = b.1 - a.1;
        let mut y = b.0 + diff_y;
        let mut x = b.1 + diff_x;
        while y >= 0 && y < h && x >= 0 && x < w {
            ans.push((y, x));
            if !all {
                break;
            }
            y += diff_y;
            x += diff_x;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn finds_antinodes() {
        let solver = Advent2024Day08Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 14);
    }

    #[test]
    fn finds_all_antinodes() {
        let solver = Advent2024Day08Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 34);
    }
}
