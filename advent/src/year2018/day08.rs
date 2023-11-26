use crate::solver::AdventSolver;

pub struct Advent2018Day08Solver {
    tree: Node,
}

impl Advent2018Day08Solver {
    pub fn new(input: String) -> Self {
        let numbers: Vec<usize> = input.split(" ").map(|n| n.parse().unwrap()).collect();
        Self { tree: read(&numbers, &mut 0) }
    }
}

impl AdventSolver for Advent2018Day08Solver {
    fn solve_part1(&self) -> usize {
        self.tree.metadata_sum()
    }

    fn solve_part2(&self) -> usize {
        self.tree.value()
    }
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>() + self.children.iter().map(Node::metadata_sum).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            self.metadata.iter()
                .filter(|m| **m != 0 && **m <= self.children.len())
                .map(|m| self.children[*m - 1].value())
                .sum()
        }
    }
}

fn read(numbers: &Vec<usize>, index: &mut usize) -> Node {
    let children_count = numbers[*index];
    *index += 1;
    let metadata_count = numbers[*index];
    *index += 1;
    let mut children = Vec::new();
    for _ in 0..children_count {
        children.push(read(numbers, index));
    }
    let mut metadata = Vec::new();
    for _ in 0..metadata_count {
        metadata.push(numbers[*index]);
        *index += 1;
    }
    Node { children, metadata }
}
