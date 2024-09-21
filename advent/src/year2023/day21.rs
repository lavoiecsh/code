use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day21Solver {
    step_counter: StepCounter,
}

impl Advent2023Day21Solver {
    pub fn new(input: String) -> Self {
        let mut starting_position = (0, 0);
        let grid = input.lines()
            .enumerate()
            .map(|(y, row)| row.chars().enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '#' => false,
                    'S' => {
                        starting_position = (x, y);
                        true
                    }
                    _ => panic!("unknown grid character {c}"),
                })
                .collect())
            .collect();
        Self { step_counter: StepCounter::new(grid, starting_position) }
    }
}

impl AdventSolver for Advent2023Day21Solver {
    fn solve_part1(&self) -> usize {
        self.step_counter.step_many(64)
    }

    fn solve_part2(&self) -> usize {
        /* todo
        self.step_counter.step_many(26501365) doesn't return the correct answer
        using repeating pattern to calculate instead
        26501365 = 202300 * grid_size + 65
        quadratic equation using steps:
        - 65: edge of center grid
        - 196: edge of second grid = 65 + 131
        - 327: edge of third grid = 65 + 131 + 131
         */
        let steps_65 = self.step_counter.step_many(65);
        let steps_196 = self.step_counter.step_many(196);
        let steps_327 = self.step_counter.step_many(327);
        let n = 26501365 / 131;
        let a = (steps_327 + steps_65 - 2 * steps_196) / 2;
        let b = steps_196 - steps_65 - a;
        let c = steps_65;
        a * n * n + b * n + c
    }
}

type GridPos = (i32, i32);
type Pos = (usize, usize);

struct StepCounter {
    grid: Vec<Vec<bool>>,
    distances: HashMap<Pos, usize>,
    max_x: usize,
    max_y: usize,
    open_even: usize,
    open_odd: usize,
    open_all: usize,
}

impl StepCounter {
    fn new(grid: Vec<Vec<bool>>, starting_position: Pos) -> Self {
        let max_x = grid[0].len() - 1;
        let max_y = grid.len() - 1;
        let mut distances = HashMap::new();
        distances.insert(starting_position, 0);
        let mut queue = VecDeque::new();
        queue.push_back((starting_position, 0));
        while let Some((current_pos, current_dist)) = queue.pop_front() {
            let other = vec!(
                if current_pos.0 == 0 { None } else { Some((current_pos.0 - 1, current_pos.1)) },
                if current_pos.0 == max_x { None } else { Some((current_pos.0 + 1, current_pos.1)) },
                if current_pos.1 == 0 { None } else { Some((current_pos.0, current_pos.1 - 1)) },
                if current_pos.1 == max_y { None } else { Some((current_pos.0, current_pos.1 + 1)) },
            )
                .into_iter()
                .flatten()
                .filter(|&(x, y)| grid[y][x])
                .filter(|p| !distances.contains_key(p))
                .map(|p| (p, current_dist + 1))
                .collect_vec();
            queue.extend(other.clone());
            distances.extend(other);
        }
        let open_even = distances.values().filter(|&d| d % 2 == 0).count();
        let open_odd = distances.values().filter(|&d| d % 2 == 1).count();

        Self {
            grid,
            max_x,
            max_y,
            open_even,
            open_odd,
            open_all: open_even + open_odd,
            distances,
        }
    }

