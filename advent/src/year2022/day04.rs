use std::fs::read_to_string;
use crate::solver::AdventSolver;

struct SectionAssignment {
    from: usize,
    to: usize,
}

type SectionAssignmentPair = (SectionAssignment, SectionAssignment);

pub struct Advent2022Day04Solver {
    section_assignment_pairs: Vec<SectionAssignmentPair>,
}

impl Advent2022Day04Solver {
    pub fn new() -> Self {
        Self {
            section_assignment_pairs: read_to_string("src/year2022/day04.txt")
                .unwrap()
                .trim()
                .lines()
                .map(|l| {
                    let mut s = l.split(",");
                    let mut s1 = s.next().unwrap().split("-");
                    let mut s2 = s.next().unwrap().split("-");
                    (SectionAssignment {
                        from: s1.next().unwrap().parse().unwrap(),
                        to: s1.next().unwrap().parse().unwrap(),
                    }, SectionAssignment {
                        from: s2.next().unwrap().parse().unwrap(),
                        to: s2.next().unwrap().parse().unwrap(),
                    })
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day04Solver {
    fn day(&self) -> usize { 04 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        self.section_assignment_pairs
            .iter()
            .filter(|(e1,e2)| fully_contains(e1, e2) || fully_contains(e2, e1))
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.section_assignment_pairs
            .iter()
            .filter(|(e1, e2)| overlap(e1, e2) || overlap(e2, e1))
            .count()
    }
}

fn fully_contains(sa1: &SectionAssignment, sa2: &SectionAssignment) -> bool {
    sa1.from >= sa2.from && sa1.to <= sa2.to
}

fn overlap(sa1: &SectionAssignment, sa2: &SectionAssignment) -> bool {
    (sa1.from >= sa2.from && sa1.from <= sa2.to) || (sa1.to >= sa2.from && sa1.to <= sa2.to)
}
