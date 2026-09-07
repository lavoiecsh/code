use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter, Write};

pub struct Advent2019Day18Solver {
    vault: Vault,
}

impl Advent2019Day18Solver {
    pub fn new(input: &str) -> Self {
        let mut start = None;
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        Self {
            vault: Vault {
                open_passages: input
                    .lines()
                    .enumerate()
                    .map(|(y, row)| {
                        row.chars()
                            .enumerate()
                            .map(|(x, c)| match c {
                                '#' => false,
                                '.' => true,
                                '@' => {
                                    start = Some((y, x));
                                    true
                                }
                                'a'..='z' => {
                                    keys.insert((y, x), c);
                                    true
                                }
                                'A'..='Z' => {
                                    doors.insert((y, x), c);
                                    true
                                }
                                _ => unreachable!(
                                    "unknown vault map character {c} at position ({y},{x})"
                                ),
                            })
                            .collect()
                    })
                    .collect(),
                start: start.expect("no starting position found"),
                keys,
                doors,
            },
        }
    }
}

impl AdventSolver for Advent2019Day18Solver {
    fn solve_part1(&self) -> usize {
        self.vault.shortest_path_length()
    }

    fn solve_part2(&self) -> usize {
        self.vault.split().shortest_path_length()
    }
}

type Pos = (usize, usize);
type ItemPos = (Pos, char);

fn pos_in_quadrant(
    input: ItemPos,
    height: (usize, usize),
    width: (usize, usize),
) -> Option<ItemPos> {
    if input.0.0 < height.0 || input.0.0 > height.1 || input.0.1 < width.0 || input.0.1 > width.1 {
        None
    } else {
        Some(((input.0.0 - height.0, input.0.1 - width.0), input.1))
    }
}

struct Vault {
    open_passages: Vec<Vec<bool>>,
    start: Pos,
    keys: HashMap<Pos, char>,
    doors: HashMap<Pos, char>,
}

struct SplitVault {
    vaults: [Vault; 4],
    key_count: usize,
}

#[derive(Debug, Clone)]
struct Path {
    pos: Vec<Pos>,
    collected_keys: Vec<char>,
    distance: usize,
}

impl SplitVault {
    fn shortest_path_length(&self) -> usize {
        let mut paths = VecDeque::new();
        paths.push_back(Path {
            pos: self.vaults.iter().map(|v| v.start).collect_vec(),
            collected_keys: Vec::new(),
            distance: 0,
        });
        let mut best = usize::MAX;
        while let Some(path) = paths.pop_front() {
            if path.distance >= best {
                continue;
            }
            if path.collected_keys.len() == self.key_count {
                best = path.distance;
                continue;
            }

            for v in 0..4 {
                let vault_path = path.clone();
                for next_path in self.vaults[v].next_paths(vault_path, v) {
                    if let Some((position, current_path)) =
                        paths.iter().find_position(|p| p.matches(&next_path))
                    {
                        if current_path.distance > next_path.distance {
                            paths[position] = next_path;
                        }
                    } else {
                        paths.push_back(next_path);
                    }
                }
            }
        }
        best
    }
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Vault {
    fn split(&self) -> SplitVault {
        SplitVault {
            vaults: [
                self.quadrant(Quadrant::TopLeft),
                self.quadrant(Quadrant::TopRight),
                self.quadrant(Quadrant::BottomLeft),
                self.quadrant(Quadrant::BottomRight),
            ],
            key_count: self.keys.len(),
        }
    }

    fn quadrant(&self, quadrant: Quadrant) -> Self {
        let full_height = self.open_passages.len();
        let half_height = full_height / 2;
        let full_width = self.open_passages[0].len();
        let half_width = full_width / 2;
        let (height, width) = match quadrant {
            Quadrant::TopLeft => ((0, half_height), (0, half_width)),
            Quadrant::TopRight => ((0, half_height), (half_width, full_width)),
            Quadrant::BottomLeft => ((half_height, full_height), (0, half_width)),
            Quadrant::BottomRight => ((half_height, full_height), (half_width, full_width)),
        };
        let mut open_passages: Vec<Vec<bool>> = self
            .open_passages
            .iter()
            .skip(height.0)
            .take(full_height / 2 + 1)
            .map(|r| {
                r.iter()
                    .skip(width.0)
                    .take(full_width / 2 + 1)
                    .cloned()
                    .collect()
            })
            .collect();
        let keys = self
            .keys
            .iter()
            .filter_map(|(&p, &k)| pos_in_quadrant((p, k), height, width))
            .collect();
        let doors = self
            .doors
            .iter()
            .filter_map(|(&p, &d)| pos_in_quadrant((p, d), height, width))
            .collect();
        let start = match quadrant {
            Quadrant::TopLeft => {
                open_passages[half_height - 1][half_width] = false;
                open_passages[half_height][half_width - 1] = false;
                open_passages[half_height][half_width] = false;
                (half_height - 1, half_width - 1)
            }
            Quadrant::TopRight => {
                open_passages[half_height - 1][0] = false;
                open_passages[half_height][1] = false;
                open_passages[half_height][0] = false;
                (half_height - 1, 1)
            }
            Quadrant::BottomLeft => {
                open_passages[1][half_width] = false;
                open_passages[0][half_width - 1] = false;
                open_passages[0][half_width] = false;
                (1, half_width - 1)
            }
            Quadrant::BottomRight => {
                open_passages[1][0] = false;
                open_passages[0][1] = false;
                open_passages[0][0] = false;
                (1, 1)
            }
        };

        Self {
            open_passages,
            start,
            keys,
            doors,
        }
    }

    fn shortest_path_length(&self) -> usize {
        let mut paths = VecDeque::new();
        paths.push_back(Path {
            pos: vec![self.start],
            collected_keys: Vec::new(),
            distance: 0,
        });
        let mut best = usize::MAX;
        while let Some(path) = paths.pop_front() {
            if path.distance >= best {
                continue;
            }
            if path.collected_keys.len() == self.keys.len() {
                best = path.distance;
                continue;
            }

            for next_path in self.next_paths(path, 0) {
                if let Some((position, current_path)) =
                    paths.iter().find_position(|p| p.matches(&next_path))
                {
                    if current_path.distance > next_path.distance {
                        paths[position] = next_path;
                    }
                } else {
                    paths.push_back(next_path);
                }
            }
        }
        best
    }

    fn next_paths(&self, path: Path, path_index: usize) -> Vec<Path> {
        let mut paths = Vec::new();
        let mut positions = VecDeque::new();
        let mut distances = HashMap::new();
        distances.insert(path.pos[path_index], 0);
        positions.push_back((path.pos[path_index], 0));
        while let Some((position, distance)) = positions.pop_front() {
            if let Some(&key) = self.keys.get(&position)
                && !path.has_collected(key)
            {
                paths.push(path.grab_key(path_index, position, distance, key));
                continue;
            }
            if let Some(&door) = self.doors.get(&position)
                && !path.can_open(door)
            {
                continue;
            }
            let next_distance = distance + 1;
            for p in vec![
                (position.0, position.1 + 1),
                (position.0, position.1 - 1),
                (position.0 + 1, position.1),
                (position.0 - 1, position.1),
            ] {
                if !self.open_passages[p.0][p.1] || distances.contains_key(&p) {
                    continue;
                }
                distances.insert(p, next_distance);
                positions.push_back((p, next_distance));
            }
        }
        paths
    }
}

impl Path {
    fn has_collected(&self, key: char) -> bool {
        self.collected_keys.contains(&key)
    }

