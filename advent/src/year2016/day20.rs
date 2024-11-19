use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2016Day20Solver {
    blacklist: Vec<(u32, u32)>,
}

impl Advent2016Day20Solver {
    pub fn new(input: &str) -> Self {
        Self {
            blacklist: input
                .lines()
                .map(|l| {
                    l.split("-")
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .sorted_by(|l: &(u32, u32), r: &(u32, u32)| l.0.cmp(&r.0))
                .collect(),
        }
    }

    fn lowest_unblocked(&self) -> u32 {
        self.blacklist.iter().fold(
            0,
            |acc, (s, e)| if acc >= *s && acc <= *e { e + 1 } else { acc },
        )
    }

    fn unblocked_count(&self) -> u32 {
        let mut windows: Vec<(u32, u32)> = Vec::new();
        for block in &self.blacklist {
            let start_included = windows
                .iter()
                .find_position(|w| block.0 >= w.0 && block.0 <= w.1);
            let end_included = windows
                .iter()
                .find_position(|w| block.1 >= w.0 && block.1 <= w.1);
            match (start_included, end_included) {
                (Some((si, s)), Some((ei, e))) => {
                    if si != ei {
                        let new_window = (s.0, e.1);
                        windows.remove(usize::max(si, ei));
                        windows.remove(usize::min(si, ei));
                        windows.push(new_window);
                    }
                }
                (Some((si, _)), None) => windows[si].1 = block.1,
                (None, Some((ei, _))) => windows[ei].0 = block.0,
                (None, None) => windows.push(*block),
            }
        }
        windows.sort_by(|l, r| l.0.cmp(&r.0));
        let mut count = windows[0].0;
        for i in 1..windows.len() {
            count += windows[i].0 - windows[i - 1].1 - 1;
        }
        count
    }
}

impl AdventSolver for Advent2016Day20Solver {
    fn solve_part1(&self) -> usize {
        self.lowest_unblocked() as usize
    }

    fn solve_part2(&self) -> usize {
        self.unblocked_count() as usize
    }
}
