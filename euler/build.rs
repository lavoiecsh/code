use std::env::var_os;
use std::fs::{read_dir, write};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let files = read_dir("src/solvers")?;
    let solvers = files.into_iter()
        .filter_map(|f| f.ok())
        .filter_map(|f| f.file_name().into_string().ok())
        .filter(|n| n.starts_with('p'))
        .map(|n| n.trim_start_matches('p').to_string())
        .map(|n| n.trim_end_matches(".rs").to_string())
        .filter_map(|n| n.parse::<usize>().ok())
        .map(|s| format!("    {s} => Ok(p{s:04}::p{s:04}_solver),"))
        .collect::<Vec<String>>()
        .join("\n");

    let path = Path::new(&var_os("OUT_DIR").unwrap()).join("problems.txt");
    let text = format!("  match problem {{\n{solvers}\n    p => Err(format!(\"Solution for problem {{p}} not found\"))\n  }}");
    write(path, text)
}
