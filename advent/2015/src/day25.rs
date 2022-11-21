const FIRST: usize = 20151125;
const MULTIPLIER: usize = 252533;
const MODULO: usize = 33554393;

const ROW: usize = 2947;
const COLUMN: usize = 3029;

pub fn part1() -> usize {
    let mut number = FIRST;
    for row in 2..(ROW + COLUMN) {
        for col in 0..row {
            number = iterate(number);
            if (row - col) == ROW && (col + 1) == COLUMN {
                return number;
            }
        }
    }
    0
}

fn iterate(number: usize) -> usize {
    (number * MULTIPLIER) % MODULO
}

pub fn part2() -> usize {
    0
}
