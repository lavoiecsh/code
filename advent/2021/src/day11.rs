#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day11.txt";

#[allow(dead_code)]
fn read_input() -> Vec<Vec<u8>> {
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(|l|l.chars().map(|c|c as u8 - 48).collect())
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let mut levels = read_input();
    let imax = levels.len();
    let jmax = levels[0].len();
    let mut total: usize = 0;
    for i in 0..100 {
        debug!((i, pretty_print(&levels)));
        total += step(&mut levels, imax, jmax);
    }
    total
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let mut levels = read_input();
    let imax = levels.len();
    let jmax = levels[0].len();
    let mut iter: usize = 0;
    while !all_flashed(&levels) {
        step(&mut levels, imax, jmax);
        iter += 1;
    }
    iter
}

fn pretty_print(levels: &Vec<Vec<u8>>) -> Vec<String> {
    levels.iter().map(|row|row.iter().map(|c|c.to_string()).collect::<String>()).collect()
}

fn pretty_print_2(levels: &Vec<Vec<(u8, bool)>>) -> Vec<String> {
    levels.iter().map(|row|row.iter().map(|(v,_)|v.to_string() + " ").collect::<String>()).collect()
}

fn all_flashed(levels: &Vec<Vec<u8>>) -> bool {
    levels.iter().all(|row| row.iter().all(|c|*c == 0))
}

fn step(levels: &mut Vec<Vec<u8>>, imax: usize, jmax: usize) -> usize {
    let mut tmp: Vec<Vec<(u8,bool)>> = levels.iter().map(|row|row.iter().map(|col|(col + 1, false)).collect()).collect();
    let mut modified = true;
    while modified {
        modified = false;
        for i in 0..imax {
            for j in 0..jmax {
                let (v,f) = tmp[i][j];
                if v > 9 && !f {
                    modified = true;
                    tmp[i][j] = (v, true);
                    increase_around(&mut tmp, imax, jmax, i, j);
                    debug!((i,j, pretty_print_2(&tmp)));
                }
            }
        }
    }
    let mut count: usize = 0;
    for i in 0..imax {
        for j in 0..jmax {
            let (v,f) = tmp[i][j];
            if f {
                count += 1;
            }
            levels[i][j] = if f { 0 } else { v };
        }
    }
    count
}

fn increase_around(levels: &mut Vec<Vec<(u8, bool)>>, imax: usize, jmax: usize, i: usize, j: usize) {
    let min_x = if i == 0 { 0 } else { i - 1 };
    let max_x = if i == imax-1 { i } else { i + 1 };
    let min_y = if j == 0 { 0 } else { j - 1 };
    let max_y = if j == jmax-1 { j } else { j + 1 };
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if x == i && y == j {
                continue;
            }
            let (v,f) = levels[x][y];
            debug!((i,j,x,y,pretty_print_2(&levels)));
            levels[x][y] = (v+1, f);
        }
    }
}
