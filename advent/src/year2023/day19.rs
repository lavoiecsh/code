use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

use regex::{Match, Regex};

use crate::solver::AdventSolver;

pub struct Advent2023Day19Solver {
    workflow_engine: WorkflowEngine,
    parts: Vec<Part>,
}

impl Advent2023Day19Solver {
    pub fn new(input: String) -> Self {
        let part_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        let part_parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        let workflow_re = Regex::new(r"(\w+)\{(.+),(\w+)\}").unwrap();
        let mut reading_workflows = true;
        let mut workflows = HashMap::new();
        let mut parts = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                reading_workflows = false;
                continue;
            }
            if reading_workflows {
                let cap = workflow_re.captures(line).unwrap();
                workflows.insert(cap.get(1).unwrap().as_str().to_string(), Workflow {
                    rules: cap.get(2).unwrap().as_str().split(',')
                        .map(|r| Rule {
                            var: r.chars().nth(0).unwrap(),
                            op: r.chars().nth(1).unwrap(),
                            val: r[2..].split(':').next().unwrap().parse().unwrap(),
                            to: r.split(':').nth(1).unwrap().to_string(),
                        })
                        .collect(),
                    last: cap.get(3).unwrap().as_str().to_string(),
                });
            } else {
                let cap = part_re.captures(line).unwrap();
                parts.push(Part {
                    x: part_parse(cap.get(1)),
                    m: part_parse(cap.get(2)),
                    a: part_parse(cap.get(3)),
                    s: part_parse(cap.get(4)),
                })
            }
        }
        Self {
            workflow_engine: WorkflowEngine::new(workflows),
            parts,
        }
    }
}

impl AdventSolver for Advent2023Day19Solver {
    fn solve_part1(&self) -> usize {
        self.parts.iter()
            .filter(|part| self.workflow_engine.is_accepted(part))
            .map(|part| part.rating_sum())
            .sum::<u64>() as usize
    }

    fn solve_part2(&self) -> usize {
        self.workflow_engine.count_accepted()
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn rating_sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

struct WorkflowEngine {
    accepted: Vec<WorkflowRange>,
}

#[derive(Clone)]
struct WorkflowRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl WorkflowEngine {
    fn new(workflows: HashMap<String, Workflow>) -> Self {
        let mut reversed: VecDeque<(WorkflowRange, String)> = VecDeque::new();
        reversed.push_back((WorkflowRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }, "in".to_string()));
        while reversed.iter().any(|rr| rr.1 != "A") {
            let rule = reversed.pop_front().unwrap();
            if rule.1 == "A" {
                reversed.push_back(rule);
                continue;
            }
            let workflow = workflows.get(&rule.1).unwrap();
            reversed.extend(workflow.reverse(&rule.0));
        }
        Self { accepted: reversed.into_iter().map(|rr| rr.0).collect() }
    }

    fn is_accepted(&self, part: &Part) -> bool {
        self.accepted.iter().any(|wr| wr.contains(part))
    }

    fn count_accepted(&self) -> usize {
        self.accepted.iter().cloned().map(|wr| wr.count()).sum()
    }
}

impl WorkflowRange {
    fn contains(&self, part: &Part) -> bool {
        self.x.contains(&part.x) &&
            self.m.contains(&part.m) &&
            self.a.contains(&part.a) &&
            self.s.contains(&part.s)
    }

    fn count(self) -> usize {
        self.x.count() *
            self.m.count() *
            self.a.count() *
            self.s.count()
    }

    fn split(&self, rule: &Rule) -> Option<(Self, Self)> {
        let mut included = self.clone();
        let mut excluded = self.clone();
        match (rule.var, rule.op) {
            ('x', '>') => {
                if !self.x.contains(&rule.val) { return None; }
                included.x = rule.val + 1..=*self.x.end();
                excluded.x = *self.x.start()..=rule.val;
            }
            ('x', '<') => {
                if !self.x.contains(&rule.val) { return None; }
                included.x = *self.x.start()..=rule.val - 1;
                excluded.x = rule.val..=*self.x.end();
            }
            ('m', '>') => {
                if !self.m.contains(&rule.val) { return None; }
                included.m = rule.val + 1..=*self.m.end();
                excluded.m = *self.m.start()..=rule.val;
            }
            ('m', '<') => {
                if !self.m.contains(&rule.val) { return None; }
                included.m = *self.m.start()..=rule.val - 1;
                excluded.m = rule.val..=*self.m.end();
            }
            ('a', '>') => {
                if !self.a.contains(&rule.val) { return None; }
                included.a = rule.val + 1..=*self.a.end();
                excluded.a = *self.a.start()..=rule.val;
            }
            ('a', '<') => {
                if !self.a.contains(&rule.val) { return None; }
                included.a = *self.a.start()..=rule.val - 1;
                excluded.a = rule.val..=*self.a.end();
            }
            ('s', '>') => {
                if !self.s.contains(&rule.val) { return None; }
                included.s = rule.val + 1..=*self.s.end();
                excluded.s = *self.s.start()..=rule.val;
            }
            ('s', '<') => {
                if !self.s.contains(&rule.val) { return None; }
                included.s = *self.s.start()..=rule.val - 1;
                excluded.s = rule.val..=*self.s.end();
            }
            _ => panic!("unknown splitting combination {} {} {}", rule.var, rule.op, rule.val)
        };
        Some((included, excluded))
    }
}

struct Workflow {
    rules: Vec<Rule>,
    last: String,
}

impl Workflow {
    fn reverse(&self, from: &WorkflowRange) -> Vec<(WorkflowRange, String)> {
        let mut reversed = vec!();
        let mut current = from.clone();
        for rule in &self.rules {
            if let Some((included, excluded)) = current.split(rule) {
                if rule.to != "R" {
                    reversed.push((included, rule.to.clone()));
                }
                current = excluded;
            }
        }
        if self.last != "R" {
            reversed.push((current, self.last.clone()));
        }
        reversed
    }
}

struct Rule {
    var: char,
    op: char,
    val: u64,
    to: String,
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day19Solver {
    Advent2023Day19Solver::new(String::from("\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"))
}

#[test]
fn finds_rating_numbers_of_accepted_parts() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part1(), 19114);
}

#[test]
fn counts_accepted_combinations() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 167409079868000);
}
