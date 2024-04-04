mod p0001;
mod p0002;
mod p0003;
mod p0004;
mod p0005;
mod p0006;
mod p0007;
mod p0008;
mod p0009;
mod p0010;
mod p0011;
mod p0012;
mod p0013;
mod p0014;
mod p0015;

pub type Solver = fn () -> String;

#[allow(unused_variables)]
pub fn get_solver(problem: usize) -> Result<Solver, String> {
    include!(concat!(env!("OUT_DIR"), "/problems.txt"))
}
