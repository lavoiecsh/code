use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub struct Advent2019Day20Solver {
    maze: Maze,
}

impl Advent2019Day20Solver {
    pub fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| l.chars().collect_vec())
            .filter(|r| !r.is_empty())
            .collect_vec();
        let parser = MazeParser::new(map);
        Self {
            maze: Maze::new(parser),
        }
    }
}

impl AdventSolver for Advent2019Day20Solver {
    fn solve_part1(&self) -> usize {
        self.maze.shortest_distance_portals()
    }

    fn solve_part2(&self) -> usize {
        self.maze.shortest_distance_recursive()
    }
}

type Name = (char, char);
const START: Name = ('A', 'A');
const END: Name = ('Z', 'Z');
type Pos = (usize, usize);

#[derive(Debug)]
struct Maze {
    start: Pos,
    end: Pos,
    portals: HashMap<Pos, (Pos, bool)>,
    paths: HashSet<Path>,
    outer_paths: HashSet<Path>,
    inner_paths: HashSet<Path>,
}

#[derive(Debug, Clone)]
struct Path {
    from: Pos,
    to: Pos,
    distance: usize,
}

impl Maze {
    fn new(parser: MazeParser) -> Self {
        let mut paths = HashSet::new();
        paths.extend(parser.paths_from(parser.start));
        paths.extend(parser.paths_from(parser.end));
        for point in &parser.points {
            paths.extend(parser.paths_from(point.outer));
            paths.extend(parser.paths_from(point.inner));
        }

        let portals = parser
            .points
            .iter()
            .flat_map(|p| vec![(p.inner, (p.outer, true)), (p.outer, (p.inner, false))])
            .collect();

        let outer_paths = paths
            .iter()
            .filter(|p| parser.is_inner(p))
            .cloned()
            .collect();

        let inner_paths = paths
            .iter()
            .filter(|p| parser.is_outer(p))
            .cloned()
            .collect();

        Self {
            start: parser.start,
            end: parser.end,
            portals,
            paths,
            outer_paths,
            inner_paths,
        }
    }

    fn shortest_distance_portals(&self) -> usize {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        distances.insert(self.start, 0);
        let mut queue = VecDeque::new();
        queue.push_back(self.start);
        let mut best_distance = usize::MAX;
        while let Some(current) = queue.pop_front() {
            let current_distance = *distances.get(&current).unwrap();
            if current_distance >= best_distance {
                continue;
            }
            for path in self
                .paths
                .iter()
                .filter_map(|p| p.starting_from(current))
                .filter(|p| p.to != self.start)
            {
                let next_distance = current_distance + path.distance + 1;
                if path.to == self.end {
                    if next_distance < best_distance {
                        best_distance = next_distance;
                    }
                    continue;
                }
                let to = self.portals.get(&path.to).unwrap().0;
                if next_distance >= *distances.get(&to).unwrap_or(&usize::MAX) {
                    continue;
                }
                distances.insert(to, next_distance);
                queue.push_back(to);
            }
        }
        best_distance - 1
    }

    fn shortest_distance_recursive(&self) -> usize {
        let mut distances: HashMap<(Pos, usize), usize> = HashMap::new();
        distances.insert((self.start, 0), 0);
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));
        let mut best_distance = usize::MAX;
        while let Some((point, level)) = queue.pop_front() {
            let current_distance = *distances.get(&(point, level)).unwrap();
            if current_distance >= best_distance {
                continue;
            }
            if level == 0 {
                for path in self
                    .outer_paths
                    .iter()
                    .filter_map(|p| p.starting_from(point))
                    .filter(|p| p.to != self.start)
                {
                    let next_distance = current_distance + path.distance + 1;
                    if path.to == self.end {
                        if next_distance < best_distance {
                            best_distance = next_distance;
                        }
                        continue;
                    }
                    let (to, outer) = *self.portals.get(&path.to).unwrap();
                    assert!(outer);
                    let previous_distance = *distances.get(&(to, 1)).unwrap_or(&usize::MAX);
                    if next_distance >= previous_distance {
                        continue;
                    }
                    distances.insert((to, 1), next_distance);
                    queue.push_back((to, 1));
                }
            } else {
                for path in self
                    .inner_paths
                    .iter()
                    .filter_map(|p| p.starting_from(point))
                {
                    let next_distance = current_distance + path.distance + 1;
                    let (to, outer) = *self.portals.get(&path.to).unwrap();
                    let next_level = if outer { level + 1 } else { level - 1 };
                    let previous_distance =
                        *distances.get(&(to, next_level)).unwrap_or(&usize::MAX);
                    if next_distance >= previous_distance {
                        continue;
                    }
                    distances.insert((to, next_level), next_distance);
                    queue.push_back((to, next_level));
                }
            }
        }
        best_distance - 1
    }
}

