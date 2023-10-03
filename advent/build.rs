use fs::read_dir;
use std::collections::HashMap;
use std::env::var_os;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let files = read_dir("src").unwrap();

    let mut solvers: HashMap<u16, Vec<u8>> = HashMap::new();
    for f in files {
        let file = f.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        if file_name.starts_with("year") {
            let year: u16 = file_name.split_at(4).1.parse().unwrap();
            let mut days: Vec<u8> = Vec::new();
            for d in read_dir(file.path().as_os_str()).unwrap() {
                let day_file_name = d.unwrap().file_name().into_string().unwrap();
                if day_file_name.starts_with("day") && day_file_name.ends_with(".rs") {
                    let day: u8 = day_file_name.split_at(3).1.split_at(2).0.parse().unwrap();
                    days.push(day);
                }
            }
            days.sort();
            solvers.insert(year, days);
        }
    }

    let mut matches: Vec<String> = Vec::new();

    for y in &solvers {
        let year = y.0;
        y.1.iter().for_each(|d| matches.push(format!("    (Some({year}), Some({d})) => {},", format_solver_builder(year, d))));
        matches.push(format!("    (Some({year}), None) => {},", format_solver_builder(year, y.1.iter().max().unwrap())));
        matches.push(format!("    (Some({year}), Some(d)) => Err(AdventError::UnknownDay({year}, *d)),"));
    }

    let years = &solvers.into_keys().collect::<Vec<_>>();
    let last_year = years.iter().max().unwrap();

    matches.push(format!("    (None, d) => solver_builder(&Some({last_year}), d),"));
    matches.push(format!("    (Some(y), _) => Err(AdventError::UnknownYear(*y)),"));

    let path = Path::new(&var_os("OUT_DIR").unwrap()).join("matches.txt");
    let text = format!("  match (year, day) {{\n{}\n  }}", matches.join("\n"));
    fs::write(path, text)
}

fn format_solver_builder(year: &u16, day: &u8) -> String {
    format!("Ok((|input: String| Box::new(crate::year{year}::day{day:02}::Advent{year}Day{day:02}Solver::new(input)), \"{year}\".to_string(), \"{day:02}\".to_string()))")
}
