use crate::solver::AdventSolver;

type Pos = (usize, usize);

pub struct Advent2022Day12Solver {
    elevation_map: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
}

impl Advent2022Day12Solver {
    pub fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut elevation_map = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let mut line_map = Vec::new();
            for (column, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (row, column);
                        line_map.push(b'a');
                    }
                    'E' => {
                        end = (row, column);
                        line_map.push(b'z');
                    }
                    x => line_map.push(x as u8),
                }
            }
            elevation_map.push(line_map);
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
        let mut moves = vec![];
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
    fn solve_part1(&self) -> usize {
        let max_i = self.elevation_map.len();
        let max_j = self.elevation_map[0].len();
        let mut move_counts: Vec<Vec<usize>> = vec![];
        for i in 0..max_i {
            let mut line = vec![];
            for j in 0..max_j {
                line.push(if (i, j) == self.start { 0 } else { usize::MAX });
            }
            move_counts.push(line);
        }
        while move_counts[self.end.0][self.end.1] == usize::MAX {
            for i in 0..max_i {
                for j in 0..max_j {
                    if move_counts[i][j] != usize::MAX {
                        continue;
                    }
                    let this_pos = (i, j);
                    let this_elevation = self.elevation(&this_pos);
                    let best_move = self
                        .orthogonal_moves(&this_pos)
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
        let mut move_counts: Vec<Vec<usize>> = vec![];
        let mut a_pos: Vec<Pos> = vec![];
        for i in 0..max_i {
            let mut line = vec![];
            for j in 0..max_j {
                line.push(if (i, j) == self.end { 0 } else { usize::MAX });
                if self.elevation_map[i][j] == b'a' {
                    a_pos.push((i, j));
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
                    let this_pos = (i, j);
                    let this_elevation = self.elevation(&this_pos);
                    let this_move_count = move_counts[i][j];
                    let new_moves: Vec<Pos> = self
                        .orthogonal_moves(&this_pos)
                        .iter()
                        .filter(|o| move_counts[o.0][o.1] == usize::MAX)
                        .filter(|o| this_elevation <= self.elevation(o) + 1)
                        .copied()
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
