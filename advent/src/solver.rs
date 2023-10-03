pub trait AdventSolver {
    fn solve_part1(&self) -> usize { 0 }
    fn solve_part1_string(&self) -> String {
        self.solve_part1().to_string()
    }
    fn solve_part2(&self) -> usize { 0 }
    fn solve_part2_string(&self) -> String {
        self.solve_part2().to_string()
    }
}
