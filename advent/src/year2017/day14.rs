use std::collections::HashSet;
use crate::solver::AdventSolver;
use crate::year2017::day10::KnotHash;

pub struct Advent2017Day14Solver {
    key: String
}

impl Advent2017Day14Solver {
    pub fn new(input: String) -> Self {
        Self { key: input }
    }
}

impl AdventSolver for Advent2017Day14Solver {
    fn solve_part1(&self) -> usize {
        to_grid(&self.key)
            .iter()
            .map(|r| r.iter().filter(|b| **b).count())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        count_regions(&to_grid(&self.key))
    }
}

fn count_regions(grid: &Vec<Vec<bool>>) -> usize {
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;
    let mut regions: Vec<Vec<Option<usize>>> = Vec::new();
    let mut index = 0;
    for i in 0..=last_row {
        let mut row = Vec::new();
        for j in 0..=last_col {
            if grid[i][j] {
                row.push(Some(index));
                index += 1;
            } else {
                row.push(None);
            }
        }
        regions.push(row);
    }

    let mut changes = true;
    while changes {
        changes = false;
        for i in 0..=last_row {
            for j in 0..=last_col {
                if let Some(current) = regions[i][j] {
                    let around = vec!(
                        if i > 0 { regions[i-1][j] } else { None },
                        if i < last_row { regions[i+1][j] } else { None },
                        if j > 0 { regions[i][j-1] } else { None },
                        if j < last_col { regions[i][j+1] } else { None },
                    ).iter()
                        .filter_map(|n| *n)
                        .min();
                    if around.is_some_and(|n| n < current) {
                        regions[i][j] = around;
                        changes = true;
                    }
                }
            }
        }
    }

    regions.iter()
        .flat_map(|row| row.iter().filter_map(|n| *n))
        .collect::<HashSet<usize>>()
        .len()
}

fn to_grid(key: &String) -> Vec<Vec<bool>> {
    (0..128)
        .map(|i| format!("{key}-{i}"))
        .map(|l| l.chars().map(|c| c as u8).collect())
        .map(|l| KnotHash::new().hash(&l))
        .map(|h| h.chars().flat_map(to_binary).collect())
        .collect()
}

fn to_binary(c: char) -> Vec<bool> {
    match c {
        '0' => vec!(false, false, false, false),
        '1' => vec!(false, false, false, true),
        '2' => vec!(false, false, true, false),
        '3' => vec!(false, false, true, true),
        '4' => vec!(false, true, false, false),
        '5' => vec!(false, true, false, true),
        '6' => vec!(false, true, true, false),
        '7' => vec!(false, true, true, true),
        '8' => vec!(true, false, false, false),
        '9' => vec!(true, false, false, true),
        'a' => vec!(true, false, true, false),
        'b' => vec!(true, false, true, true),
        'c' => vec!(true, true, false, false),
        'd' => vec!(true, true, false, true),
        'e' => vec!(true, true, true, false),
        'f' => vec!(true, true, true, true),
        u => panic!("unknown hex character {u}"),
    }
}