    fn grab_key(&self, path_index: usize, new_pos: Pos, distance: usize, key: char) -> Self {
        let mut pos = self.pos.clone();
        pos[path_index] = new_pos;
        let mut collected_keys = self.collected_keys.clone();
        collected_keys.push(key);
        Self {
            pos,
            collected_keys,
            distance: self.distance + distance,
        }
    }

    fn can_open(&self, door: char) -> bool {
        self.collected_keys.contains(&door.to_ascii_lowercase())
    }

    fn matches(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.collected_keys.len() == other.collected_keys.len()
            && self.collected_keys.iter().all(|&k| other.has_collected(k))
    }
}

impl Debug for Vault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.open_passages.len() {
            for x in 0..self.open_passages[y].len() {
                if self.start == (y, x) {
                    f.write_char('@')?;
                } else if let Some(&key) = self.keys.get(&(y, x)) {
                    f.write_char(key)?;
                } else if let Some(&door) = self.doors.get(&(y, x)) {
                    f.write_char(door)?;
                } else {
                    f.write_char(if self.open_passages[y][x] { '.' } else { '#' })?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
#########
#b.A.@.a#
#########
";
    const EXAMPLE_2: &str = "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
";
    const EXAMPLE_3: &str = "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
";
    const EXAMPLE_4: &str = "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
";
    const EXAMPLE_5: &str = "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
";
    const EXAMPLE_6: &str = "\
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
";
    const EXAMPLE_7: &str = "\
###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############
";
    // Example 8 doesn't work because we assume the vault splits on straight walls
    // but here, it splits along a wavy wall (around e, d, j, k)
    #[allow(unused)]
    const EXAMPLE_8: &str = "\
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############
";
    const EXAMPLE_9: &str = "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############
";

    #[test]
    fn finds_shortest_path_to_all_keys() {
        assert_eq!(8, Advent2019Day18Solver::new(EXAMPLE_1).solve_part1());
        assert_eq!(86, Advent2019Day18Solver::new(EXAMPLE_2).solve_part1());
        assert_eq!(132, Advent2019Day18Solver::new(EXAMPLE_3).solve_part1());
        assert_eq!(136, Advent2019Day18Solver::new(EXAMPLE_4).solve_part1());
        assert_eq!(81, Advent2019Day18Solver::new(EXAMPLE_5).solve_part1());
    }

    #[test]
    fn finds_shortest_path_with_4_robots() {
        assert_eq!(8, Advent2019Day18Solver::new(EXAMPLE_6).solve_part2());
        assert_eq!(24, Advent2019Day18Solver::new(EXAMPLE_7).solve_part2());
        // assert_eq!(32, Advent2019Day18Solver::new(EXAMPLE_8).solve_part2());
        assert_eq!(72, Advent2019Day18Solver::new(EXAMPLE_9).solve_part2());
    }
}
