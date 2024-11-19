use crate::solver::AdventSolver;

#[derive(Debug, Clone)]
struct Pos(usize, usize);

impl Pos {
    fn new(pos: &str) -> Self {
        let mut s = pos.split(",");
        Self(
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        )
    }

    fn down(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn down_left(&self) -> Self {
        Self(self.0 - 1, self.1 + 1)
    }

    fn down_right(&self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }
}

pub struct Advent2022Day14Solver {
    paths: Vec<Vec<Pos>>,
}

impl Advent2022Day14Solver {
    pub fn new(input: &str) -> Self {
        Self {
            paths: input
                .lines()
                .map(|p| p.split(" -> ").map(Pos::new).collect())
                .collect(),
        }
    }

    fn to_map(&self) -> SandMap {
        let max_col = self
            .paths
            .iter()
            .map(|p| p.iter().map(|q| q.0).max().unwrap())
            .max()
            .unwrap()
            + 1;
        let max_row = self
            .paths
            .iter()
            .map(|p| p.iter().map(|q| q.1).max().unwrap())
            .max()
            .unwrap()
            + 1;
        let mut sand_map = vec![];
        for _ in 0..max_row {
            sand_map.push(vec!['.'; max_col]);
        }
        for path in &self.paths {
            for i in 1..path.len() {
                if path[i].0 == path[i - 1].0 {
                    let c = path[i].0;
                    for r in
                        usize::min(path[i].1, path[i - 1].1)..=usize::max(path[i].1, path[i - 1].1)
                    {
                        sand_map[r][c] = '#';
                    }
                } else {
                    let r = path[i].1;
                    for c in
                        usize::min(path[i].0, path[i - 1].0)..=usize::max(path[i].0, path[i - 1].0)
                    {
                        sand_map[r][c] = '#';
                    }
                }
            }
        }
        SandMap { sand_map }
    }
}

struct SandMap {
    sand_map: Vec<Vec<char>>,
}

impl SandMap {
    fn _pp(&self) {
        let min_col = self
            .sand_map
            .iter()
            .map(|r| r.iter().position(|c| *c != '.').unwrap_or(usize::MAX))
            .min()
            .unwrap();
        for row in &self.sand_map {
            println!("{}", row.iter().skip(min_col).collect::<String>());
        }
    }

    fn add_floor(&mut self) {
        let increase = 200;
        let max_col = self.sand_map[0].len() + increase;
        for r in 0..self.sand_map.len() {
            for _ in 0..increase {
                self.sand_map[r].push('.');
            }
        }
        self.sand_map.push((0..max_col).map(|_| '.').collect());
        self.sand_map.push((0..max_col).map(|_| '#').collect());
    }

    fn is_open(&self, pos: Pos) -> bool {
        self.sand_map[pos.1][pos.0] == '.'
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_pos = Pos(500, 0);
        if !self.is_open(sand_pos.clone()) {
            return false;
        }
        while sand_pos.1 < self.sand_map.len() - 1 {
            if self.is_open(sand_pos.down()) {
                sand_pos = sand_pos.down();
                continue;
            }
            if self.is_open(sand_pos.down_left()) {
                sand_pos = sand_pos.down_left();
                continue;
            }
            if self.is_open(sand_pos.down_right()) {
                sand_pos = sand_pos.down_right();
                continue;
            }
            break;
        }
        if sand_pos.1 == self.sand_map.len() - 1 {
            return false;
        }
        self.sand_map[sand_pos.1][sand_pos.0] = 'o';
        true
    }
}

impl AdventSolver for Advent2022Day14Solver {
    fn solve_part1(&self) -> usize {
        let mut sand_map = self.to_map();
        let mut sand_count = 0;
        while sand_map.drop_sand() {
            sand_count += 1;
        }
        sand_count
    }

    fn solve_part2(&self) -> usize {
        let mut sand_map = self.to_map();
        sand_map.add_floor();
        let mut sand_count = 0;
        while sand_map.drop_sand() {
            sand_count += 1;
        }
        sand_count
    }
}
