use crate::solver::AdventSolver;
use State::{Edge, Lumberyard, Open, Trees};

pub struct Advent2018Day18Solver {
    map: Map,
}

impl Advent2018Day18Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: Map::new(
                input
                    .lines()
                    .map(|l| l.chars().map(State::from).collect())
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2018Day18Solver {
    fn solve_part1(&self) -> usize {
        let mut map = self.map.clone();
        for _ in 0..10 {
            map = map.step();
        }
        map.resource_value()
    }

    fn solve_part2(&self) -> usize {
        const NUM_STEPS: usize = 1_000_000_000;
        fn find_map(start: &Map) -> Map {
            let mut maps = vec![start.clone()];
            for current in 1..NUM_STEPS {
                let next_map = maps.last().unwrap().step();
                if let Some(found) = maps.iter().position(|m| m == &next_map) {
                    let distance = current - found;
                    let remaining = (NUM_STEPS - found) % distance;
                    return maps[found + remaining].clone();
                }
                maps.push(next_map);
            }
            maps.last().unwrap().clone()
        }
        find_map(&self.map).resource_value()
    }
}

#[derive(Clone, Eq, PartialEq)]
enum State {
    Edge,
    Open,
    Trees,
    Lumberyard,
}

impl State {
    fn compute_next(&self, adjacents: &[&State]) -> Self {
        match self {
            Edge => Edge,
            Open => {
                if adjacents.iter().filter(|&a| matches!(a, Trees)).count() >= 3 {
                    Trees
                } else {
                    Open
                }
            }
            Trees => {
                if adjacents
                    .iter()
                    .filter(|&a| matches!(a, Lumberyard))
                    .count()
                    >= 3
                {
                    Lumberyard
                } else {
                    Trees
                }
            }
            Lumberyard => {
                if adjacents.iter().any(|&a| matches!(a, Lumberyard))
                    && adjacents.iter().any(|&a| matches!(a, Trees))
                {
                    Lumberyard
                } else {
                    Open
                }
            }
        }
    }

    fn is_trees(&self) -> bool {
        matches!(self, Trees)
    }
    fn is_lumberyard(&self) -> bool {
        matches!(self, Lumberyard)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Map {
    map: Vec<Vec<State>>,
}

impl Map {
    fn new(map: Vec<Vec<State>>) -> Self {
        let mut padded_map = vec![];
        padded_map.push(vec![Edge; map[0].len() + 2]);
        padded_map.extend(map.iter().map(|row| {
            let mut row = row.clone();
            row.insert(0, Edge);
            row.push(Edge);
            row
        }));
        padded_map.push(vec![Edge; map[0].len() + 2]);
        Self { map: padded_map }
    }

    fn resource_value(&self) -> usize {
        self.count(State::is_trees) * self.count(State::is_lumberyard)
    }

    fn count(&self, pred: fn(&State) -> bool) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|s| pred(s)).count())
            .sum()
    }

    fn step(&self) -> Self {
        let mut next_map = vec![];
        for y in 0..self.map.len() {
            let mut next_row = vec![];
            for x in 0..self.map[y].len() {
                if matches!(self.map[y][x], Edge) {
                    next_row.push(Edge);
                } else {
                    let adjacents = [
                        &self.map[y - 1][x - 1],
                        &self.map[y - 1][x],
                        &self.map[y - 1][x + 1],
                        &self.map[y][x - 1],
                        &self.map[y][x + 1],
                        &self.map[y + 1][x - 1],
                        &self.map[y + 1][x],
                        &self.map[y + 1][x + 1],
                    ];
                    next_row.push(self.map[y][x].compute_next(&adjacents));
                }
            }
            next_map.push(next_row);
        }
        Self { map: next_map }
    }
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Open,
            '|' => Trees,
            '#' => Lumberyard,
            _ => unreachable!("unknown state value: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
";

    #[test]
    fn calculates_resource_value() {
        let solver = Advent2018Day18Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 1147);
    }
}
