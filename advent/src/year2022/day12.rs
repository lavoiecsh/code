use std::fs::read_to_string;
use crate::solver::AdventSolver;

type Pos = (usize, usize);

pub struct Advent2022Day12Solver {
    elevation_map: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
}

impl Advent2022Day12Solver {
    pub fn new() -> Self {
        let mut start = (0,0);
        let mut end = (0,0);
        let mut elevation_map = Vec::new();
        let mut row = 0;
        for line in read_to_string("src/year2022/day12.txt").unwrap().lines() {
            let mut line_map = Vec::new();
            let mut column = 0;
            for c in line.chars() {
                match c {
                    'S' => {
                        start = (row, column);
                        line_map.push('a' as u8);
                    },
                    'E' => {
                        end = (row, column);
                        line_map.push('z' as u8);
                    },
                    x => line_map.push(x as u8),
                }
                column += 1;
            }
            elevation_map.push(line_map);
            row += 1;
        }
        Self {
            elevation_map,
            start,
            end,
        }
    }

    fn elevation(&self, pos: &Pos) -> u8 {
        self.elevation_map[pos.0][pos.1]
    }

    fn orthogonal_moves(&self, pos: &Pos) -> Vec<Pos> {
        let mut moves = vec!();
        if pos.0 > 0 {
            moves.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.elevation_map.len() - 1 {
            moves.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            moves.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.elevation_map[0].len() - 1 {
            moves.push((pos.0, pos.1 + 1));
        }
        moves
    }
}

impl AdventSolver for Advent2022Day12Solver {
    fn day(&self) -> usize { 12 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        let max_i = self.elevation_map.len();
        let max_j = self.elevation_map[0].len();
        let mut move_counts: Vec<Vec<usize>> = vec!();
        for i in 0..max_i {
            let mut line = vec!();
            for j in 0..max_j {
                line.push(if (i,j) == self.start { 0 } else { usize::MAX });
            }
            move_counts.push(line);
        }
        while move_counts[self.end.0][self.end.1] == usize::MAX {
            for i in 0..max_i {
                for j in 0..max_j {
                    if move_counts[i][j] != usize::MAX {
                        continue;
                    }
                    let this_pos = (i,j);
                    let this_elevation = self.elevation(&this_pos);
                    let best_move = self.orthogonal_moves(&this_pos)
                        .iter()
                        .filter(|o| move_counts[o.0][o.1] != usize::MAX)
                        .filter(|o| this_elevation <= self.elevation(o) + 1)
                        .map(|o| move_counts[o.0][o.1])
                        .min();
                    if best_move.is_some() {
                        move_counts[i][j] = best_move.unwrap() + 1;
                    }
                }
            }
        }
        move_counts[self.end.0][self.end.1]
    }

    fn solve_part2(&self) -> usize {
        let max_i = self.elevation_map.len();
        let max_j = self.elevation_map[0].len();
        let mut move_counts: Vec<Vec<usize>> = vec!();
        let mut a_pos: Vec<Pos> = vec!();
        for i in 0..max_i {
            let mut line = vec!();
            for j in 0..max_j {
                line.push(if (i,j) == self.end { 0 } else { usize::MAX });
                if self.elevation_map[i][j] == 'a' as u8 {
                    a_pos.push((i,j));
                }
            }
            move_counts.push(line);
        }
        let mut touched = true;
        while touched {
            touched = false;
            for i in 0..max_i {
                for j in 0..max_j {
                    if move_counts[i][j] == usize::MAX {
                        continue;
                    }
                    let this_pos = (i,j);
                    let this_elevation = self.elevation(&this_pos);
                    let this_move_count = move_counts[i][j];
                    let new_moves: Vec<Pos> = self.orthogonal_moves(&this_pos)
                        .iter()
                        .filter(|o| move_counts[o.0][o.1] == usize::MAX)
                        .filter(|o| this_elevation <= self.elevation(o) + 1)
                        .map(|o| o.clone())
                        .collect();
                    for nm in new_moves {
                        move_counts[nm.0][nm.1] = this_move_count + 1;
                        touched = true;
                    }
                }
            }
        }
        a_pos
            .iter()
            .map(|ap| move_counts[ap.0][ap.1])
            .min()
            .unwrap()
    }
}
