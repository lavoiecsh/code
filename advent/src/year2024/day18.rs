use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Advent2024Day18Solver {
    bytes: Vec<Pos>,
    width: usize,
    height: usize,
    first_part_byte_count: usize,
}

impl Advent2024Day18Solver {
    pub fn new(input: &str) -> Self {
        Self {
            width: 70,
            height: 70,
            first_part_byte_count: 1024,
            bytes: input
                .lines()
                .filter_map(|l| l.split(',').map(|v| v.parse().unwrap()).collect_tuple())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2024Day18Solver {
    fn solve_part1(&self) -> usize {
        let mut memory_space = MemorySpace::new(self.width, self.height);
        self.bytes
            .iter()
            .take(self.first_part_byte_count)
            .for_each(|&b| memory_space.corrupt(b));
        memory_space.steps_to_exit()
    }

    fn solve_part2_string(&self) -> String {
        let mut memory_space = MemorySpace::new(self.width, self.height);
        self.bytes
            .iter()
            .take(self.first_part_byte_count)
            .for_each(|&b| memory_space.corrupt(b));
        self.bytes
            .iter()
            .skip(self.first_part_byte_count)
            .find(|&&b| {
                memory_space.corrupt(b);
                !memory_space.has_path_to_exit()
            })
            .map(|p| format!("{},{}", p.0, p.1))
            .unwrap()
    }
}

type Pos = (usize, usize);

struct MemorySpace {
    corrupted: HashSet<Pos>,
    width: usize,
    height: usize,
}

impl MemorySpace {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            corrupted: HashSet::new(),
        }
    }

    fn corrupt(&mut self, pos: Pos) {
        self.corrupted.insert(pos);
    }

    fn steps_to_exit(&self) -> usize {
        let mut steps = HashMap::new();
        steps.insert((0, 0), 0);
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        macro_rules! update_if_better {
            ($pos: expr, $next: expr) => {
                if !self.corrupted.contains(&$pos)
                    && steps.get(&$pos).map_or(true, |&current| $next < current) {
                    steps.insert($pos, $next);
                    queue.push_back($pos);
                }
            }
        }
        while let Some(pos) = queue.pop_front() {
            let next = steps.get(&pos).unwrap() + 1;
            if pos.0 > 0 {
                update_if_better!((pos.0 - 1, pos.1), next);
            }
            if pos.0 < self.width {
                update_if_better!((pos.0 + 1, pos.1), next);
            }
            if pos.1 > 0 {
                update_if_better!((pos.0, pos.1 - 1), next);
            }
            if pos.1 < self.height {
                update_if_better!((pos.0, pos.1 + 1), next);
            }
        }
        *steps.get(&(self.width, self.height)).unwrap()
    }

    fn has_path_to_exit(&self) -> bool {
        let mut grid = vec![vec!['.'; self.width + 1]; self.height + 1];
        self.corrupted
            .iter()
            .for_each(|&pos| grid[pos.1][pos.0] = '#');
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        grid[0][0] = 'O';
        while let Some((x, y)) = queue.pop_front() {
            if x > 0 && grid[y][x - 1] == '.' {
                grid[y][x - 1] = 'O';
                queue.push_back((x - 1, y));
            }
            if x < self.width && grid[y][x + 1] == '.' {
                grid[y][x + 1] = 'O';
                queue.push_back((x + 1, y));
            }
            if y > 0 && grid[y - 1][x] == '.' {
                grid[y - 1][x] = 'O';
                queue.push_back((x, y - 1));
            }
            if y < self.height && grid[y + 1][x] == '.' {
                grid[y + 1][x] = 'O';
                queue.push_back((x, y + 1));
            }
        }
        grid[self.height][self.width] == 'O'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn finds_minimum_number_of_steps_to_reach_exit_after_12_bytes() {
        let mut solver = Advent2024Day18Solver::new(EXAMPLE);
        solver.width = 6;
        solver.height = 6;
        solver.first_part_byte_count = 12;
        assert_eq!(solver.solve_part1(), 22);
    }

    #[test]
    fn finds_first_byte_blocking_path() {
        let mut solver = Advent2024Day18Solver::new(EXAMPLE);
        solver.width = 6;
        solver.height = 6;
        solver.first_part_byte_count = 12;
        assert_eq!(solver.solve_part2_string(), "6,1");
    }
}
