use itertools::Itertools;
use std::fmt::{Debug, Formatter};

use crate::solver::AdventSolver;

pub struct Advent2017Day21Solver {
    rules: Vec<Rule>,
}

impl Advent2017Day21Solver {
    pub fn new(input: &str) -> Self {
        Self {
            rules: input
                .lines()
                .map(|l| l.split(" => ").collect::<Vec<&str>>())
                .map(|s| Rule::new(s[0], s[1]))
                .collect(),
        }
    }

    fn solve(&self, iterations: usize) -> usize {
        let mut grid = Grid::init(&self.rules);
        (0..iterations).for_each(|_| grid.iterate());
        grid.count_on()
    }
}

impl AdventSolver for Advent2017Day21Solver {
    fn solve_part1(&self) -> usize {
        self.solve(5)
    }

    fn solve_part2(&self) -> usize {
        self.solve(18)
    }
}

struct Grid<'a> {
    grid: Vec<Vec<bool>>,
    rules: &'a Vec<Rule>,
}

impl<'a> Grid<'a> {
    fn init(rules: &'a Vec<Rule>) -> Self {
        Self {
            grid: vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true],
            ],
            rules,
        }
    }

    fn iterate(&mut self) {
        let sub = if self.grid.len() % 2 == 0 { 2 } else { 3 };
        let mut new_grid = Vec::new();
        let size = self.grid.len() / sub;
        for i in 0..size {
            let mut new_rows: Vec<Vec<bool>> = (0..=sub).map(|_| Vec::new()).collect();
            for j in 0..size {
                let subgrid = self.subgrid(i * sub, j * sub, sub);
                let options = options(&subgrid, sub);
                let rule = self.rules.iter().find(|r| r.matches(&options)).unwrap();
                (0..=sub).for_each(|i| new_rows[i].extend(rule.to[i].iter()));
            }
            new_grid.extend(new_rows.into_iter());
        }
        self.grid = new_grid;
    }

    fn subgrid(&self, row: usize, col: usize, size: usize) -> Vec<bool> {
        (0..size)
            .flat_map(|i| (0..size).map(move |j| self.grid[row + i][col + j]))
            .collect()
    }

    fn count_on(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|col| **col).count())
            .sum()
    }
}

impl Debug for Grid<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\n{}",
            &self
                .grid
                .iter()
                .map(|row| row.iter().map(|col| if *col { '#' } else { '.' }).join(""))
                .join("\n")
        ))
    }
}

fn options(input: &[bool], size: usize) -> Vec<Vec<bool>> {
    (if size == 2 {
        vec![
            vec![0, 1, 2, 3], // original
            vec![1, 3, 0, 2], // rotate left
            vec![2, 0, 3, 1], // rotate right
            vec![3, 2, 1, 0], // rotate 180
            vec![1, 0, 3, 2], // flip vertical
            vec![2, 3, 0, 1], // flip horizontal,
            vec![0, 2, 1, 3], // flip \
            vec![3, 1, 2, 0], // flip /
        ]
    } else {
        vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8], // original
            vec![2, 5, 8, 1, 4, 7, 0, 3, 6], // rotate left
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2], // rotate right
            vec![8, 7, 6, 5, 4, 3, 2, 1, 0], // rotate 180
            vec![6, 7, 8, 3, 4, 5, 0, 1, 2], // flip vertical
            vec![2, 1, 0, 5, 4, 3, 8, 7, 6], // flip horizontal
            vec![0, 3, 6, 1, 4, 7, 2, 5, 8], // flip \
            vec![8, 5, 2, 7, 4, 1, 6, 3, 0], // flip /
        ]
    })
    .iter()
    .map(|o| o.iter().map(|i| input[*i]).collect())
    .collect()
}

struct Rule {
    from: Vec<bool>,
    to: Vec<Vec<bool>>,
}

impl Rule {
    fn new(from: &str, to: &str) -> Self {
        Self {
            from: from
                .chars()
                .filter(|c| *c != '/')
                .map(|c| c == '#')
                .collect(),
            to: to
                .split("/")
                .map(|row| row.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }

    fn matches(&self, options: &[Vec<bool>]) -> bool {
        options.iter().any(|o| o == &self.from)
    }
}
