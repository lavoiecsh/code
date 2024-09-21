use std::collections::HashMap;

use regex::Regex;

use crate::solver::AdventSolver;

struct D {
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    files: Vec<F>,
}

impl D {
    fn new(name: String, parent: Option<usize>) -> Self {
        Self { name, parent, children: Vec::new(), files: Vec::new() }
    }

    fn file_sizes(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }
}

struct F {
    size: usize,
}

pub struct Advent2022Day07Solver {
    directories: Vec<D>,
}

impl Advent2022Day07Solver {
    pub fn new(input: String) -> Self {
        let command_regex = Regex::new(r"\$ (cd|ls) ?(.*)?").unwrap();
        let mut directories = vec!(D::new(String::from("/"), None));
        let mut directories_len = 1;
        let mut wdi = 0;
        for line in input.lines() {
            let command_captures = command_regex.captures(line);
            if command_captures.is_some() {
                let c = command_captures.unwrap();
                if c.get(1).unwrap().as_str() == "ls" {
                    continue;
                }
                let to = c.get(2).unwrap().as_str();
                if to == "/" {
                    wdi = 0;
                } else if to == ".." {
                    wdi = directories[wdi].parent.unwrap();
                } else {
                    wdi = *directories[wdi].children.iter().find(|cdi| directories[**cdi].name == to).unwrap();
                }
                continue;
            }
            // ls
            let s = line.split(" ").map(String::from).collect::<Vec<String>>();
            if s[0] == "dir" {
                directories.push(D::new(s[1].clone(), Some(wdi)));
                directories[wdi].children.push(directories_len);
                directories_len += 1;
            } else {
                directories[wdi].files.push(F { size: s[0].parse().unwrap() });
            }
        }
        Self { directories }
    }

    fn compute_directory_sizes(&self) -> HashMap<usize, usize> {
        let mut sizes: HashMap<usize, usize> = HashMap::new();
        while sizes.len() != self.directories.len() {
            for di in 0..self.directories.len() {
                if sizes.contains_key(&di) {
                    continue;
                }
                let d = &self.directories[di];
                if d.children.is_empty() {
                    sizes.insert(di, d.file_sizes());
                    continue;
                }
                let children_sizes = d.children
                    .iter()
                    .map(|ci| sizes.get(ci))
                    .try_fold(0usize, |acc, cur| cur.map(|c| acc + c));
                if let Some(children_size) = children_sizes {
                    sizes.insert(di, d.file_sizes() + children_size);
                }
            }
        }
        sizes
    }
}

impl AdventSolver for Advent2022Day07Solver {
    fn solve_part1(&self) -> usize {
        let max: usize = 100000;
        let directory_sizes = self.compute_directory_sizes();
        directory_sizes
            .into_values()
            .filter(|s| *s <= max)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let max: usize = 70000000;
        let need: usize = 30000000;
        let directory_sizes = self.compute_directory_sizes();
        let current = directory_sizes.get(&0).unwrap();
        let to_free = need - (max - current);
        directory_sizes
            .into_values()
            .filter(|s| *s >= to_free)
            .min()
            .unwrap()
    }
}
