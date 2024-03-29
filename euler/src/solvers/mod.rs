mod p0001;
mod p0002;
mod p0003;
mod p0004;

pub type Solver = fn () -> usize;

#[allow(unused_variables)]
pub fn get_solver(problem: usize) -> Result<Solver, String> {
    include!(concat!(env!("OUT_DIR"), "/problems.txt"))
}
