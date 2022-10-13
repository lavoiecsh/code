#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day04.txt";

#[derive(Clone)]
struct Tile {
    number: usize,
    marked: bool,
}

type Board = Vec<Tile>;

#[allow(dead_code)]
fn read_input() -> (Vec<usize>, Vec<Board>) {
    let txt = fs::read_to_string(FILENAME).expect("error reading");
    let mut lines = txt.trim().lines();
    
    let numbers: Vec<usize> = lines.next().unwrap().trim().split(",").map(|n|n.parse().expect("error parsing")).collect();
    let mut boards: Vec<Board> = Vec::new();
    while lines.next().is_some() {
        let mut tiles: Vec<Tile> = Vec::new();
        for _i in 0..5 {
            let line = lines.next().unwrap();
            debug!(line);
            let numbers: Vec<usize> = line
                .split(" ")
                .map(|s|s.trim())
                .filter(|s|s.len() >= 1)
                .map(|s|{ debug!(s); s.parse().expect("error parsing") })
                .collect();
            let ts: Vec<Tile> = numbers.iter()
                .map(|n|Tile { number: *n, marked: false })
                .collect();
            for t in ts {
                tiles.push(t);
            }
        }
        boards.push(tiles);
    }
    (numbers, boards)
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let (numbers, mut boards) = read_input();
    for n in numbers {
        boards.iter_mut().for_each(|b|mark(b,n));
        let winning_boards: Vec<Board> = boards.iter().filter(|b|is_winning(&b)).cloned().collect();
        assert!(winning_boards.len() <= 1);
        if winning_boards.len() == 1 {
            return score(&winning_boards[0], n);
        }
    }
    panic!("no solution found")
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let (numbers, mut boards) = read_input();
    for n in numbers {
        boards.iter_mut().for_each(|b|mark(b,n));
        if boards.len() == 1 {
            if is_winning(&boards[0]) {
                return score(&boards[0], n);
            }
        }
        boards = boards.iter().filter(|b|!is_winning(b)).cloned().collect();
    }
    panic!("no solution found")
}

fn mark(board: &mut Board, number: usize) {
    for t in board {
        if t.number == number {
            t.marked = true;
        }
    }
}

fn score(board: &Board, last: usize) -> usize {
    last * board.iter().fold(0, |acc,t| acc + if t.marked { 0 } else { t.number })
}

fn is_winning(board: &Board) -> bool {
    for i in 0..5 {
        let mut row_winning: bool = true;
        let mut col_winning: bool = true;
        for j in 0..5 {
            if !board[i*5+j].marked {
                row_winning = false;
            }
            if !board[i+j*5].marked {
                col_winning = false;
            }
        }
        if row_winning || col_winning {
            return true;
        }
    }
    false
}
