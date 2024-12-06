use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Advent2024Day06Solver {
    grid: Grid,
}

impl Advent2024Day06Solver {
    pub fn new(input: &str) -> Self {
        let mut obstacles = HashSet::new();
        let mut guard = None;
        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| match c {
                '.' => {}
                '#' => {
                    obstacles.insert((y, x));
                }
                '^' => guard = Some(((y, x), Dir::Up)),
                'v' => guard = Some(((y, x), Dir::Down)),
                '<' => guard = Some(((y, x), Dir::Left)),
                '>' => guard = Some(((y, x), Dir::Right)),
                _ => unreachable!("unknown character {c}"),
            })
        });
        Self {
            grid: Grid {
                height: input.lines().count(),
                width: input.lines().next().unwrap().chars().count(),
                initial_guard_pos: guard.unwrap().0,
                initial_guard_dir: guard.unwrap().1,
                obstacles,
            },
        }
    }
}

impl AdventSolver for Advent2024Day06Solver {
    fn solve_part1(&self) -> usize {
        let mut guard = self.grid.new_guard();
        guard.patrol(&self.grid);
        guard.seen.iter().map(|&(pos, _)| pos).unique().count()
    }

    fn solve_part2(&self) -> usize {
        // todo slow (12s)
        let mut guard = self.grid.new_guard();
        guard.patrol(&self.grid);
        guard
            .seen
            .iter()
            .map(|&(pos, _)| pos)
            .unique()
            .filter(|&p| {
                let new_grid = self.grid.add_obstacle(p);
                let mut new_guard = new_grid.new_guard();
                new_guard.patrol(&new_grid);
                new_guard.looped
            })
            .count()
    }
}

#[derive(Clone)]
struct Grid {
    height: usize,
    width: usize,
    obstacles: HashSet<Pos>,
    initial_guard_pos: Pos,
    initial_guard_dir: Dir,
}

impl Grid {
    fn has_obstacle(&self, pos: Pos) -> bool {
        self.obstacles.contains(&pos)
    }

    fn new_guard(&self) -> Guard {
        Guard::new(self.initial_guard_pos, self.initial_guard_dir)
    }

    fn add_obstacle(&self, pos: Pos) -> Self {
        let mut copy = self.clone();
        copy.obstacles.insert(pos);
        copy
    }
}

type Pos = (usize, usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
struct Guard {
    pos: Pos,
    dir: Dir,
    seen: HashSet<(Pos, Dir)>,
    looped: bool,
}

impl Guard {
    fn new(pos: Pos, dir: Dir) -> Self {
        let mut seen = HashSet::new();
        seen.insert((pos, dir));
        Self {
            pos,
            dir,
            seen,
            looped: false,
        }
    }

    fn patrol(&mut self, grid: &Grid) {
        while let Some((next_pos, next_dir)) = self.next_position(grid) {
            self.pos = next_pos;
            self.dir = next_dir;
            if !self.seen.insert((next_pos, next_dir)) {
                self.looped = true;
                break;
            }
        }
    }

    fn next_position(&mut self, grid: &Grid) -> Option<(Pos, Dir)> {
        match self.dir {
            Dir::Up => {
                if self.pos.0 == 0 {
                    None
                } else if grid.has_obstacle((self.pos.0 - 1, self.pos.1)) {
                    self.dir = Dir::Right;
                    self.next_position(grid)
                } else {
                    Some(((self.pos.0 - 1, self.pos.1), Dir::Up))
                }
            }
            Dir::Right => {
                if self.pos.1 == grid.width - 1 {
                    None
                } else if grid.has_obstacle((self.pos.0, self.pos.1 + 1)) {
                    self.dir = Dir::Down;
                    self.next_position(grid)
                } else {
                    Some(((self.pos.0, self.pos.1 + 1), Dir::Right))
                }
            }
            Dir::Down => {
                if self.pos.0 == grid.height - 1 {
                    None
                } else if grid.has_obstacle((self.pos.0 + 1, self.pos.1)) {
                    self.dir = Dir::Left;
                    self.next_position(grid)
                } else {
                    Some(((self.pos.0 + 1, self.pos.1), Dir::Down))
                }
            }
            Dir::Left => {
                if self.pos.1 == 0 {
                    None
                } else if grid.has_obstacle((self.pos.0, self.pos.1 - 1)) {
                    self.dir = Dir::Up;
                    self.next_position(grid)
                } else {
                    Some(((self.pos.0, self.pos.1 - 1), Dir::Left))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn finds_guarded_positions() {
        let solver = Advent2024Day06Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 41);
    }

    #[test]
    fn finds_possible_obstructions() {
        let solver = Advent2024Day06Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 6);
    }
}
