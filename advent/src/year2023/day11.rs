use crate::solver::AdventSolver;

pub struct Advent2023Day11Solver {
    universe: Universe,
}

impl Advent2023Day11Solver {
    pub fn new(input: &str) -> Self {
        Self {
            universe: Universe::from(input),
        }
    }
}

impl AdventSolver for Advent2023Day11Solver {
    fn solve_part1(&self) -> usize {
        self.universe.expand(2).distance_sum()
    }

    fn solve_part2(&self) -> usize {
        self.universe.expand(1000000).distance_sum()
    }
}

type Pos = (usize, usize);

struct Universe {
    galaxies: Vec<Pos>,
}

impl Universe {
    fn expand(&self, speed: usize) -> Self {
        let mut x_expanded: Vec<Pos> = Vec::new();
        let max_x = self.galaxies.iter().map(|g| g.0).max().unwrap();
        let mut x_expansion = 0;
        for x in 0..=max_x {
            let x_galaxies: Vec<&Pos> = self.galaxies.iter().filter(|g| g.0 == x).collect();
            if x_galaxies.is_empty() {
                x_expansion += speed - 1;
            } else {
                x_expanded.extend(x_galaxies.iter().map(|g| (g.0 + x_expansion, g.1)));
            }
        }

        let mut y_expanded: Vec<Pos> = Vec::new();
        let max_y = x_expanded.iter().map(|g| g.1).max().unwrap();
        let mut y_expansion = 0;
        for y in 0..=max_y {
            let y_galaxies: Vec<&Pos> = x_expanded.iter().filter(|g| g.1 == y).collect();
            if y_galaxies.is_empty() {
                y_expansion += speed - 1;
            } else {
                y_expanded.extend(y_galaxies.iter().map(|g| (g.0, g.1 + y_expansion)));
            }
        }

        Self {
            galaxies: y_expanded,
        }
    }

    fn distance_sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                sum += distance(&self.galaxies[i], &self.galaxies[j]);
            }
        }
        sum
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        Self {
            galaxies: value
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '#')
                        .map(move |(x, _)| (x, y))
                })
                .collect(),
        }
    }
}

fn distance(a: &Pos, b: &Pos) -> usize {
    (if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 })
        + (if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn shortest_paths() {
        let solver = Advent2023Day11Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 374);
    }

    #[test]
    fn faster_expansion() {
        let solver = Advent2023Day11Solver::new(EXAMPLE);
        assert_eq!(solver.universe.expand(10).distance_sum(), 1030);
        assert_eq!(solver.universe.expand(100).distance_sum(), 8410);
    }
}
