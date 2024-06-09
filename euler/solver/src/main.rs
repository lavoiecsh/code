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
mod p0040;
mod p0041;
mod p0042;
mod p0043;
mod p0044;
mod p0045;
mod p0046;
mod p0047;
mod p0048;
mod p0049;
mod p0050;
mod p0051;
mod p0052;
mod p0053;
mod p0054;

use std::env;
use std::process::Command;
use std::time::SystemTime;

use itertools::Itertools;

pub type Solver = fn () -> String;

fn main() -> Result<(), String> {
    let problem: usize = env::args().nth(1)
        .or_else(get_modified_solver)
        .ok_or_else(|| "No argument passed and no modified solver in tree".to_string())?
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    println!("Solving problem {problem}");

    let solver: Solver = get_solver(problem)?;

    let now = SystemTime::now();
    println!("Solution: {}", solver());
    now.elapsed()
        .map(|d| println!("Time: {}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000))
        .map_err(|e| e.to_string())
}

fn get_modified_solver() -> Option<String> {
    Command::new(r"sh")
        .arg("-c")
        .arg(r"git status --porcelain | sed -n 's/.*\/p\([0-9]\{4\}\)\.rs/\1/p' | tail -1")
        .output()
        .ok()
        .map(|o| o.stdout.iter().take(4).map(|&c| c as char).join(""))
}

#[allow(unused_variables)]
pub fn get_solver(problem: usize) -> Result<Solver, String> {
    include!(concat!(env!("OUT_DIR"), "/problems.txt"))
}
