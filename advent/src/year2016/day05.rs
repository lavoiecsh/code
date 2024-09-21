use crate::solver::AdventSolver;

pub struct Advent2016Day05Solver {
    door_id: String,
}

impl Advent2016Day05Solver {
    pub fn new(input: String) -> Self {
        Self { door_id: input }
    }
}

impl AdventSolver for Advent2016Day05Solver {
    fn solve_part1_string(&self) -> String {
        (0..)
            .map(|i| format!("{:x}", md5::compute(format!("{}{i}", self.door_id))))
            .filter(|d| d.starts_with("00000"))
            .map(|d| d.chars().nth(5).unwrap())
            .take(8)
            .collect()
    }

    fn solve_part2_string(&self) -> String {
        let mut password = [' '; 8];
        let mut i = 0;
        while password.contains(&' ') {
            let d = format!("{:x}", md5::compute(format!("{}{i}", self.door_id)));
            i += 1;
            if !d.starts_with("00000") {
                continue;
            }
            let mut c = d.chars().skip(5);
            let is = c.next().unwrap();
            if !"01234567".contains(is) {
                continue;
            }
            let index: usize = is.to_string().parse().unwrap();
            if password[index] != ' ' {
                continue;
            }

            password[index] = c.next().unwrap();
        }
        password.iter().collect()
    }
}
