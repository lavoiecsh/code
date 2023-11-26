use crate::solver::AdventSolver;

pub struct Advent2018Day11Solver {
    grid: Grid,
}

impl Advent2018Day11Solver {
    pub fn new(input: String) -> Self {
        Self { grid: Grid::new(input.parse().unwrap()) }
    }
}

impl AdventSolver for Advent2018Day11Solver {
    fn solve_part1_string(&self) -> String {
        let (largest,_) = self.grid.largest(3);
        format!("{},{}", largest.0 + 1, largest.1 + 1)
    }

    fn solve_part2_string(&self) -> String {
        let ((largest,_),size) = self.grid.largest_any();
        format!("{},{},{}", largest.0 + 1, largest.1 + 1, size)
    }
}

struct Grid {
    grid: Vec<Vec<isize>>,
}

impl Grid {
    fn new(serial_number: usize) -> Self {
        let grid = (0..300)
            .map(|y| (0..300).map(|x| {
                let rack_id = x + 11;
                let mut power: isize = rack_id * (y + 1);
                power += serial_number as isize;
                power *= rack_id;
                power /= 100;
                power %= 10;
                power - 5
            }).collect())
            .collect();
        Self { grid }
    }

    fn largest_any(&self) -> (((usize, usize), isize), usize) {
        let mut sum_grid: Vec<Vec<isize>> = self.grid.clone();
        let mut best: (((usize, usize), isize), usize) = (sum_grid.iter()
            .enumerate()
            .map(|(y,row)| (y,row.iter().enumerate().max_by_key(|(_,s)| *s).unwrap()))
            .map(|(y,(x,s))| ((x,y),*s))
            .max_by_key(|(_,s)| *s)
            .unwrap(), 1);
        for size in 1..300 {
            for y in 0..300-size {
                for x in 0..300-size {
                    for i in 0..size {
                        sum_grid[y][x] += self.grid[y+size][x+i] + self.grid[y+i][x+size];
                    }
                    sum_grid[y][x] += self.grid[y+size][x+size];
                    if sum_grid[y][x] > best.0.1 {
                        best = (((x,y),sum_grid[y][x]),size+1);
                    }
                }
            }
        }
        best
    }

    fn largest(&self, size: usize) -> ((usize, usize), isize) {
        (0..300-size)
            .flat_map(|y| (0..300-size).map(move |x| self.total((x.clone(), y.clone()), size)))
            .max_by_key(|(_, t)| *t)
            .unwrap()
    }

    fn total(&self, position: (usize, usize), size: usize) -> ((usize, usize), isize) {
        // dbg!(format_args!("{:3}, ({:3}, {:3})", size, position.0, position.1));
        (position,
         (0..size)
             .map(|y| (0..size).map(|x| self.grid[y + position.1][x + position.0]).sum::<isize>())
             .sum())
    }
}
