use crate::solver::AdventSolver;

pub struct Advent2023Day14Solver {
    dish: Dish,
}

impl Advent2023Day14Solver {
    pub fn new(input: String) -> Self {
        Self {
            dish: Dish {
                grid: input.lines()
                    .map(|l| l.chars().collect())
                    .collect()
            }
        }
    }
}

impl AdventSolver for Advent2023Day14Solver {
    fn solve_part1(&self) -> usize {
        let mut dish = self.dish.clone();
        dish.move_north();
        dish.load_on_north_support_beam()
    }

    fn solve_part2(&self) -> usize {
        let mut dish = self.dish.clone();
        dish.cycle_many(1000000000);
        dish.load_on_north_support_beam()
    }
}

#[derive(Clone)]
struct Dish {
    grid: Vec<Vec<char>>,
}

impl Dish {
    fn load_on_north_support_beam(&self) -> usize {
        let mut load = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == 'O' {
                    load += self.grid.len() - y;
                }
            }
        }
        load
    }

    fn cycle_many(&mut self, total: usize) {
        let mut seen: Vec<Vec<Vec<char>>> = vec!();
        self.cycle();
        let mut count = 0;
        while !seen.contains(&self.grid) {
            seen.push(self.grid.clone());
            self.cycle();
            count += 1;
        }
        let found = seen.iter().position(|g| g == &self.grid).unwrap();
        let remaining = (total - found) % (count - found);
        self.grid = seen[found + remaining - 1].clone();
    }

    fn cycle(&mut self) {
        self.move_north();
        self.move_west();
        self.move_south();
        self.move_east();
    }

    fn move_north(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            for y in 1..self.grid.len() {
                for x in 0..self.grid[y].len() {
                    if self.grid[y][x] != 'O' { continue; }
                    if self.grid[y - 1][x] == '.' {
                        self.grid[y][x] = '.';
                        self.grid[y - 1][x] = 'O';
                        moved = true;
                    }
                }
            }
        }
    }

    fn move_south(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            for y in 0..self.grid.len() - 1 {
                for x in 0..self.grid[y].len() {
                    if self.grid[y][x] != 'O' { continue; }
                    if self.grid[y + 1][x] == '.' {
                        self.grid[y][x] = '.';
                        self.grid[y + 1][x] = 'O';
                        moved = true;
                    }
                }
            }
        }
    }

    fn move_west(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            for y in 0..self.grid.len() {
                for x in 1..self.grid[y].len() {
                    if self.grid[y][x] != 'O' { continue; }
                    if self.grid[y][x - 1] == '.' {
                        self.grid[y][x] = '.';
                        self.grid[y][x - 1] = 'O';
                        moved = true;
                    }
                }
            }
        }
    }

    fn move_east(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            for y in 0..self.grid.len() {
                for x in 0..self.grid[y].len() - 1 {
                    if self.grid[y][x] != 'O' { continue; }
                    if self.grid[y][x + 1] == '.' {
                        self.grid[y][x] = '.';
                        self.grid[y][x + 1] = 'O';
                        moved = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day14Solver {
    Advent2023Day14Solver::new(String::from("\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"))
}

#[test]
fn calculates_load_on_north_support_beam() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part1(), 136);
}

#[test]
fn calculates_many_cycles() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 64);
}
