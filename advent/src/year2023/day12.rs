use crate::solver::AdventSolver;

pub struct Advent2023Day12Solver {
    springs: Vec<Spring>,
}

impl Advent2023Day12Solver {
    pub fn new(input: String) -> Self {
        Self { springs: input.lines().map(Spring::from).collect() }
    }
}

impl AdventSolver for Advent2023Day12Solver {
    fn solve_part1(&self) -> usize {
        self.springs.iter().map(|s| s.arrangements()).sum()
    }

    fn solve_part2(&self) -> usize {
        self.springs.iter().map(|s| s.unfold(5).arrangements()).sum()
    }
}

struct Spring {
    conditions: Vec<char>,
    groups: Vec<usize>,
}

impl Spring {
    fn arrangements(&self) -> usize {
        let mut possibilities = vec!(false);
        for i in 0..self.groups.len() {
            possibilities.extend((0..self.groups[i]).map(|_| true));
            possibilities.push(false);
        }

        let mut row_start = 1;
        let mut row: Vec<usize> = (0..possibilities.len()).map(|_| 0).collect();
        row[0] = row_start;
        for c in 0..self.conditions.len() {
            let condition = self.conditions[c];
            if condition == '#' { row_start = 0; }
            let mut next = vec!(row_start);
            for i in 1..possibilities.len() {
                next.push(match (condition, possibilities[i]) {
                    ('.', false) => row[i - 1] + row[i],
                    ('.', true) => 0,
                    ('#', false) => 0,
                    ('#', true) => row[i - 1],
                    ('?', false) => row[i - 1] + row[i],
                    ('?', true) => row[i - 1],
                    _ => panic!(),
                });
            }
            row = next;
        }

        row.iter().rev().take(2).sum()
    }

    fn unfold(&self, count: usize) -> Self {
        let mut conditions = self.conditions.clone();
        let mut groups = self.groups.clone();
        for _ in 0..count - 1 {
            conditions.push('?');
            conditions.extend(self.conditions.iter());
            groups.extend(self.groups.iter());
        }

        Self { conditions, groups }
    }
}

impl From<&str> for Spring {
    fn from(value: &str) -> Self {
        let mut s = value.split(" ");
        Self {
            conditions: s.next().unwrap().chars().collect(),
            groups: s.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect(),
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day12Solver {
    Advent2023Day12Solver::new(String::from("\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"))
}

#[test]
fn possible_arrangements() {
    let solver = test_solver_1();
    let arrangements: Vec<usize> = solver.springs.iter().map(|s| s.arrangements()).collect();
    assert_eq!(arrangements, vec!(1, 4, 1, 1, 4, 10));
    assert_eq!(solver.solve_part1(), 21);
}

#[test]
fn unfolded_possible_arrangements() {
    let solver = test_solver_1();
    let arrangements: Vec<usize> = solver.springs.iter().map(|s| s.unfold(5).arrangements()).collect();
    assert_eq!(arrangements, vec!(1, 16384, 1, 16, 2500, 506250));
    assert_eq!(solver.solve_part2(), 525152);
}
