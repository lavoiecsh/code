use fs::read_dir;
use std::collections::HashMap;
use std::fs;

fn main() {
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
        let last_day = format_day(&y.1.iter().max().unwrap());
        let mut mods: Vec<String> = Vec::new();
        for d in y.1 {
            let day = format_day(d);
            mods.push(format!("pub mod day{};", day));
            matches.push(format!("    (Some({}), Some({})) => Ok((|input: String| Box::new(year{}::day{}::Advent{}Day{}Solver::new(input)), \"{}\".to_string(), \"{}\".to_string())),", y.0, day, y.0, day, y.0, day, y.0, day));
        }
        matches.push(format!("    (Some({}), None) => Ok((|input: String| Box::new(year{}::day{}::Advent{}Day{}Solver::new(input)), \"{}\".to_string(), \"{}\".to_string())),", y.0, y.0, last_day, y.0, last_day, y.0, last_day));
        matches.push(format!("    (Some({}), Some(d)) => Err(AdventError::UnknownDay({}, *d)),", y.0, y.0));
        fs::write(format!("src/year{}/mod.rs", y.0), mods.join("\n")).unwrap();
    }

    let years = &solvers.into_keys().collect::<Vec<_>>();
    let last_year = years.iter().max().unwrap();
    let text = format!("\
        use crate::{{AdventError, AdventSolverBuilder, {}}};\n\
        pub fn solver_builder(year: &Option<u16>, day: &Option<u8>) -> Result<(AdventSolverBuilder, String, String), AdventError> {{\n\
          match (year, day) {{\n\
            {}\n\
            (None, d) => solver_builder(&Some({}), d),\n\
            (Some(y), _) => Err(AdventError::UnknownYear(*y)),
          }}\n\
        }}\n",
                       years.iter().map(|y| format!("year{}", y)).collect::<Vec<_>>().join(", "),
                       matches.join("\n"),
                       last_year);

    fs::write("src/solver_builder.rs", text).unwrap();
}

fn format_day(day: &u8) -> String {
    format!("{:02}", day)
}
