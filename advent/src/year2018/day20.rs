use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

pub struct Advent2018Day20Solver {
    map: Map,
}

impl Advent2018Day20Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: Map::new(
                &input
                    .chars()
                    .skip(1)
                    .take(input.len() - 2)
                    .collect::<String>(),
            ),
        }
    }
}

impl AdventSolver for Advent2018Day20Solver {
    fn solve_part1(&self) -> usize {
        self.map.furthest_room()
    }

    fn solve_part2(&self) -> usize {
        self.map.outside_range(1000)
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Pos(i32, i32);
impl Pos {
    fn move_to(self, d: char) -> Self {
        match d {
            'N' => Self(self.0, self.1 - 1),
            'S' => Self(self.0, self.1 + 1),
            'E' => Self(self.0 + 1, self.1),
            'W' => Self(self.0 - 1, self.1),
            _ => unreachable!("Invalid direction: {}", d),
        }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:4}, {:4})", self.0, self.1))
    }
}

#[derive(Debug)]
struct Map {
    distances: HashMap<Pos, usize>,
}

impl Map {
    fn new(regex: &str) -> Self {
        let mut doors = HashSet::new();
        let mut nodes = vec![PathNode::root()];
        let mut current_node = 0;
        for c in regex.chars() {
            match c {
                'N' | 'S' | 'E' | 'W' => {
                    let this_position = nodes[current_node].position;
                    let next_position = this_position.move_to(c);
                    nodes[current_node].position = next_position;
                    doors.insert((this_position, next_position));
                }
                '(' => {
                    let new_node = nodes.len();
                    nodes.push(PathNode::new(current_node, nodes[current_node].position));
                    nodes[current_node].children.push(new_node);
                    current_node = new_node;
                }
                ')' => {
                    current_node = nodes[current_node].parent.unwrap();
                }
                '|' => {
                    let new_node = nodes.len();
                    let parent_node = nodes[current_node].parent.unwrap();
                    nodes.push(PathNode::new(parent_node, nodes[parent_node].position));
                    nodes[parent_node].children.push(new_node);
                    current_node = new_node;
                }
                _ => unreachable!("Invalid char: {}", c),
            }
        }
        let mut distances = HashMap::new();
        distances.insert(Pos(0, 0), 0usize);
        let mut queue = VecDeque::new();
        queue.push_back((Pos(0, 0), 0));
        while let Some((pos, distance)) = queue.pop_front() {
            let next_distance = distance + 1;
            let adjacents = doors
                .iter()
                .filter(|&(p, _)| p == &pos)
                .map(|&(_, a)| a)
                .collect_vec();
            for adjacent in adjacents {
                if let Some(&da) = distances.get(&adjacent) {
                    if next_distance < da {
                        distances.insert(adjacent, next_distance);
                        queue.push_back((adjacent, next_distance));
                    }
                } else {
                    distances.insert(adjacent, next_distance);
                    queue.push_back((adjacent, next_distance));
                }
            }
        }
        Self { distances }
    }

    fn furthest_room(&self) -> usize {
        *self.distances.values().max().unwrap()
    }

    fn outside_range(&self, distance: usize) -> usize {
        self.distances.values().filter(|&&d| d >= distance).count()
    }
}

#[derive(Debug)]
struct PathNode {
    parent: Option<usize>,
    children: Vec<usize>,
    position: Pos,
}

impl PathNode {
    fn root() -> Self {
        Self {
            parent: None,
            children: vec![],
            position: Pos(0, 0),
        }
    }

    fn new(parent: usize, position: Pos) -> Self {
        Self {
            parent: Some(parent),
            children: vec![],
            position,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = "^ENWWW(NEEE|SSE(EE|N))$";
    const EXAMPLE_2: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    const EXAMPLE_3: &str = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
    const EXAMPLE_4: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

    #[test]
    fn finds_most_amount_of_doors() {
        assert_eq!(Advent2018Day20Solver::new(EXAMPLE_1).solve_part1(), 10);
        assert_eq!(Advent2018Day20Solver::new(EXAMPLE_2).solve_part1(), 18);
        assert_eq!(Advent2018Day20Solver::new(EXAMPLE_3).solve_part1(), 23);
        assert_eq!(Advent2018Day20Solver::new(EXAMPLE_4).solve_part1(), 31);
    }
}
