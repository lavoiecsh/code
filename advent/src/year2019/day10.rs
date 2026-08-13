use crate::solver::AdventSolver;
use itertools::Itertools;
use num_integer::Integer;
use std::collections::{HashSet, VecDeque};
use std::f64::consts::PI;

pub struct Advent2019Day10Solver {
    map: Map,
}

impl Advent2019Day10Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: Map::new(
                input
                    .lines()
                    .map(|l| l.chars().map(|c| c == '#').collect())
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2019Day10Solver {
    fn solve_part1(&self) -> usize {
        self.map.best_station_position().1
    }

    fn solve_part2(&self) -> usize {
        let best_station = self.map.best_station_position().0;
        let vaporization_order = self.map.vaporization_order(&best_station);
        let solution_asteroid = &vaporization_order[200];
        solution_asteroid.pos.0 * 100 + solution_asteroid.pos.1
    }
}

type Pos = (usize, usize);
type Angle = (isize, isize);

struct Map {
    asteroids: Vec<Pos>,
}

struct PolarAsteroid {
    pos: Pos,
    angle: f64,
    distance: f64,
}

impl PolarAsteroid {
    fn new(pos: Pos, angle: f64, distance: f64) -> Self {
        Self {
            pos,
            angle,
            distance,
        }
    }

    fn from_station_to_asteroid(station: &Pos, asteroid: &Pos) -> Self {
        let x = asteroid.0 as f64 - station.0 as f64;
        let y = asteroid.1 as f64 - station.1 as f64;
        let mut angle = (y / x).atan() + PI / 2f64;
        if x < 0f64 {
            angle += PI;
        }
        let distance = (x * x + y * y).sqrt();
        Self {
            pos: *asteroid,
            angle,
            distance,
        }
    }
}

impl Map {
    fn new(map: Vec<Vec<bool>>) -> Self {
        let asteroids = map
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, col)| **col)
                    .map(move |(col_index, _)| (col_index, row_index))
            })
            .collect();
        Self {
            asteroids,
        }
    }

    fn best_station_position(&self) -> (Pos, usize) {
        let mut best = ((0, 0), 0);
        for asteroid in &self.asteroids {
            let count = self.asteroids_visible_from(asteroid);
            if count > best.1 {
                best = (*asteroid, count);
            }
        }
        best
    }

    fn asteroids_visible_from(&self, pos: &Pos) -> usize {
        self.asteroids
            .iter()
            .map(|a| angle_between(pos, a))
            .collect::<HashSet<Angle>>()
            .len()
            - 1
    }

    fn vaporization_order(&self, station: &Pos) -> Vec<PolarAsteroid> {
        let mut polarized: VecDeque<PolarAsteroid> = self
            .asteroids
            .iter()
            .filter(|&a| a != station)
            .map(|a| PolarAsteroid::from_station_to_asteroid(station, a))
            .sorted_by(|a, b| a.angle.total_cmp(&b.angle).then(a.distance.total_cmp(&b.distance)))
            .collect();
        let mut vaporization_order = vec![PolarAsteroid::new(*station, 0f64, 0f64)];
        let mut angle = 0f64;
        let mut pushed_back = polarized.len();
        while let Some(asteroid) = polarized.pop_front() {
            if asteroid.angle == angle && pushed_back < polarized.len() + 1 {
                polarized.push_back(asteroid);
                pushed_back += 1;
                continue;
            }
            angle = asteroid.angle;
            vaporization_order.push(asteroid);
            pushed_back = 0;
        }
        vaporization_order
    }
}

