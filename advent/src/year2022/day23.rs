use crate::solver::AdventSolver;

type Pos = (isize, isize);

pub struct Advent2022Day23Solver {
    elves: Vec<Pos>,
}

impl Advent2022Day23Solver {
    pub fn new(input: String) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect())
            .collect();
        let mut elves = vec!();
        let half_height: isize = map.len() as isize / 2;
        let half_width: isize = map[0].len() as isize / 2;
        for (row_index, row) in map.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                if col == &'.' { continue; }
                elves.push((row_index as isize - half_height, col_index as isize - half_width));
            }
        }
        Self {
            elves,
        }
    }
}

impl AdventSolver for Advent2022Day23Solver {
    fn solve_part1(&self) -> usize {
        let mut elves = self.elves.clone();
        for i in 0..10 {
            elves = iterate(&elves, i);
        }
        let bounds = bounds(&elves);
        ((bounds.0.1 - bounds.0.0 + 1) * (bounds.1.1 - bounds.1.0 + 1)) as usize - elves.len()
    }

    fn solve_part2(&self) -> usize {
        let mut elves = self.elves.clone();
        let mut tmp_elves = iterate(&elves, 0);
        let mut i = 1;
        while tmp_elves != elves {
            elves = tmp_elves;
            tmp_elves = iterate(&elves, i);
            i += 1;
        }
        i
    }
}

fn bounds(elves: &[Pos]) -> ((isize, isize), (isize, isize)) {
    let rows: Vec<isize> = elves.iter().map(|e| e.0).collect();
    let min_row = *rows.iter().min().unwrap();
    let max_row = *rows.iter().max().unwrap();
    let cols: Vec<isize> = elves.iter().map(|e| e.1).collect();
    let min_col = *cols.iter().min().unwrap();
    let max_col = *cols.iter().max().unwrap();
    ((min_row, max_row), (min_col, max_col))
}

fn _pp(elves: &[Pos]) {
    let bounds = bounds(elves);
    let mut empty = 0;
    for row in bounds.0.0..=bounds.0.1 {
        for col in bounds.1.0..=bounds.1.1 {
            let tile = if elves.contains(&(row, col)) { '#' } else { '.' };
            print!("{}", tile);
            if tile == '.' {
                empty += 1;
            }
        }
        println!();
    }
    println!("Empty: {empty}");
}

fn iterate(elves: &[Pos], round: usize) -> Vec<Pos> {
    let proposals: Vec<Option<Pos>> = elves
        .iter()
        .map(|e| propose_move(e, elves, round))
        .collect();
    elves
        .iter()
        .zip(proposals.clone())
        .map(|(e, p)| match p {
            None => *e,
            Some(x) => if proposals.iter().filter(|x2| &&Some(x) == x2).count() == 1 { x } else { *e },
        })
        .collect()
}

fn propose_move(elf: &Pos, elves: &[Pos], round: usize) -> Option<Pos> {
    let nw = elves.contains(&(elf.0 - 1, elf.1 - 1));
    let n = elves.contains(&(elf.0 - 1, elf.1));
    let ne = elves.contains(&(elf.0 - 1, elf.1 + 1));
    let e = elves.contains(&(elf.0, elf.1 + 1));
    let se = elves.contains(&(elf.0 + 1, elf.1 + 1));
    let s = elves.contains(&(elf.0 + 1, elf.1));
    let sw = elves.contains(&(elf.0 + 1, elf.1 - 1));
    let w = elves.contains(&(elf.0, elf.1 - 1));
    if [nw, n, ne, e, se, s, sw, w].iter().all(|b| !*b) { return None; }
    let mut directions = [
        if !nw && !n && !ne { Some((elf.0 - 1, elf.1)) } else { None },
        if !sw && !s && !se { Some((elf.0 + 1, elf.1)) } else { None },
        if !nw && !w && !sw { Some((elf.0, elf.1 - 1)) } else { None },
        if !ne && !e && !se { Some((elf.0, elf.1 + 1)) } else { None },
    ];
    directions.rotate_left(round % 4);
    directions.iter().find_map(|d| *d)
}
