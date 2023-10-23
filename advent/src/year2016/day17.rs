use crate::solver::AdventSolver;

pub struct Advent2016Day17Solver {
    passcode: String,
}

impl Advent2016Day17Solver {
    pub fn new(input: String) -> Self {
        Self { passcode: input }
    }
}

impl AdventSolver for Advent2016Day17Solver {
    fn solve_part1_string(&self) -> String {
        let start = Path { current: (0, 0), path: String::new() };
        let mut paths: Vec<Path> = vec!(start);
        while !paths.iter().any(|p| p.is_at_vault()) {
            paths = paths.iter().flat_map(|p| p.next_rooms(&self.passcode)).collect();
        }
        paths.iter().find(|p| p.is_at_vault()).unwrap().path.clone()
    }

    fn solve_part2(&self) -> usize {
        let start = Path { current: (0, 0), path: String::new() };
        let mut paths: Vec<Path> = vec!(start);
        let mut longest = 0;
        while !paths.is_empty() {
            let tmp_paths: Vec<Path> = paths.iter()
                .flat_map(|p| p.next_rooms(&self.passcode))
                .collect();
            let tmp_paths_length = tmp_paths.len();
            paths = tmp_paths.into_iter().filter(|p| !p.is_at_vault()).collect();
            if tmp_paths_length != paths.len() && paths.len() != 0 {
                longest = paths[0].path.len();
            }
        }
        longest
    }
}

type Pos = (usize, usize);

struct Path {
    current: Pos,
    path: String,
}

impl Path {
    fn is_at_vault(&self) -> bool {
        self.current == (3, 3)
    }

    fn next_rooms(&self, passcode: &str) -> Vec<Path> {
        format!("{:x}", md5::compute(format!("{passcode}{}", self.path)))
            .chars()
            .take(4)
            .enumerate()
            .filter_map(|(i,c)| match c {
                'b' | 'c' | 'd' | 'e' | 'f' => self.next_room(i),
                _ => None,
            })
            .collect()
    }

    fn next_room(&self, direction: usize) -> Option<Self> {
        match (direction, self.current) {
            (0, (_, 0)) => None,
            (0, (x, y)) => Some(Self { current: (x, y - 1), path: format!("{}U", self.path) }),
            (1, (_, 3)) => None,
            (1, (x, y)) => Some(Self { current: (x, y + 1), path: format!("{}D", self.path) }),
            (2, (0, _)) => None,
            (2, (x, y)) => Some(Self { current: (x - 1, y), path: format!("{}L", self.path) }),
            (3, (3, _)) => None,
            (3, (x, y)) => Some(Self { current: (x + 1, y), path: format!("{}R", self.path) }),
            _ => None,
        }
    }
}
