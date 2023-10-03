use crate::solver::AdventSolver;

pub struct Advent2022Day08Solver {
    trees: Vec<Vec<u8>>,
}

impl Advent2022Day08Solver {
    pub fn new(input: String) -> Self {
        Self {
            trees: input
                .lines()
                .map(|l| l.chars().map(|c| String::from(c).parse().unwrap()).collect())
                .collect()
        }
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        let value = self.trees[i][j];
        let max = self.trees.len();
        let mut top = 0;
        for x in (0..i).rev() {
            if self.trees[x][j] < value {
                top += 1;
                continue;
            }
            top += 1;
            break;
        }
        let mut bottom = 0;
        for x in i+1..max {
            if self.trees[x][j] < value {
                bottom += 1;
                continue;
            }
            bottom += 1;
            break;
        }
        let mut left = 0;
        for x in (0..j).rev() {
            if self.trees[i][x] < value {
                left += 1;
                continue;
            }
            left += 1;
            break;
        }
        let mut right = 0;
        for x in j+1..max {
            if self.trees[i][x] < value {
                right += 1;
                continue;
            }
            right += 1;
            break;
        }
        top * bottom * left * right
    }

    fn scenic_scores(&self) -> Vec<Vec<usize>> {
        let mut scores = Vec::new();
        let max = self.trees.len() - 1;
        for i in 0..=max {
            scores.push(Vec::new());
            for j in 0..=max {
                scores[i].push(self.scenic_score(i, j));
            }
        }
        scores
    }
}

impl AdventSolver for Advent2022Day08Solver {
    fn solve_part1(&self) -> usize {
        let mut visible: Vec<Vec<bool>> = Vec::new();
        let max = self.trees.len() - 1;
        for i in 0..=max {
            visible.push(Vec::new());
            for j in 0..=max {
                visible[i].push(i == 0 || i == max || j == 0 || j == max);
            }
        }
        for i in 0..=max {
            let mut highest_row = 0;
            let mut highest_col = 0;
            for j in 0..=max {
                if self.trees[i][j] > highest_row {
                    highest_row = self.trees[i][j];
                    visible[i][j] = true;
                }
                if self.trees[j][i] > highest_col {
                    highest_col = self.trees[j][i];
                    visible[j][i] = true;
                }
            }
            highest_row = 0;
            highest_col = 0;
            for j in (0..=max).rev() {
                if self.trees[i][j] > highest_row {
                    highest_row = self.trees[i][j];
                    visible[i][j] = true;
                }
                if self.trees[j][i] > highest_col {
                    highest_col = self.trees[j][i];
                    visible[j][i] = true;
                }
            }
        }
        visible
            .iter()
            .map(|l| l.iter().filter(|v| **v).count())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.scenic_scores()
            .iter()
            .map(|l| *l.iter().max().unwrap())
            .max()
            .unwrap()
    }
}
