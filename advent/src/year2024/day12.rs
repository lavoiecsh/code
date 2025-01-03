use crate::solver::AdventSolver;
use std::collections::VecDeque;

pub struct Advent2024Day12Solver {
    garden: Garden,
}

impl Advent2024Day12Solver {
    pub fn new(input: &str) -> Self {
        Self {
            garden: Garden::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2024Day12Solver {
    fn solve_part1(&self) -> usize {
        self.garden
            .regions
            .iter()
            .map(|r| r.area() * r.perimeter())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.garden
            .regions
            .iter()
            .map(|r| r.area() * r.sides())
            .sum()
    }
}

#[derive(Debug)]
struct Garden {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    regions: Vec<Region>,
}

impl Garden {
    fn new(map: Vec<Vec<char>>) -> Self {
        let height = map.len();
        let width = map[0].len();
        let mut garden = Self {
            map,
            height,
            width,
            regions: Vec::new(),
        };
        garden.build_regions();
        garden
    }

    fn build_regions(&mut self) {
        let mut seen = Vec::with_capacity(self.height * self.width);
        for y in 0..self.height {
            for x in 0..self.width {
                if seen.contains(&(y, x)) {
                    continue;
                }
                let plant = self.plant((y, x));
                let plots = self.grow_plot((y, x), plant);
                seen.extend(plots.iter());
                self.regions.push(Region::new(plots));
            }
        }
    }

    fn grow_plot(&self, pos: Pos, plant: char) -> Vec<Pos> {
        let mut plots = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(pos);
        while let Some(c) = queue.pop_front() {
            if plots.contains(&c) {
                continue;
            }
            plots.push(c);
            queue.extend(
                self.around(c)
                    .iter()
                    .filter(|&p| !plots.contains(p) && self.plant(*p) == plant),
            );
        }
        plots
    }

    fn around(&self, pos: Pos) -> Vec<Pos> {
        let mut a = Vec::new();
        if pos.0 > 0 {
            a.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.height - 1 {
            a.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            a.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.width - 1 {
            a.push((pos.0, pos.1 + 1));
        }
        a
    }

    fn plant(&self, pos: Pos) -> char {
        self.map[pos.0][pos.1]
    }
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Region {
    plots: Vec<Pos>,
}

impl Region {
    fn new(plots: Vec<Pos>) -> Self {
        Self { plots }
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.outsides().iter().map(|o| o.len()).sum()
    }

    fn sides(&self) -> usize {
        let [up, down, left, right] = self.outsides();
        let x_bound = |(_, x)| x == 0;
        let y_bound = |(y, _)| y == 0;
        let x_lower = |(y, x)| (y, x - 1);
        let x_upper = |(y, x)| (y, x + 1);
        let y_lower = |(y, x)| (y - 1, x);
        let y_upper = |(y, x)| (y + 1, x);
        count_sides(up, x_bound, x_lower, x_upper) +
            count_sides(down, x_bound, x_lower, x_upper) +
            count_sides(left, y_bound, y_lower, y_upper) +
            count_sides(right, y_bound, y_lower, y_upper)
    }

    fn outsides(&self) -> [Vec<Pos>; 4] {
        let mut up = Vec::new();
        let mut down = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();
        for &plot in &self.plots {
            if plot.0 == 0 || !self.plots.contains(&(plot.0 - 1, plot.1)) {
                up.push(plot);
            }
            if !self.plots.contains(&(plot.0 + 1, plot.1)) {
                down.push(plot);
            }
            if plot.1 == 0 || !self.plots.contains(&(plot.0, plot.1 - 1)) {
                left.push(plot);
            }
            if !self.plots.contains(&(plot.0, plot.1 + 1)) {
                right.push(plot);
            }
        }
        [up, down, left, right]
    }
}

fn count_sides(
    mut positions: Vec<Pos>,
    low_bound: impl Fn(Pos) -> bool,
    lower: impl Fn(Pos) -> Pos,
    upper: impl Fn(Pos) -> Pos,
) -> usize {
    let mut sides = 0;
    while let Some(position) = positions.pop() {
        sides += 1;
        let mut s = upper(position);
        while let Some(i) = positions.iter().position(|&p| p == s) {
            positions.swap_remove(i);
            s = upper(s);
        }
        if low_bound(position) {
            continue;
        }
        s = lower(position);
        while let Some(i) = positions.iter().position(|&p| p == s) {
            positions.swap_remove(i);
            if low_bound(s) {
                break;
            }
            s = lower(s);
        }
    }
    sides
}

//noinspection SpellCheckingInspection
#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = "\
AAAA
BBCD
BBCC
EEEC
";
    const EXAMPLE_2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    const EXAMPLE_3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn finds_total_price_of_fencing() {
        assert_eq!(Advent2024Day12Solver::new(EXAMPLE_1).solve_part1(), 140);
        assert_eq!(Advent2024Day12Solver::new(EXAMPLE_2).solve_part1(), 772);
        assert_eq!(Advent2024Day12Solver::new(EXAMPLE_3).solve_part1(), 1930);
    }

    #[test]
    fn finds_bulk_price_of_fencing() {
        assert_eq!(Advent2024Day12Solver::new(EXAMPLE_1).solve_part2(), 80);
        assert_eq!(Advent2024Day12Solver::new(EXAMPLE_2).solve_part2(), 436);
        assert_eq!(Advent2024Day12Solver::new(EXAMPLE_3).solve_part2(), 1206);
    }
}
