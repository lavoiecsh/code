use crate::solver::AdventSolver;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use MapTile::*;
use Movement::*;

pub struct Advent2024Day15Solver {
    warehouse: Warehouse,
    movements: Vec<Movement>,
}

impl Advent2024Day15Solver {
    pub fn new(input: &str) -> Self {
        let map = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|l| l.chars().map(MapTile::from).collect())
            .collect();
        let movements = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .flat_map(|l| l.chars().map(Movement::from))
            .collect();
        Self {
            warehouse: Warehouse::new(map),
            movements,
        }
    }
}

impl AdventSolver for Advent2024Day15Solver {
    fn solve_part1(&self) -> usize {
        let mut warehouse = self.warehouse.clone();
        self.movements.iter().for_each(|&m| warehouse.execute(m));
        warehouse.boxes().iter().map(|(y, x)| y * 100 + x).sum()
    }

    fn solve_part2(&self) -> usize {
        let mut warehouse = self.warehouse.widen();
        self.movements.iter().for_each(|&m| warehouse.execute(m));
        warehouse.boxes().iter().map(|(y, x)| y * 100 + x).sum()
    }
}

type Pos = (usize, usize);

#[derive(Clone)]
struct Warehouse {
    map: Vec<Vec<MapTile>>,
    robot: Pos,
}

impl Warehouse {
    fn new(map: Vec<Vec<MapTile>>) -> Self {
        let (y, row) = map.iter().find_position(|r| r.contains(&Robot)).unwrap();
        let x = row.iter().find_position(|&&r| r == Robot).unwrap().0;
        Self { map, robot: (y, x) }
    }

    fn widen(&self) -> Self {
        Self::new(
            self.map
                .iter()
                .map(|r| r.iter().flat_map(MapTile::widen).collect())
                .collect(),
        )
    }

    fn execute(&mut self, movement: Movement) {
        let next = movement.execute(self.robot);
        match (self.tile(next), movement) {
            (Wall, _) => {}
            (Empty, _) => {
                self.set_empty(self.robot);
                self.set_robot(next);
            }
            (Box, _) => {
                let mut after = movement.execute(next);
                while self.tile(after) == Box {
                    after = movement.execute(after);
                }
                match self.tile(after) {
                    Wall => {}
                    Empty => {
                        self.set_box(after);
                        self.set_empty(self.robot);
                        self.set_robot(next);
                    }
                    BoxLeft => unreachable!("cannot mix thin and wide box"),
                    BoxRight => unreachable!("cannot mix thin and wide box"),
                    Box => unreachable!("should not find a box here"),
                    Robot => unreachable!("should not find a robot here"),
                }
            }
            (BoxLeft, Left) => unreachable!("wide box has been split"),
            (BoxLeft, Right) => {
                let mut after = movement.execute(next);
                while [BoxRight, BoxLeft].contains(&self.tile(after)) {
                    after = movement.execute(after);
                }
                match self.tile(after) {
                    Wall => {}
                    Empty => {
                        self.set_box_right(after);
                        after = movement.rev().execute(after);
                        while after != next {
                            self.swap_box(after);
                            after = movement.rev().execute(after);
                        }
                        self.set_empty(self.robot);
                        self.set_robot(next);
                    }
                    Box => unreachable!("cannot mix thin and wide box"),
                    Robot => unreachable!("should not find a robot here"),
                    BoxLeft => unreachable!("should not find a box here"),
                    BoxRight => unreachable!("should not find a box here"),
                }
            }
            (BoxLeft, m) => {
                if let Some(moves) = self.wide_box_movements(&[next], m) {
                    moves
                        .iter()
                        .filter(|&&(_, t)| t == Empty)
                        .for_each(|&(p, t)| self.set(p, t));
                    moves
                        .iter()
                        .filter(|&&(_, t)| t != Empty)
                        .for_each(|&(p, t)| self.set(p, t));
                    self.set_empty(self.robot);
                    self.set_robot(next);
                }
            }
            (BoxRight, Left) => {
                let mut after = movement.execute(next);
                while [BoxLeft, BoxRight].contains(&self.tile(after)) {
                    after = movement.execute(after);
                }
                match self.tile(after) {
                    Wall => {}
                    Empty => {
                        self.set_box_left(after);
                        after = movement.rev().execute(after);
                        while after != next {
                            self.swap_box(after);
                            after = movement.rev().execute(after);
                        }
                        self.set_empty(self.robot);
                        self.set_robot(next);
                    }
                    Box => unreachable!("cannot mix thin and wide box"),
                    Robot => unreachable!("should not find a robot here"),
                    BoxLeft => unreachable!("should not find a box here"),
                    BoxRight => unreachable!("should not find a box here"),
                }
            }
            (BoxRight, Right) => unreachable!("wide box has been split"),
            (BoxRight, m) => {
                if let Some(moves) = self.wide_box_movements(&[Left.execute(next)], m) {
                    moves
                        .iter()
                        .filter(|&&(_, t)| t == Empty)
                        .for_each(|&(p, t)| self.set(p, t));
                    moves
                        .iter()
                        .filter(|&&(_, t)| t != Empty)
                        .for_each(|&(p, t)| self.set(p, t));
                    self.set_empty(self.robot);
                    self.set_robot(next);
                }
            }
            (Robot, _) => unreachable!("robot cannot be at 2 places at the same time"),
        }
    }

