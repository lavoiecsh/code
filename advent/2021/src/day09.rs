#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day09.txt";

#[allow(dead_code)]
fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let heightmap = read_input();
    let mut sum: usize = 0;
    let imax = heightmap.len();
    let jmax = heightmap[0].len();
    for i in 0..imax {
        for j in 0..jmax {
            if is_low_point(&heightmap, imax, jmax, i, j) {
                sum += heightmap[i][j] as usize - 47;
            }
        }
    }
    sum
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let heightmap = read_input();
    let mut basins: Vec<usize> = Vec::new();
    let imax = heightmap.len();
    let jmax = heightmap[0].len();
    for i in 0..imax {
        for j in 0..jmax {
            if !is_low_point(&heightmap, imax, jmax, i, j) {
                continue;
            }

            basins.push(find_basin(&heightmap, imax, jmax, i, j));
        }
    }
    basins.sort();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

fn is_low_point(heightmap: &Vec<Vec<char>>, imax: usize, jmax: usize, i: usize, j: usize) -> bool {
    let value = heightmap[i][j];
    if i > 0 && heightmap[i-1][j] <= value {
        return false;
    }
    if i < imax-1 && heightmap[i+1][j] <= value {
        return false;
    }
    if j > 0 && heightmap[i][j-1] <= value {
        return false;
    }
    if j < jmax-1 && heightmap[i][j+1] <= value {
        return false;
    }
    true
}

fn find_basin(heightmap: &Vec<Vec<char>>, imax: usize, jmax: usize, i: usize, j: usize) -> usize {
    let mut marked: Vec<(usize, usize)> = vec![(i,j)];
    let mut added: bool = true;
    while added {
        added = false;
        for (i,j) in marked.to_vec() {
            let current = heightmap[i][j];
            if i > 0 && is_in_basin(&heightmap, current, i-1, j, &marked) {
                marked.push((i-1,j));
                added = true;
            }
            if i < imax-1 && is_in_basin(&heightmap, current, i+1, j, &marked) {
                marked.push((i+1, j));
                added = true;
            }
            if j > 0 && is_in_basin(&heightmap, current, i, j-1, &marked) {
                marked.push((i, j-1));
                added = true;
            }
            if j < jmax-1 && is_in_basin(&heightmap, current, i, j+1, &marked) {
                marked.push((i, j+1));
                added = true;
            }
        }
    }
    marked.len()
}

fn is_in_basin(heightmap: &Vec<Vec<char>>, current: char, i: usize, j: usize, marked: &Vec<(usize, usize)>) -> bool {
    let check = heightmap[i][j];
    !marked.contains(&(i,j)) && check != '9' && check > current
}
