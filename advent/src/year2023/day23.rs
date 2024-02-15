use std::collections::VecDeque;

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day23Solver {
    trails: Trails,
}

impl Advent2023Day23Solver {
    pub fn new(input: String) -> Self {
        Self {
            trails: Trails::new(input.lines()
                .map(|l| l.chars().collect())
                .collect())
        }
    }
}

impl AdventSolver for Advent2023Day23Solver {
    fn solve_part1(&self) -> usize {
        self.trails.longest_path(false)
    }

    fn solve_part2(&self) -> usize {
        self.trails.longest_path(true)
    }
}

type Pos = (usize, usize);

struct Trails {
    grid: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

struct Node {
    pos: Pos,
    edges: Vec<usize>,
}

impl Node {
    fn new(pos: Pos) -> Self {
        Self { pos, edges: Vec::new() }
    }
}

struct Edge {
    end: usize,
    weight: usize,
    way: Way,
}

impl Edge {
    fn new(end: usize, weight: usize, way: Way) -> Self {
        Self { end, weight, way }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Way {
    Both,
    StartEnd,
    EndStart,
    None,
}

impl Way {
    fn combine_with(self, other: Self) -> Self {
        match (self, other) {
            (Way::None, _) => Way::None,
            (_, Way::None) => Way::None,
            (Way::StartEnd, Way::EndStart) => Way::None,
            (Way::StartEnd, _) => Way::StartEnd,
            (Way::EndStart, Way::StartEnd) => Way::None,
            (Way::EndStart, _) => Way::EndStart,
            (Way::Both, o) => o,
        }
    }

    fn is_usable(&self) -> bool {
        match self {
            Way::StartEnd | Way::Both => true,
            Way::None | Way::EndStart => false,
        }
    }
}

impl Trails {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let mut s = Self { max_y: grid.len() - 1, max_x: grid[0].len() - 1, grid, nodes: Vec::new(), edges: Vec::new() };
        s.build();
        s
    }

    fn build(&mut self) {
        let mut queue: VecDeque<(usize, Pos, Direction, Way)> = VecDeque::new();
        self.nodes.push(Node::new(self.start()));
        self.nodes.push(Node::new(self.end()));
        queue.push_back((0, self.nodes[0].pos, Direction::None, Way::Both));
        queue.push_back((1, self.nodes[1].pos, Direction::None, Way::Both));
        for y in 1..self.max_y {
            for x in 1..self.max_x {
                if self.grid[y][x] == '#' { continue; }
                let nexts = self.nexts((y, x), Direction::None);
                if nexts.len() == 2 { continue; }
                queue.extend(nexts.into_iter().map(|(c, f, w)| (self.nodes.len(), c, f, w)));
                self.nodes.push(Node::new((y, x)));
            }
        }

        while let Some((start, mut current, mut from, mut way)) = queue.pop_front() {
            let mut nexts = self.nexts(current, from);
            let mut weight = 1;
            let mut path_way = way;
            while nexts.len() == 1 {
                (current, from, way) = nexts[0];
                weight += 1;
                path_way = path_way.combine_with(way);
                nexts = self.nexts(current, from);
            }
            if let Some(end) = self.nodes.iter().position(|n| n.pos == current) {
                self.nodes[start].edges.push(self.edges.len());
                self.edges.push(Edge::new(end, weight, path_way));
            }
        }
    }

    fn start(&self) -> Pos {
        (0, self.grid[0].iter().position(|&c| c == '.').unwrap())
    }

    fn end(&self) -> Pos {
        (self.max_y, self.grid.last().unwrap().iter().position(|&c| c == '.').unwrap())
    }

    fn nexts(&self, pos: Pos, from: Direction) -> Vec<(Pos, Direction, Way)> {
        let mut nexts: Vec<(Pos, Direction, Way)> = vec!();
        if pos.0 > 0 && from != Direction::Down {
            let above = (pos.0 - 1, pos.1);
            match self.grid[above.0][above.1] {
                '#' => {}
                'v' => { nexts.push((above, Direction::Up, Way::EndStart)) }
                '.' => { nexts.push((above, Direction::Up, Way::Both)) }
                '^' => { nexts.push((above, Direction::Up, Way::StartEnd)) }
                '>' | '<' => panic!("horizontal slope going up at position ({},{})", above.0, above.1),
                c => panic!("unknown character {c} at position ({},{})", above.0, above.1),
            };
        }
        if pos.0 < self.max_y && from != Direction::Up {
            let below = (pos.0 + 1, pos.1);
            match self.grid[below.0][below.1] {
                '#' => {}
                '^' => { nexts.push((below, Direction::Down, Way::EndStart)) }
                '.' => { nexts.push((below, Direction::Down, Way::Both)) }
                'v' => { nexts.push((below, Direction::Down, Way::StartEnd)) }
                '>' | '<' => panic!("horizontal slope going down at position ({},{})", below.0, below.1),
                c => panic!("unknown character {c} at position ({},{})", below.0, below.1),
            }
        }
        if pos.1 > 0 && from != Direction::Right {
            let left = (pos.0, pos.1 - 1);
            match self.grid[left.0][left.1] {
                '#' => {}
                '>' => { nexts.push((left, Direction::Left, Way::EndStart)) }
                '.' => { nexts.push((left, Direction::Left, Way::Both)) }
                '<' => { nexts.push((left, Direction::Left, Way::StartEnd)) }
                'v' | '^' => panic!("vertical slope going left at position ({},{})", left.0, left.1),
                c => panic!("unknown character {c} at position ({},{})", left.0, left.1),
            }
        }
        if pos.1 < self.max_x && from != Direction::Left {
            let right = (pos.0, pos.1 + 1);
            match self.grid[right.0][right.1] {
                '#' => {}
                '<' => { nexts.push((right, Direction::Right, Way::EndStart)) }
                '.' => { nexts.push((right, Direction::Right, Way::Both)) }
                '>' => { nexts.push((right, Direction::Right, Way::StartEnd)) }
                'v' | '^' => panic!("vertical slope going right at position ({},{})", right.0, right.1),
                c => panic!("unknown character {c} at position ({},{})", right.0, right.1),
            }
        }
        nexts
    }

    fn longest_path(&self, undirected: bool) -> usize {
        let mut hikes: Vec<Hike> = Vec::new();
        let mut queue: VecDeque<Hike> = VecDeque::new();
        queue.push_back(Hike::new());
        while let Some(hike) = queue.pop_front() {
            if hike.last_node == 1 {
                hikes.push(hike);
                continue;
            }
            let mut next_edges = self.nodes[hike.last_node].edges
                .iter()
                .cloned()
                .filter(|&e| !hike.visited_nodes.contains(&self.edges[e].end) && (undirected || self.edges[e].way.is_usable()))
                .collect_vec();
            if let Some(e) = next_edges.pop() {
                queue.push_front(hike.visit(self.edges[e].end, e));
            }
            queue.extend(next_edges
                .into_iter()
                .map(|e| hike.visit(self.edges[e].end, e)));
        }
        hikes.iter()
            .map(|h| self.hike_len(h))
            .max()
            .unwrap()
    }

    fn hike_len(&self, hike: &Hike) -> usize {
        hike.visited_edges.iter()
            .map(|&e| self.edges[e].weight)
            .sum::<usize>() - 1
    }
}

#[derive(Clone)]
struct Hike {
    visited_nodes: Vec<usize>,
    visited_edges: Vec<usize>,
    last_node: usize,
}

impl Hike {
    fn new() -> Self {
        Self { visited_nodes: Vec::new(), visited_edges: Vec::new(), last_node: 0 }
    }

    fn visit(&self, node: usize, edge: usize) -> Self {
        let mut next = self.clone();
        next.visited_nodes.push(node);
        next.visited_edges.push(edge);
        next.last_node = node;
        next
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day23Solver {
    Advent2023Day23Solver::new(String::from("\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"))
}

#[test]
fn finds_longest_hike_going_down() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part1(), 94);
}

#[test]
fn finds_longest_hike_all_nexts() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 154);
}
