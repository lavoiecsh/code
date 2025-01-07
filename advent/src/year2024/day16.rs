use std::cmp::Ordering;
use crate::solver::AdventSolver;
use std::collections::{HashMap, HashSet, VecDeque};
use Direction::*;

pub struct Advent2024Day16Solver {
    maze: Maze,
}

impl Advent2024Day16Solver {
    pub fn new(input: &str) -> Self {
        Self {
            maze: Maze::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2024Day16Solver {
    fn solve_part1(&self) -> usize {
        self.maze.lowest_score().1.score
    }

    fn solve_part2(&self) -> usize {
        self.maze.tiles_in_best_paths()
    }
}

type Pos = (usize, usize, Direction);

struct Maze {
    map: Vec<Vec<char>>,
    start: Pos,
    ends: [Pos; 4],
    scores: HashMap<Pos, MazeScore>,
}

#[derive(Clone, Debug)]
struct MazeScore {
    score: usize,
    parents: Vec<Pos>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl MazeScore {
    fn initial() -> Self {
        Self {
            score: 0,
            parents: vec![],
        }
    }

    fn new(score: usize, parent: Pos) -> Self {
        Self {
            score,
            parents: vec![parent],
        }
    }
}

impl Maze {
    fn new(map: Vec<Vec<char>>) -> Self {
        let start = map
            .iter()
            .enumerate()
            .filter_map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .find(|(_, c)| **c == 'S')
                    .map(|(x, _)| (y, x, East))
            })
            .next()
            .unwrap();
        let end = map
            .iter()
            .enumerate()
            .filter_map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .find(|(_, c)| **c == 'E')
                    .map(|(x, _)| (y, x))
            })
            .next()
            .unwrap();
        let mut s = Self {
            map,
            start,
            ends: [
                (end.0, end.1, North),
                (end.0, end.1, South),
                (end.0, end.1, West),
                (end.0, end.1, East),
            ],
            scores: HashMap::new(),
        };
        s.build();
        s
    }

    fn build(&mut self) {
        let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
        self.scores.insert(self.start, MazeScore::initial());
        queue.push_back((self.start, 0));
        while let Some((pos, score)) = queue.pop_front() {
            let (forward, left, right) = match pos.2 {
                North => (
                    (pos.0 - 1, pos.1, North),
                    (pos.0, pos.1 - 1, West),
                    (pos.0, pos.1 + 1, East),
                ),
                South => (
                    (pos.0 + 1, pos.1, South),
                    (pos.0, pos.1 + 1, East),
                    (pos.0, pos.1 - 1, West),
                ),
                West => (
                    (pos.0, pos.1 - 1, West),
                    (pos.0 + 1, pos.1, South),
                    (pos.0 - 1, pos.1, North),
                ),
                East => (
                    (pos.0, pos.1 + 1, East),
                    (pos.0 - 1, pos.1, North),
                    (pos.0 + 1, pos.1, South),
                ),
            };
            let new_forward_score = score + 1;
            let new_side_score = score + 1001;
            if self.maybe_replace_score(forward, new_forward_score, pos) {
                queue.push_back((forward, new_forward_score));
            }
            if self.maybe_replace_score(left, new_side_score, pos) {
                queue.push_back((left, new_side_score));
            }
            if self.maybe_replace_score(right, new_side_score, pos) {
                queue.push_back((right, new_side_score));
            }
        }
    }

    fn maybe_replace_score(&mut self, pos: Pos, new_score: usize, parent: Pos) -> bool {
        if self.is_wall(pos) {
            return false;
        }
        if let Some(old_score) = self.scores.get_mut(&pos) {
            match old_score.score.cmp(&new_score) {
                Ordering::Less => {
                    false
                }
                Ordering::Equal => {
                    old_score.parents.push(parent);
                    false
                }
                Ordering::Greater => {
                    old_score.score = new_score;
                    old_score.parents = vec![parent];
                    true
                }
            }
        } else {
            self.scores.insert(pos, MazeScore::new(new_score, parent));
            true
        }
    }

    fn is_wall(&self, pos: Pos) -> bool {
        self.map[pos.0][pos.1] == '#'
    }

    fn lowest_score(&self) -> (Pos, MazeScore) {
        self.ends
            .iter()
            .filter_map(|p| self.scores.get(p).map(|s| (*p, s.clone())))
            .min_by_key(|(_, s)| s.score)
            .unwrap()
    }

    fn tiles_in_best_paths(&self) -> usize {
        let (best_position, best_score) = self.lowest_score();
        let mut tiles = HashSet::new();
        tiles.insert(best_position);
        let mut queue = VecDeque::new();
        queue.extend(best_score.parents);
        while let Some(current) = queue.pop_front() {
            if let Some(current_score) = self.scores.get(&current) {
                if tiles.insert(current) {
                    queue.extend(current_score.parents.iter());
                }
            }
        }
        tiles
            .iter()
            .map(|p| (p.0, p.1))
            .collect::<HashSet<(usize, usize)>>()
            .len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    static EXAMPLE_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn finds_lowest_score_for_maze() {
        assert_eq!(Advent2024Day16Solver::new(EXAMPLE_1).solve_part1(), 7036);
        assert_eq!(Advent2024Day16Solver::new(EXAMPLE_2).solve_part1(), 11048);
    }

    #[test]
    fn counts_number_of_tiles_in_best_paths() {
        assert_eq!(Advent2024Day16Solver::new(EXAMPLE_1).solve_part2(), 45);
        assert_eq!(Advent2024Day16Solver::new(EXAMPLE_2).solve_part2(), 64);
    }
}
