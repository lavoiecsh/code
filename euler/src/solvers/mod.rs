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
mod p0016;
mod p0017;
mod p0018;
mod p0019;
mod p0020;
mod p0021;
mod p0022;
mod p0023;
mod p0024;
mod p0025;
mod p0026;
mod p0027;
mod p0028;
mod p0029;
mod p0030;
mod p0031;
mod p0032;
mod p0033;
mod p0034;
mod p0035;
mod p0036;
mod p0037;
mod p0038;
mod p0039;

pub type Solver = fn () -> String;

#[allow(unused_variables)]
pub fn get_solver(problem: usize) -> Result<Solver, String> {
    include!(concat!(env!("OUT_DIR"), "/problems.txt"))
}