    fn step_many(&self, step_count: usize) -> usize {
        let parity = step_count % 2;
        let grid_max = usize::max(step_count / (self.max_x + 1), step_count / (self.max_y + 1));
        let mut grid_min = if grid_max > 3 { grid_max - 4 } else { 0 };
        if grid_min % 2 == 1 { grid_min -= 1; }

        let mut count = 0;
        count += self.calculate_center(step_count, parity);

        count += self.calculate_top(grid_min, grid_max, step_count, parity);
        count += self.calculate_bottom(grid_min, grid_max, step_count, parity);

        count += self.calculate_left(grid_min, grid_max, step_count, parity);
        count += self.calculate_right(grid_min, grid_max, step_count, parity);

        count += grid_min * 2 * self.open_all;

        for iteration in usize::max(grid_min, 1)..=grid_max {
            let top_left = self.calculate_diagonal((-(iteration as i32), -1), step_count, parity);
            count += top_left * iteration;

            let bottom_left = self.calculate_diagonal((-(iteration as i32), 1), step_count, parity);
            count += bottom_left * iteration;

            let top_right = self.calculate_diagonal((iteration as i32, -1), step_count, parity);
            count += top_right * iteration;

            let bottom_right = self.calculate_diagonal((iteration as i32, 1), step_count, parity);
            count += bottom_right * iteration;
        }

        let inside_iterations = grid_min / 2;
        let even = (0..inside_iterations).map(|i| i * 2 + 1).sum::<usize>() * 4;
        let odd = (1..inside_iterations).map(|i| i * 2).sum::<usize>() * 4;
        count += even * self.open_even + odd * self.open_odd;

        count
    }

    fn calculate_left(&self, grid_min: usize, grid_max: usize, step_count: usize, parity: usize) -> usize {
        self.calculate_horizontal(grid_min, grid_max, step_count, parity, self.max_y, 0)
    }

    fn calculate_right(&self, grid_min: usize, grid_max: usize, step_count: usize, parity: usize) -> usize {
        self.calculate_horizontal(grid_min, grid_max, step_count, parity, 0, self.max_y)
    }

    fn calculate_horizontal(&self, grid_min: usize, grid_max: usize, step_count: usize, parity: usize, next_col: usize, this_col: usize) -> usize {
        let mut init = self.build_init_vec_horizontal(grid_min, next_col, this_col);
        let mut count = 0;
        for _ in grid_min..=grid_max {
            let distances = self.calculate_grid(init.into_iter(), step_count);
            count += count_steps(&distances, step_count, parity);
            init = (0..=self.max_y).map(|y| ((next_col, y), distances.get(&(this_col, y)).unwrap_or(&(step_count + 1)) + 1)).collect_vec();
        }
        count
    }

    fn build_init_vec_horizontal(&self, grid_distance: usize, next_col: usize, this_col: usize) -> Vec<(Pos, usize)> {
        if grid_distance == 0 {
            (0..=self.max_y).map(|y| ((next_col, y), self.distances.get(&(this_col, y)).unwrap() + 1)).collect_vec()
        } else {
            vec!(
                ((next_col, 0), self.distances.get(&(this_col, 0)).unwrap() + grid_distance * (self.max_x + 1) + 1),
                ((next_col, self.max_y), self.distances.get(&(this_col, self.max_y)).unwrap() + grid_distance * (self.max_x + 1) + 1),
            )
        }
    }

    fn calculate_top(&self, grid_min: usize, grid_max: usize, step_count: usize, parity: usize) -> usize {
        self.calculate_vertical(grid_min, grid_max, step_count, parity, self.max_y, 0)
    }

    fn calculate_bottom(&self, grid_min: usize, grid_max: usize, step_count: usize, parity: usize) -> usize {
        self.calculate_vertical(grid_min, grid_max, step_count, parity, 0, self.max_y)
    }

    fn calculate_vertical(&self, grid_min: usize, grid_max: usize, step_count: usize, parity: usize, next_row: usize, this_row: usize) -> usize {
        let mut init = self.build_init_vec_vertical(grid_min, next_row, this_row);
        let mut count = 0;
        for _ in grid_min..=grid_max {
            let distances = self.calculate_grid(init.into_iter(), step_count);
            count += count_steps(&distances, step_count, parity);
            init = (0..=self.max_x).map(|x| ((x, next_row), distances.get(&(x, this_row)).unwrap_or(&(step_count + 1)) + 1)).collect_vec();
        }
        count
    }

    fn build_init_vec_vertical(&self, grid_distance: usize, next_row: usize, this_row: usize) -> Vec<(Pos, usize)> {
        if grid_distance == 0 {
            (0..=self.max_x).map(|x| ((x, next_row), self.distances.get(&(x, this_row)).unwrap() + 1)).collect_vec()
        } else {
            vec!(
                ((0, next_row), self.distances.get(&(0, this_row)).unwrap() + grid_distance * (self.max_y + 1) + 1),
                ((self.max_x, next_row), self.distances.get(&(self.max_x, this_row)).unwrap() + grid_distance * (self.max_y + 1) + 1),
            )
        }
    }

