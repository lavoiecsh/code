use Direction::*;

use crate::solver::AdventSolver;

pub struct Advent2023Day16Solver {
    contraption: Contraption,
}

impl Advent2023Day16Solver {
    pub fn new(input: &str) -> Self {
        Self {
            contraption: Contraption::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2023Day16Solver {
    fn solve_part1(&self) -> usize {
        self.contraption.energized_squares((0, 0, Right))
    }

    fn solve_part2(&self) -> usize {
        let mut highest = 0;
        for y in 0..=self.contraption.max_y {
            let right = self.contraption.energized_squares((0, y, Right));
            if right > highest {
                highest = right;
            }
            let left = self
                .contraption
                .energized_squares((self.contraption.max_x, y, Left));
            if left > highest {
                highest = left;
            }
        }
        for x in 0..=self.contraption.max_x {
            let down = self.contraption.energized_squares((x, 0, Down));
            if down > highest {
                highest = down;
            }
            let up = self
                .contraption
                .energized_squares((x, self.contraption.max_y, Up));
            if up > highest {
                highest = up;
            }
        }
        highest
    }
}

struct Contraption {
    grid: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}

impl Contraption {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self {
            max_x: grid[0].len() - 1,
            max_y: grid.len() - 1,
            grid,
        }
    }

    fn energized_squares(&self, beam: Beam) -> usize {
        let mut energized: Vec<Vec<EnergizedSquare>> = self
            .grid
            .iter()
            .map(|row| row.iter().map(|_| EnergizedSquare::new()).collect())
            .collect();
        let mut beams: Vec<Beam> = vec![beam];
        while let Some(current) = beams.pop() {
            if energized[current.1][current.0].has_evaluated(current.2) {
                continue;
            }
            energized[current.1][current.0].evaluate(current.2);
            beams.extend(self.calculate_next(&current));
        }
        energized
            .iter()
            .map(|row| row.iter().filter(|s| s.is_energized()).count())
            .sum()
    }

    fn calculate_next(&self, (x, y, d): &Beam) -> Vec<Beam> {
        match (self.grid[*y][*x], *d) {
            ('.', Right) | ('-', Right) => {
                if *x < self.max_x {
                    vec![(x + 1, *y, Right)]
                } else {
                    vec![]
                }
            }
            ('.', Down) | ('|', Down) => {
                if *y < self.max_y {
                    vec![(*x, y + 1, Down)]
                } else {
                    vec![]
                }
            }
            ('.', Left) | ('-', Left) => {
                if *x > 0 {
                    vec![(x - 1, *y, Left)]
                } else {
                    vec![]
                }
            }
            ('.', Up) | ('|', Up) => {
                if *y > 0 {
                    vec![(*x, y - 1, Up)]
                } else {
                    vec![]
                }
            }
            ('|', Right) | ('|', Left) => {
                let mut next = vec![];
                if *y > 0 {
                    next.push((*x, y - 1, Up));
                }
                if *y < self.max_y {
                    next.push((*x, y + 1, Down));
                }
                next
            }
            ('-', Down) | ('-', Up) => {
                let mut next = vec![];
                if *x > 0 {
                    next.push((x - 1, *y, Left));
                }
                if *x < self.max_x {
                    next.push((x + 1, *y, Right));
                }
                next
            }
            ('/', Right) | ('\\', Left) => {
                if *y > 0 {
                    vec![(*x, y - 1, Up)]
                } else {
                    vec![]
                }
            }
            ('/', Up) | ('\\', Down) => {
                if *x < self.max_x {
                    vec![(x + 1, *y, Right)]
                } else {
                    vec![]
                }
            }
            ('/', Left) | ('\\', Right) => {
                if *y < self.max_y {
                    vec![(*x, y + 1, Down)]
                } else {
                    vec![]
                }
            }
            ('/', Down) | ('\\', Up) => {
                if *x > 0 {
                    vec![(x - 1, *y, Left)]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }
}

struct EnergizedSquare {
    directions: [bool; 4],
}

impl EnergizedSquare {
    fn new() -> Self {
        Self {
            directions: [false; 4],
        }
    }

    fn has_evaluated(&self, direction: Direction) -> bool {
        self.directions[direction as usize]
    }

    fn evaluate(&mut self, direction: Direction) {
        self.directions[direction as usize] = true;
    }

    fn is_energized(&self) -> bool {
        self.directions.iter().any(|&d| d)
    }
}

type Beam = (usize, usize, Direction);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";

    #[test]
    fn counts_energized_squares() {
        let solver = Advent2023Day16Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 46);
    }

    #[test]
    fn finds_best_energizing() {
        let solver = Advent2023Day16Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 51);
    }
}
