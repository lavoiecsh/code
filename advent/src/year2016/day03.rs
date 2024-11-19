use crate::solver::AdventSolver;
use itertools::Itertools;

pub struct Advent2016Day03Solver {
    triangles: Vec<Triangle>,
}

impl Advent2016Day03Solver {
    pub fn new(input: &str) -> Self {
        Self {
            triangles: input
                .lines()
                .map(|l| {
                    let (first, rest) = l.split_at(5);
                    let (second, third) = rest.split_at(5);
                    Triangle {
                        a: first.trim().parse().unwrap(),
                        b: second.trim().parse().unwrap(),
                        c: third.trim().parse().unwrap(),
                    }
                })
                .collect(),
        }
    }

    fn rotate_triangles(&self) -> Vec<Triangle> {
        self.triangles
            .iter()
            .chunks(3)
            .into_iter()
            .flat_map(|c| {
                let mut i = c.into_iter();
                let t = (i.next().unwrap(), i.next().unwrap(), i.next().unwrap());
                vec![
                    Triangle {
                        a: t.0.a,
                        b: t.1.a,
                        c: t.2.a,
                    },
                    Triangle {
                        a: t.0.b,
                        b: t.1.b,
                        c: t.2.b,
                    },
                    Triangle {
                        a: t.0.c,
                        b: t.1.c,
                        c: t.2.c,
                    },
                ]
            })
            .collect()
    }
}

impl AdventSolver for Advent2016Day03Solver {
    fn solve_part1(&self) -> usize {
        self.triangles.iter().filter(|t| t.is_valid()).count()
    }

    fn solve_part2(&self) -> usize {
        self.rotate_triangles()
            .iter()
            .filter(|t| t.is_valid())
            .count()
    }
}

struct Triangle {
    a: u16,
    b: u16,
    c: u16,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}
