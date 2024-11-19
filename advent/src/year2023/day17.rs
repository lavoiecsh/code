use std::collections::HashMap;
use std::fmt::Debug;

use Direction::*;

use crate::solver::AdventSolver;

pub struct Advent2023Day17Solver {
    city: City,
}

impl Advent2023Day17Solver {
    pub fn new(input: &str) -> Self {
        Self {
            city: City::new(
                input
                    .lines()
                    .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2023Day17Solver {
    fn solve_part1(&self) -> usize {
        // todo could be faster, 5 minutes
        self.city.least_heat(false)
    }

    fn solve_part2(&self) -> usize {
        // todo could be faster, 5 minutes
        self.city.least_heat(true)
    }
}

struct City {
    blocks: Vec<Vec<u8>>,
}

impl City {
    fn new(blocks: Vec<Vec<u8>>) -> Self {
        Self { blocks }
    }

    fn least_heat(&self, ultra: bool) -> usize {
        HeatedTree::new(&self.blocks).least_to_lower_right(ultra)
    }
}

struct HeatedTree {
    max_x: usize,
    max_y: usize,
    blocks: Vec<Vec<HeatedBlock>>,
    nodes: Vec<HeatedBlockNode>,
}

impl HeatedTree {
    fn new(grid: &[Vec<u8>]) -> Self {
        Self {
            max_x: grid[0].len() - 1,
            max_y: grid.len() - 1,
            blocks: grid
                .iter()
                .map(|row| row.iter().map(HeatedBlock::new).collect())
                .collect(),
            nodes: vec![],
        }
    }

    fn least_to_lower_right(&mut self, ultra: bool) -> usize {
        self.nodes.push(HeatedBlockNode {
            parent: None,
            pos: (0, 0),
            direction: Init,
            heat: 0,
        });
        let mut best_found = usize::MAX;
        let mut index = 0;
        while index < self.nodes.len() {
            let current = &self.nodes[index];
            if current.pos == (self.max_x, self.max_y) && current.heat < best_found {
                best_found = current.heat;
            }
            if current.heat >= best_found {
                index += 1;
                continue;
            }
            self.nexts(index, ultra);
            index += 1;
        }
        best_found
    }

    fn nexts(&mut self, node_id: usize, ultra: bool) {
        match self.nodes[node_id].direction {
            Left(_) | Right(_) => {
                self.up(node_id, ultra);
                self.down(node_id, ultra);
            }
            Up(_) | Down(_) => {
                self.left(node_id, ultra);
                self.right(node_id, ultra);
            }
            Init => {
                self.up(node_id, ultra);
                self.down(node_id, ultra);
                self.left(node_id, ultra);
                self.right(node_id, ultra);
            }
        };
    }

    fn up(&mut self, node_id: usize, ultra: bool) {
        let mut heat = self.nodes[node_id].heat;
        for c in 1..=(if ultra { 10 } else { 3 }) {
            if self.nodes[node_id].pos.1 < c {
                return;
            }
            let pos = (self.nodes[node_id].pos.0, self.nodes[node_id].pos.1 - c);
            heat += self.blocks[pos.1][pos.0].heat;
            if (!ultra && c <= 3) || (ultra && c >= 4) {
                self.insert(node_id, pos, Up(c), heat);
            }
        }
    }

    fn down(&mut self, node_id: usize, ultra: bool) {
        let mut heat = self.nodes[node_id].heat;
        for c in 1..=(if ultra { 10 } else { 3 }) {
            if self.nodes[node_id].pos.1 > self.max_y - c {
                return;
            }
            let pos = (self.nodes[node_id].pos.0, self.nodes[node_id].pos.1 + c);
            heat += self.blocks[pos.1][pos.0].heat;
            if (!ultra && c <= 3) || (ultra && c >= 4) {
                self.insert(node_id, pos, Down(c), heat);
            }
        }
    }

    fn left(&mut self, node_id: usize, ultra: bool) {
        let mut heat = self.nodes[node_id].heat;
        for c in 1..=(if ultra { 10 } else { 3 }) {
            if self.nodes[node_id].pos.0 < c {
                return;
            }
            let pos = (self.nodes[node_id].pos.0 - c, self.nodes[node_id].pos.1);
            heat += self.blocks[pos.1][pos.0].heat;
            if (!ultra && c <= 3) || (ultra && c >= 4) {
                self.insert(node_id, pos, Left(c), heat);
            }
        }
    }

    fn right(&mut self, node_id: usize, ultra: bool) {
        let mut heat = self.nodes[node_id].heat;
        for c in 1..=(if ultra { 10 } else { 3 }) {
            if self.nodes[node_id].pos.0 > self.max_x - c {
                return;
            }
            let pos = (self.nodes[node_id].pos.0 + c, self.nodes[node_id].pos.1);
            heat += self.blocks[pos.1][pos.0].heat;
            if (!ultra && c <= 3) || (ultra && c >= 4) {
                self.insert(node_id, pos, Right(c), heat);
            }
        }
    }

    fn insert(&mut self, parent: usize, pos: (usize, usize), direction: Direction, heat: usize) {
        let mut p = self.nodes[parent].parent;
        while let Some(q) = p {
            if self.nodes[q].pos == pos {
                return;
            }
            p = self.nodes[q].parent;
        }
        if let Some(previous_heat) = self.blocks[pos.1][pos.0].least.get(&direction) {
            if heat >= *previous_heat {
                return;
            }
        }
        self.blocks[pos.1][pos.0].least.insert(direction, heat);
        self.nodes.push(HeatedBlockNode {
            parent: Some(parent),
            pos,
            direction,
            heat,
        });
    }
}

struct HeatedBlock {
    heat: usize,
    least: HashMap<Direction, usize>,
}

impl HeatedBlock {
    fn new(heat: &u8) -> Self {
        Self {
            heat: *heat as usize,
            least: HashMap::new(),
        }
    }
}

struct HeatedBlockNode {
    parent: Option<usize>,
    pos: (usize, usize),
    direction: Direction,
    heat: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Right(usize),
    Up(usize),
    Left(usize),
    Down(usize),
    Init,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn finds_least_heat_loss_path() {
        let solver = Advent2023Day17Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 102);
    }

    #[test]
    fn finds_least_heat_loss_path_ultra() {
        let solver = Advent2023Day17Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 94);
    }
}