    fn calculate_diagonal(&self, grid_pos: GridPos, step_count: usize, parity: usize) -> usize {
        let (starting_pos, distance_pos) = match (grid_pos.0.signum(), grid_pos.1.signum()) {
            (-1, -1) => ((self.max_x, self.max_y), (0, 0)),
            (-1, 1) => ((self.max_x, 0), (0, self.max_y)),
            (1, -1) => ((0, self.max_y), (self.max_x, 0)),
            (1, 1) => ((0, 0), (self.max_x, self.max_y)),
            (x, y) => panic!("unknown signs {x} - {y}"),
        };
        let grid_step = (grid_pos.0.unsigned_abs() as usize - 1) * self.max_x +
            (grid_pos.1.unsigned_abs() as usize - 1) * self.max_y +
            grid_pos.0.unsigned_abs() as usize +
            grid_pos.1.unsigned_abs() as usize;
        let init = vec!((starting_pos, self.distances.get(&distance_pos).unwrap() + grid_step)).into_iter();
        let distances = self.calculate_grid(init, step_count);
        count_steps(&distances, step_count, parity)
    }

    fn calculate_center(&self, step_count: usize, parity: usize) -> usize {
        count_steps(&self.distances, step_count, parity)
    }

    fn calculate_grid(&self, init: impl Iterator<Item=(Pos, usize)>, step_count: usize) -> HashMap<Pos, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        for (pos, dist) in init {
            distances.insert(pos, dist);
            queue.push_back((pos, dist));
        }
        while let Some((current_pos, current_count)) = queue.pop_front() {
            let others = self.around(&current_pos)
                .filter(|p| *distances.get(p).unwrap_or(&usize::MAX) > current_count + 1)
                .map(|p| (p, current_count + 1))
                .collect_vec();
            if current_count < step_count {
                queue.extend(others.clone());
            }
            distances.extend(others);
        }
        distances
    }

    fn around(&self, pos: &Pos) -> impl Iterator<Item=Pos> {
        let mut around = vec!();
        if pos.0 > 0 && self.grid[pos.1][pos.0 - 1] {
            around.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.max_x && self.grid[pos.1][pos.0 + 1] {
            around.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 && self.grid[pos.1 - 1][pos.0] {
            around.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.max_y && self.grid[pos.1 + 1][pos.0] {
            around.push((pos.0, pos.1 + 1));
        }
        around.into_iter()
    }

    fn _pp(&self, distances: &HashMap<Pos, usize>) {
        let mut tmp = vec!();
        for y in 0..=self.max_y {
            let mut tmp_row = vec!();
            for x in 0..=self.max_x {
                if self.grid[y][x] {
                    if let Some(d) = distances.get(&(x, y)) {
                        tmp_row.push(format!("{:2}", d % 100));
                    } else {
                        tmp_row.push("__".to_string());
                    }
                } else {
                    tmp_row.push("##".to_string());
                }
            }
            tmp.push(tmp_row.join(""));
        }
    }
}

fn count_steps(distances: &HashMap<Pos, usize>, step_count: usize, parity: usize) -> usize {
    distances.values()
        .filter(|&d| *d <= step_count && d % 2 == parity)
        .count()
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day21Solver {
    Advent2023Day21Solver::new(String::from("\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"))
}

#[test]
fn reachable_spots() {
    let solver = test_solver_1();

    assert_eq!(solver.step_counter.step_many(6), 16);
    assert_eq!(solver.step_counter.step_many(10), 50);
    assert_eq!(solver.step_counter.step_many(50), 1594);
    assert_eq!(solver.step_counter.step_many(100), 6536);
    assert_eq!(solver.step_counter.step_many(500), 167004);
    assert_eq!(solver.step_counter.step_many(1000), 668697);
    assert_eq!(solver.step_counter.step_many(5000), 16733044);
}