fn angle_between(station: &Pos, asteroid: &Pos) -> Angle {
    let r = station.0 as isize - asteroid.0 as isize;
    let c = station.1 as isize - asteroid.1 as isize;
    match (r, c) {
        (0, 0) => (0, 0),
        (0, _) => (0, if c > 0 { 1 } else { -1 }),
        (_, 0) => (if r > 0 { 1 } else { -1 }, 0),
        _ => {
            let gcd = r.gcd(&c);
            (r / gcd, c / gcd)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
.#..#
.....
#####
....#
...##
";
    const EXAMPLE_2: &str = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
    const EXAMPLE_3: &str = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
    const EXAMPLE_4: &str = "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
    const EXAMPLE_5: &str = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
    const EXAMPLE_6: &str = "\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
";

    #[test]
    fn finds_best_asteroid_detection_station() {
        let solver = Advent2019Day10Solver::new(EXAMPLE_1);
        let solution = solver.map.best_station_position();
        assert_eq!(solution, ((3, 4), 8));

        let solver = Advent2019Day10Solver::new(EXAMPLE_2);
        let solution = solver.map.best_station_position();
        assert_eq!(solution, ((5, 8), 33));

        let solver = Advent2019Day10Solver::new(EXAMPLE_3);
        let solution = solver.map.best_station_position();
        assert_eq!(solution, ((1, 2), 35));

        let solver = Advent2019Day10Solver::new(EXAMPLE_4);
        let solution = solver.map.best_station_position();
        assert_eq!(solution, ((6, 3), 41));

        let solver = Advent2019Day10Solver::new(EXAMPLE_5);
        let solution = solver.map.best_station_position();
        assert_eq!(solution, ((11, 13), 210));
    }

    #[test]
    fn orders_asteroids_by_vaporization() {
        let solver = Advent2019Day10Solver::new(EXAMPLE_6);
        let solution = solver.map.best_station_position();
        assert_eq!(solution.0, (8, 3));
        let vaporized = solver.map.vaporization_order(&solution.0);
        assert_eq!(vaporized[0].pos, (8, 3));
        assert_eq!(vaporized[1].pos, (8, 1));
        assert_eq!(vaporized[2].pos, (9, 0));
        assert_eq!(vaporized[3].pos, (9, 1));
        assert_eq!(vaporized[4].pos, (10, 0));
        assert_eq!(vaporized[5].pos, (9, 2));
        assert_eq!(vaporized[6].pos, (11, 1));
        assert_eq!(vaporized[7].pos, (12, 1));
        assert_eq!(vaporized[8].pos, (11, 2));
        assert_eq!(vaporized[9].pos, (15, 1));
        assert_eq!(vaporized[10].pos, (12, 2));
        assert_eq!(vaporized[11].pos, (13, 2));
        assert_eq!(vaporized[12].pos, (14, 2));
        assert_eq!(vaporized[13].pos, (15, 2));
        assert_eq!(vaporized[14].pos, (12, 3));
        assert_eq!(vaporized[15].pos, (16, 4));
        assert_eq!(vaporized[16].pos, (15, 4));
        assert_eq!(vaporized[17].pos, (10, 4));
        assert_eq!(vaporized[18].pos, (4, 4));
        assert_eq!(vaporized[19].pos, (2, 4));
        assert_eq!(vaporized[20].pos, (2, 3));
        assert_eq!(vaporized[21].pos, (0, 2));
        assert_eq!(vaporized[22].pos, (1, 2));
        assert_eq!(vaporized[23].pos, (0, 1));
        assert_eq!(vaporized[24].pos, (1, 1));
        assert_eq!(vaporized[25].pos, (5, 2));
        assert_eq!(vaporized[26].pos, (1, 0));
        assert_eq!(vaporized[27].pos, (5, 1));
        assert_eq!(vaporized[28].pos, (6, 1));
        assert_eq!(vaporized[29].pos, (6, 0));
        assert_eq!(vaporized[30].pos, (7, 0));
        assert_eq!(vaporized[31].pos, (8, 0));
        assert_eq!(vaporized[32].pos, (10, 1));
        assert_eq!(vaporized[33].pos, (14, 0));
        assert_eq!(vaporized[34].pos, (16, 1));
        assert_eq!(vaporized[35].pos, (13, 3));
        assert_eq!(vaporized[36].pos, (14, 3));
        assert_eq!(vaporized.len(), 37);

        let solver = Advent2019Day10Solver::new(EXAMPLE_5);
        let solution = solver.map.best_station_position();
        let vaporized = solver.map.vaporization_order(&solution.0);
        assert_eq!(vaporized[0].pos, (11, 13));
        assert_eq!(vaporized[1].pos, (11, 12));
        assert_eq!(vaporized[2].pos, (12, 1));
        assert_eq!(vaporized[3].pos, (12, 2));
        assert_eq!(vaporized[10].pos, (12, 8));
        assert_eq!(vaporized[20].pos, (16, 0));
        assert_eq!(vaporized[50].pos, (16, 9));
        assert_eq!(vaporized[100].pos, (10, 16));
        assert_eq!(vaporized[199].pos, (9, 6));
        assert_eq!(vaporized[200].pos, (8, 2));
        assert_eq!(vaporized[201].pos, (10, 9));
        assert_eq!(vaporized[297].pos, (11, 3));
        assert_eq!(vaporized[298].pos, (11, 2));
        assert_eq!(vaporized[299].pos, (11, 1));
        assert_eq!(vaporized.len(), 300);
    }
}
