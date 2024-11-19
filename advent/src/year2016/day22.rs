use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use regex::{Match, Regex};

use crate::solver::AdventSolver;

pub struct Advent2016Day22Solver {
    nodes: Vec<Node>,
}

impl Advent2016Day22Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%")
            .unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        Self {
            nodes: input
                .lines()
                .filter_map(|l| re.captures(l))
                .map(|c| Node {
                    x: parse(c.get(1)) as u8,
                    y: parse(c.get(2)) as u8,
                    size: parse(c.get(3)),
                    used: parse(c.get(4)),
                    avail: parse(c.get(5)),
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2016Day22Solver {
    fn solve_part1(&self) -> usize {
        self.nodes
            .iter()
            .map(|n1| self.nodes.iter().filter(|n2| n1.is_viable_pair(n2)).count())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let grid = Grid::new(&self.nodes);
        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        queue.push(State::init(&self.nodes));
        while let Some(state) = queue.pop() {
            if state.is_terminal() {
                return state.steps.len();
            }
            state.nexts(&grid).for_each(|s| {
                if !queue.iter().contains(&s) {
                    queue.push(s)
                }
            });
        }
        0
    }
}

#[derive(Clone, Debug)]
struct Node {
    x: u8,
    y: u8,
    size: u32,
    used: u32,
    avail: u32,
}

impl Node {
    fn is_viable_pair(&self, other: &Self) -> bool {
        self.used != 0 && (self.x != other.x || self.y != other.y) && self.used <= other.avail
    }
}

type Pos = (usize, usize);

struct Grid {
    capacities: Vec<Vec<u32>>,
    max_x: usize,
    max_y: usize,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn not_opposite(&self, other: &Self) -> bool {
        (self == &Direction::Up && other != &Direction::Down)
            || (self == &Direction::Down && other != &Direction::Up)
            || (self == &Direction::Left && other != &Direction::Right)
            || (self == &Direction::Right && other != &Direction::Left)
    }
}

struct State {
    used: Vec<Vec<u32>>,
    steps: Vec<Direction>,
    target: Pos,
    zero: Pos,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({},{}) ({},{}) -> {} = {}\n{}",
            self.zero.0,
            self.zero.1,
            self.target.0,
            self.target.1,
            self.steps.len(),
            self.cost(),
            self.used
                .iter()
                .map(|n| n.iter().map(|v| format!("{v:3}")).join("  "))
                .join("\n")
        ))
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.used.eq(&other.used) && self.target.eq(&other.target) && self.zero.eq(&other.zero)
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.used.hash(state);
        self.target.hash(state);
        self.zero.hash(state);
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost().cmp(&self.cost())
    }
}

impl Grid {
    fn new(nodes: &[Node]) -> Self {
        Self {
            capacities: nodes
                .iter()
                .into_group_map_by(|n| n.y)
                .iter()
                .sorted_by_key(|(g, _)| *g)
                .map(|(_, ns)| ns.iter().sorted_by_key(|n| n.x).map(|n| n.size).collect())
                .collect(),
            max_x: nodes.iter().map(|n| n.x).max().unwrap() as usize,
            max_y: nodes.iter().map(|n| n.y).max().unwrap() as usize,
        }
    }

    fn capacity(&self, point: &Pos) -> u32 {
        self.capacities[point.1][point.0]
    }
}

impl State {
    fn init(nodes: &[Node]) -> Self {
        Self {
            used: nodes
                .iter()
                .into_group_map_by(|n| n.y)
                .iter()
                .sorted_by_key(|(y, _)| *y)
                .map(|(_, ns)| ns.iter().sorted_by_key(|n| n.x).map(|n| n.used).collect())
                .collect(),
            steps: Vec::new(),
            target: (
                nodes
                    .iter()
                    .filter(|n| n.y == 0)
                    .map(|n| n.x)
                    .max()
                    .unwrap() as usize,
                0,
            ),
            zero: nodes
                .iter()
                .find(|n| n.used == 0)
                .map(|n| (n.x as usize, n.y as usize))
                .unwrap(),
        }
    }

    fn is_terminal(&self) -> bool {
        self.target == (0, 0)
    }

    fn cost(&self) -> usize {
        self.steps.len()
            + self.distance(
                &self.zero,
                &(
                    if self.target.0 == 0 {
                        0
                    } else {
                        self.target.0 - 1
                    },
                    self.target.1,
                ),
            ) * 1000
            + self.distance(&self.target, &(0, 0)) * 1000000
    }

    fn distance(&self, from: &Pos, to: &Pos) -> usize {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        distances.insert(*from, 0);
        let mut queue: VecDeque<Pos> = VecDeque::new();
        queue.push_back(*from);
        while !distances.contains_key(to) {
            let current = queue.pop_front().unwrap();
            let next_distance = distances.get(&current).unwrap() + 1;
            if current.0 > 0
                && !distances.contains_key(&(current.0 - 1, current.1))
                && self.used[current.1][current.0 - 1] < 100
            {
                distances.insert((current.0 - 1, current.1), next_distance);
                queue.push_back((current.0 - 1, current.1));
            }
            if current.0 < self.used[current.1].len() - 1
                && !distances.contains_key(&(current.0 + 1, current.1))
                && self.used[current.1][current.0 + 1] < 100
            {
                distances.insert((current.0 + 1, current.1), next_distance);
                queue.push_back((current.0 + 1, current.1));
            }
            if current.1 > 0
                && !distances.contains_key(&(current.0, current.1 - 1))
                && self.used[current.1 - 1][current.0] < 100
            {
                distances.insert((current.0, current.1 - 1), next_distance);
                queue.push_back((current.0, current.1 - 1));
            }
            if current.1 < self.used.len() - 1
                && !distances.contains_key(&(current.0, current.1 + 1))
                && self.used[current.1 + 1][current.0] < 100
            {
                distances.insert((current.0, current.1 + 1), next_distance);
                queue.push_back((current.0, current.1 + 1));
            }
        }
        *distances.get(to).unwrap()
    }

    fn move_data(&self, from: &Pos, to: &Pos, direction: Direction) -> Self {
        let mut used = self.used.clone();
        used[from.1][from.0] = 0;
        used[to.1][to.0] += self.used[from.1][from.0];
        let target = if &self.target == from {
            *to
        } else {
            self.target
        };
        let mut steps = self.steps.clone();
        steps.push(direction);
        Self {
            used,
            steps,
            target,
            zero: *from,
        }
    }

    fn nexts(&self, grid: &Grid) -> impl Iterator<Item = Self> {
        let capacity = grid.capacity(&self.zero);
        vec![
            if self.zero.0 > 0 {
                Some((Direction::Left, (self.zero.0 - 1, self.zero.1)))
            } else {
                None
            },
            if self.zero.0 < grid.max_x {
                Some((Direction::Right, (self.zero.0 + 1, self.zero.1)))
            } else {
                None
            },
            if self.zero.1 > 0 {
                Some((Direction::Up, (self.zero.0, self.zero.1 - 1)))
            } else {
                None
            },
            if self.zero.1 < grid.max_y {
                Some((Direction::Down, (self.zero.0, self.zero.1 + 1)))
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .filter(|from| capacity > self.used[from.1 .1][from.1 .0])
        .filter(|from| {
            self.steps.is_empty() || self.steps[self.steps.len() - 1].not_opposite(&from.0)
        })
        .map(|from| self.move_data(&from.1, &self.zero, from.0))
        .collect_vec()
        .into_iter()
    }
}