    fn tile(&self, pos: Pos) -> MapTile {
        self.map[pos.0][pos.1]
    }

    fn set_empty(&mut self, pos: Pos) {
        self.map[pos.0][pos.1] = Empty;
    }

    fn set_robot(&mut self, pos: Pos) {
        self.map[pos.0][pos.1] = Robot;
        self.robot = pos;
    }

    fn set_box(&mut self, pos: Pos) {
        self.map[pos.0][pos.1] = Box;
    }

    fn set_box_left(&mut self, pos: Pos) {
        self.map[pos.0][pos.1] = BoxLeft;
    }

    fn set_box_right(&mut self, pos: Pos) {
        self.map[pos.0][pos.1] = BoxRight;
    }

    fn swap_box(&mut self, pos: Pos) {
        self.map[pos.0][pos.1] = if self.tile(pos) == BoxLeft {
            BoxRight
        } else {
            BoxLeft
        };
    }

    fn set(&mut self, pos: Pos, tile: MapTile) {
        self.map[pos.0][pos.1] = tile;
    }

    fn wide_box_movements(&self, boxes: &[Pos], movement: Movement) -> Option<Vec<(Pos, MapTile)>> {
        let mut movements = Vec::new();
        let mut next_boxes = Vec::new();
        for &l in boxes {
            let r = Right.execute(l);
            let nl = movement.execute(l);
            let nr = movement.execute(r);
            movements.push((l, Empty));
            movements.push((r, Empty));
            movements.push((nl, BoxLeft));
            movements.push((nr, BoxRight));
            #[allow(unused_variables)] match (self.tile(nl), self.tile(nr)) {
                (Wall, _) => {
                    return None;
                }
                (_, Wall) => {
                    return None;
                }
                (Empty, Empty) => {}
                (BoxLeft, BoxRight) => {
                    next_boxes.push(nl);
                }
                (BoxRight, Empty) => {
                    next_boxes.push(Left.execute(nl));
                }
                (Empty, BoxLeft) => {
                    next_boxes.push(nr);
                }
                (BoxRight, BoxLeft) => {
                    next_boxes.push(Left.execute(nl));
                    next_boxes.push(nr);
                }
                (nlt, nrt) => unreachable!("invalid next row {nl:#?} = {nlt:#?}, {nr:#?} = {nrt:#?}"),
            }
        }
        if next_boxes.is_empty() {
            return Some(movements);
        }
        if let Some(movements_rec) = self.wide_box_movements(&next_boxes, movement) {
            movements.extend(movements_rec);
            Some(movements)
        } else {
            None
        }
    }

    fn boxes(&self) -> Vec<Pos> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == Box || c == BoxLeft)
                    .map(move |(x, _)| (y, x))
            })
            .collect()
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut map = String::with_capacity(self.map.len() * (self.map[0].len() + 2));
        map.push('\n');
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                map.push(self.tile((y, x)).into());
            }
            map.push('\n');
        }
        f.write_str(&map)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum MapTile {
    Wall,
    Empty,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

impl MapTile {
    fn widen(&self) -> [MapTile; 2] {
        match self {
            Wall => [Wall, Wall],
            Empty => [Empty, Empty],
            Box => [BoxLeft, BoxRight],
            Robot => [Robot, Empty],
            BoxLeft => unreachable!("cannot widen a widened tile"),
            BoxRight => unreachable!("cannot widen a widened tile"),
        }
    }
}

impl From<MapTile> for char {
    fn from(value: MapTile) -> Self {
        match value {
            Wall => '#',
            Empty => '.',
            Box => 'O',
            Robot => '@',
            BoxLeft => '[',
            BoxRight => ']',
        }
    }
}

impl From<char> for MapTile {
    fn from(value: char) -> Self {
        match value {
            '#' => Wall,
            '.' => Empty,
            'O' => Box,
            '@' => Robot,
            _ => unreachable!("unknown map tile {value}"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Movement {
    fn rev(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn execute(&self, pos: Pos) -> Pos {
        match self {
            Up => (pos.0 - 1, pos.1),
            Down => (pos.0 + 1, pos.1),
            Left => (pos.0, pos.1 - 1),
            Right => (pos.0, pos.1 + 1),
        }
    }
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => unreachable!("unknown movement {value}"),
        }
    }
}

//noinspection SpellCheckingInspection
#[cfg(test)]
mod test {
    use super::*;

    static SMALL_EXAMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
    static LARGE_EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn finds_sum_of_gps_coordinates() {
        assert_eq!(
            Advent2024Day15Solver::new(SMALL_EXAMPLE).solve_part1(),
            2028
        );
        assert_eq!(
            Advent2024Day15Solver::new(LARGE_EXAMPLE).solve_part1(),
            10092
        );
    }

    #[test]
    fn finds_sum_of_gps_coordinates_in_widened_map() {
        assert_eq!(
            Advent2024Day15Solver::new(LARGE_EXAMPLE).solve_part2(),
            9021
        );
    }
}
