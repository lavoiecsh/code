use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};

pub struct Advent2024Day20Solver {
    racetrack: Racetrack,
}

impl Advent2024Day20Solver {
    pub fn new(input: &str) -> Self {
        Self {
            racetrack: Racetrack::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2024Day20Solver {
    fn solve_part1(&self) -> usize {
        self.racetrack.cheats_saving_at_least(100, 2)
    }

    fn solve_part2(&self) -> usize {
        self.racetrack.cheats_saving_at_least(100, 20)
    }
}

type Pos = (usize, usize);

struct Racetrack {
    map: Vec<Vec<char>>,
    times: HashMap<Pos, usize>,
    start: Pos,
    height: usize,
    width: usize,
}

impl Racetrack {
    fn new(map: Vec<Vec<char>>) -> Self {
        Self {
            start: map
                .iter()
                .enumerate()
                .filter_map(|(y, r)| {
                    r.iter()
                        .enumerate()
                        .find(|(_, &c)| c == 'S')
                        .map(|(x, _)| (y, x))
                })
                .next()
                .unwrap(),
            height: map.len() - 1,
            width: map[0].len() - 1,
            map,
            times: HashMap::new(),
        }
        .build()
    }

    fn build(mut self) -> Self {
        self.times.insert(self.start, 0);
        let mut queue = VecDeque::new();
        queue.push_back(self.start);
        while let Some(current) = queue.pop_front() {
            let next_time = self.times.get(&current).unwrap() + 1;
            self.around(current)
                .filter(|p| self.is_open(*p))
                .filter(|p| self.times.get(p).map_or(true, |&t| t > next_time))
                .collect_vec()
                .into_iter()
                .for_each(|p| {
                    self.times.insert(p, next_time);
                    queue.push_back(p);
                });
        }
        self
    }

    fn cheats_saving_at_least(&self, threshold: usize, max_cheat_length: usize) -> usize {
        let mut count = 0;
        for (start, start_time) in &self.times {
            for (end, end_time) in &self.times {
                if start == end {
                    continue;
                }
                let distance = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);
                if distance > max_cheat_length {
                    continue;
                }
                let time_save = end_time.saturating_sub(start_time + distance);
                if time_save >= threshold {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_open(&self, pos: Pos) -> bool {
        !self.is_wall(pos)
    }

    fn is_wall(&self, pos: Pos) -> bool {
        self.map[pos.0][pos.1] == '#'
    }

    fn around(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let mut a = Vec::new();
        if pos.0 > 0 {
            a.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.height {
            a.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            a.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.width {
            a.push((pos.0, pos.1 + 1));
        }
        a.into_iter()
    }
}

impl Debug for Racetrack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut map = String::with_capacity((self.width + 2) * (self.height + 2) * 3);
        map.push('\n');
        for y in 0..=self.height {
            for x in 0..=self.width {
                if self.is_wall((y, x)) {
                    map.push_str(" ##");
                } else {
                    map.push_str(&format!(" {:02}", self.times.get(&(y, x)).unwrap()));
                }
            }
            map.push('\n');
        }
        f.write_str(&map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn counts_cheats_saving_time() {
        let solver = Advent2024Day20Solver::new(EXAMPLE);
        assert_eq!(solver.racetrack.cheats_saving_at_least(10, 2), 10);
    }

    #[test]
    fn counts_big_cheats_saving_time() {
        let solver = Advent2024Day20Solver::new(EXAMPLE);
        assert_eq!(solver.racetrack.cheats_saving_at_least(50, 20), 285);
    }
}