impl Path {
    fn starting_from(&self, pos: Pos) -> Option<Self> {
        if pos == self.from {
            Some(self.clone())
        } else if pos == self.to {
            Some(Self {
                from: self.to,
                to: self.from,
                distance: self.distance,
            })
        } else {
            None
        }
    }
}

impl PartialEq<Self> for Path {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.from.0 * self.to.0);
        state.write_usize(self.from.1 * self.to.1);
        state.write_usize(self.distance);
    }
}

struct MazeParser {
    map: Vec<Vec<Tile>>,
    start: Pos,
    end: Pos,
    points: Vec<Point>,
}

#[derive(Debug)]
struct Point {
    inner: Pos,
    outer: Pos,
}

#[derive(Debug)]
enum Tile {
    Wall,
    Open,
    Inner,
    Portal,
}

impl Tile {
    fn is_open(&self) -> bool {
        matches!(self, Tile::Open | Tile::Portal)
    }
}

#[derive(Debug)]
enum Parse {
    Before,
    FirstDonut,
    Inside,
    SecondDonut,
    After,
}

impl MazeParser {
    fn new(input: Vec<Vec<char>>) -> Self {
        let mut point_positions = Vec::new();
        let mut point_names = HashSet::new();
        let mut map: Vec<Vec<Tile>> = Vec::new();
        // top row
        let y = 2;
        let mut row: Vec<Tile> = Vec::new();
        for x in 0..input[y].len() {
            match input[y][x] {
                ' ' => {}
                '#' => {
                    row.push(Tile::Wall);
                }
                '.' => {
                    let name = (input[y - 2][x], input[y - 1][x]);
                    let pos = (y, x);
                    point_positions.push((name, pos, true));
                    point_names.insert(name);
                    row.push(Tile::Portal);
                }
                _c => unreachable!("invalid top row character '{_c}'"),
            }
        }
        map.push(row);
        // middle
        for y in 3..input.len() - 3 {
            let mut row: Vec<Tile> = Vec::new();
            let mut status = Parse::Before;
            for x in 0..input[y].len() {
                status = match (input[y][x], &status) {
                    (' ', Parse::Before | Parse::After) => status,
                    (' ', Parse::FirstDonut | Parse::Inside) => {
                        row.push(Tile::Inner);
                        Parse::Inside
                    }
                    (' ', Parse::SecondDonut) => Parse::After,
                    ('#', Parse::Before | Parse::FirstDonut) => {
                        row.push(Tile::Wall);
                        Parse::FirstDonut
                    }
                    ('#', Parse::Inside | Parse::SecondDonut) => {
                        row.push(Tile::Wall);
                        Parse::SecondDonut
                    }
                    ('.', Parse::Before) => {
                        // left side portal
                        let name = (input[y][x - 2], input[y][x - 1]);
                        let pos = (y, x);
                        point_positions.push((name, pos, true));
                        point_names.insert(name);
                        row.push(Tile::Portal);
                        Parse::FirstDonut
                    }
                    ('.', Parse::Inside) => {
                        // start second donut portal
                        let name = (input[y][x - 2], input[y][x - 1]);
                        let pos = (y, x);
                        point_positions.push((name, pos, false));
                        point_names.insert(name);
                        row.push(Tile::Portal);
                        Parse::SecondDonut
                    }
                    ('.', Parse::FirstDonut) => {
                        if matches!(input[y][x + 1], 'A'..='Z') {
                            // end first donut portal
                            let name = (input[y][x + 1], input[y][x + 2]);
                            let pos = (y, x);
                            point_positions.push((name, pos, false));
                            point_names.insert(name);
                            row.push(Tile::Portal);
                        } else if matches!(input[y + 1][x], 'A'..='Z') {
                            // top of inner portal
                            let name = (input[y + 1][x], input[y + 2][x]);
                            let pos = (y, x);
                            point_positions.push((name, pos, false));
                            point_names.insert(name);
                            row.push(Tile::Portal);
                        } else if matches!(input[y - 1][x], 'A'..='Z') {
                            // bottom of inner portal
                            let name = (input[y - 2][x], input[y - 1][x]);
                            let pos = (y, x);
                            point_positions.push((name, pos, false));
                            point_names.insert(name);
                            row.push(Tile::Portal);
                        } else {
                            row.push(Tile::Open);
                        }
                        status
                    }
                    ('.', Parse::SecondDonut) => {
                        if matches!(input[y][x + 1], 'A'..='Z') {
                            // right side portal
                            let name = (input[y][x + 1], input[y][x + 2]);
                            let pos = (y, x);
                            point_positions.push((name, pos, true));
                            point_names.insert(name);
                            row.push(Tile::Portal);
                        } else {
                            row.push(Tile::Open);
                        }
                        status
                    }
                    ('A'..='Z', Parse::Before) => Parse::Before,
                    ('A'..='Z', Parse::FirstDonut) => {
                        row.push(Tile::Inner);
                        Parse::Inside
                    }
                    ('A'..='Z', Parse::Inside) => {
                        row.push(Tile::Inner);
                        Parse::Inside
                    }
                    ('A'..='Z', Parse::SecondDonut | Parse::After) => Parse::After,
                    (_c, _s) => {
                        unreachable!("invalid status {_s:?} and character '{_c}' combination");
                    }
                }
            }
            map.push(row);
        }
        // bottom row
        let y = input.len() - 3;
        let mut row: Vec<Tile> = Vec::new();
        for x in 0..input[y].len() {
            match input[y][x] {
                ' ' => {}
                '#' => {
                    row.push(Tile::Wall);
                }
                '.' => {
                    let name = (input[y + 1][x], input[y + 2][x]);
                    let pos = (y, x);
                    point_positions.push((name, pos, true));
                    point_names.insert(name);
                    row.push(Tile::Portal);
                }
                _c => unreachable!("invalid bottom row character '{_c}'"),
            }
        }
        map.push(row);
        // combine portals
        let points = point_names
            .into_iter()
            .filter(|&name| name != START && name != END)
            .map(|name| {
                let inner = point_positions
                    .iter()
                    .filter(|&(n, _, o)| n == &name && !o)
                    .map(|&(_, p, _)| (p.0 - 2, p.1 - 2))
                    .collect_vec();
                if inner.len() != 1 {
                    unreachable!(
                        "invalid number of inner points for {name:?}: {:?}",
                        inner.len()
                    );
                }
                let outer = point_positions
                    .iter()
                    .filter(|&(n, _, o)| n == &name && *o)
                    .map(|&(_, p, _)| (p.0 - 2, p.1 - 2))
                    .collect_vec();
                if outer.len() != 1 {
                    unreachable!(
                        "invalid number of outer points for {name:?}: {:?}",
                        outer.len()
                    );
                }
                Point {
                    inner: inner[0],
                    outer: outer[0],
                }
            })
            .collect();
        Self {
            map,
            start: point_positions
                .iter()
                .find(|&(n, _, _)| n == &START)
                .map(|&(_, p, _)| (p.0 - 2, p.1 - 2))
                .expect("no start found"),
            end: point_positions
                .iter()
                .find(|&(n, _, _)| n == &END)
                .map(|&(_, p, _)| (p.0 - 2, p.1 - 2))
                .expect("no end found"),
            points,
        }
    }

