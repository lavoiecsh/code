use crate::solver::AdventSolver;

pub struct Advent2016Day07Solver {
    ips: Vec<IP>,
}

impl Advent2016Day07Solver {
    pub fn new(input: &str) -> Self {
        Self {
            ips: input
                .lines()
                .map(|l| {
                    let splits = l.split("[");
                    let sections = splits
                        .flat_map(|s| {
                            if s.contains("]") {
                                let mut t = s.split("]");
                                vec![
                                    IPSection::Hypernet(t.next().unwrap().to_string()),
                                    IPSection::Supernet(t.next().unwrap().to_string()),
                                ]
                            } else {
                                vec![IPSection::Supernet(s.to_string())]
                            }
                        })
                        .collect();
                    IP { sections }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2016Day07Solver {
    fn solve_part1(&self) -> usize {
        self.ips.iter().filter(|ip| ip.supports_tls()).count()
    }

    fn solve_part2(&self) -> usize {
        self.ips.iter().filter(|ip| ip.supports_ssl()).count()
    }
}

struct IP {
    sections: Vec<IPSection>,
}

enum IPSection {
    Supernet(String),
    Hypernet(String),
}

impl IPSection {
    fn is_supernet(&self) -> bool {
        match self {
            IPSection::Supernet(_) => true,
            IPSection::Hypernet(_) => false,
        }
    }

    fn str(&self) -> &str {
        match self {
            IPSection::Supernet(s) => s,
            IPSection::Hypernet(s) => s,
        }
    }
}

impl IP {
    fn supports_tls(&self) -> bool {
        self.sections
            .iter()
            .filter(|s| s.is_supernet())
            .map(|s| s.str())
            .any(has_abba)
            && !self
                .sections
                .iter()
                .filter(|s| !s.is_supernet())
                .map(|s| s.str())
                .any(has_abba)
    }

    fn supports_ssl(&self) -> bool {
        let hypernets: Vec<&str> = self
            .sections
            .iter()
            .filter(|s| !s.is_supernet())
            .map(|s| s.str())
            .collect();
        self.sections
            .iter()
            .filter(|s| s.is_supernet())
            .map(|s| s.str())
            .flat_map(bab_sequences)
            .any(|aba| hypernets.iter().any(|h| h.contains(&aba)))
    }
}

fn has_abba(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    (3..chars.len()).any(|i| {
        chars[i - 3] == chars[i] && chars[i - 1] == chars[i - 2] && chars[i] != chars[i - 1]
    })
}

fn bab_sequences(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    (2..chars.len())
        .filter(|i| chars[i - 2] == chars[*i] && chars[i - 1] != chars[*i])
        .map(|i| format!("{1}{0}{1}", chars[i], chars[i - 1]))
        .collect()
}