    fn is_inner(&self, path: &Path) -> bool {
        !self
            .points
            .iter()
            .any(|p| p.outer == path.from || p.outer == path.to)
    }

    fn is_outer(&self, path: &Path) -> bool {
        path.from != self.start
            && path.from != self.end
            && path.to != self.start
            && path.to != self.end
    }

    fn paths_from(&self, from: Pos) -> Vec<Path> {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        distances.insert(from, 0);
        let mut queue = VecDeque::new();
        queue.extend(self.around(from).map(|p| (p, 1)));
        let mut paths = Vec::new();
        while let Some((to, distance)) = queue.pop_front() {
            distances.insert(to, distance);
            match self.map[to.0][to.1] {
                Tile::Open => {
                    queue.extend(
                        self.around(to)
                            .filter(|p| !distances.contains_key(p))
                            .map(|p| (p, distance + 1)),
                    );
                }
                Tile::Portal => {
                    paths.push(Path { from, to, distance });
                }
                Tile::Inner | Tile::Wall => {
                    unreachable!("invalid tile {:?} at position {to:?}", self.map[to.0][to.1]);
                }
            }
        }
        paths
    }

    fn around(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let mut around = Vec::new();
        if pos.1 > 0 && self.map[pos.0][pos.1 - 1].is_open() {
            around.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.map[pos.0].len() - 1 && self.map[pos.0][pos.1 + 1].is_open() {
            around.push((pos.0, pos.1 + 1));
        }
        if pos.0 > 0 && self.map[pos.0 - 1][pos.1].is_open() {
            around.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.map.len() - 1 && self.map[pos.0 + 1][pos.1].is_open() {
            around.push((pos.0 + 1, pos.1));
        }
        around.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
";
    const EXAMPLE_2: &str = "
                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
";
    const EXAMPLE_3: &str = "
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M
";

    #[test]
    fn finds_shortest_path() {
        assert_eq!(23, Advent2019Day20Solver::new(EXAMPLE_1).solve_part1());
        assert_eq!(58, Advent2019Day20Solver::new(EXAMPLE_2).solve_part1());
    }

    #[test]
    fn finds_shortest_recursive_path() {
        assert_eq!(26, Advent2019Day20Solver::new(EXAMPLE_1).solve_part2());
        assert_eq!(396, Advent2019Day20Solver::new(EXAMPLE_3).solve_part2());
    }
}
