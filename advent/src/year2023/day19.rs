use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
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
            workflow_engine: WorkflowEngine { workflows },
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
        let reversed = self.workflow_engine.reverse();
        reversed.iter()
            .map(|r| r.distinct())
            .sum()
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
    workflows: HashMap<String, Workflow>,
}

impl WorkflowEngine {
    fn is_accepted(&self, part: &Part) -> bool {
        let mut workflow = "in";
        while workflow != "A" && workflow != "R" {
            workflow = self.workflows.get(workflow).unwrap().next(part);
        }
        workflow == "A"
    }

    fn reverse(&self) -> VecDeque<ReverseRule> {
        let mut reversed: VecDeque<ReverseRule> = VecDeque::new();
        reversed.push_back(ReverseRule::default());
        while reversed.iter().any(|rr| rr.to != "A") {
            let rule = reversed.pop_front().unwrap();
            if rule.to == "A" {
                reversed.push_back(rule);
                continue;
            }
            let workflow = self.workflows.get(&rule.to).unwrap();
            reversed.extend(workflow.reverse(&rule));
        }
        reversed
    }
}

struct Workflow {
    rules: Vec<Rule>,
    last: String,
}

impl Workflow {
    fn next(&self, part: &Part) -> &str {
        self.rules.iter()
            .find(|rule| rule.accepts(part))
            .map(|rule| rule.to.as_str())
            .unwrap_or_else(|| self.last.as_str())
    }

    fn reverse(&self, from: &ReverseRule) -> Vec<ReverseRule> {
        let mut reversed = vec!();
        let mut current = from.clone();
        for rule in &self.rules {
            if let Some((included, excluded)) = current.split(&rule) {
                if included.to != "R" {
                    reversed.push(included);
                }
                current = excluded;
            }
        }
        if self.last != "R" {
            current.to = self.last.clone();
            reversed.push(current);
        }
        reversed
    }
}

#[derive(Clone)]
struct ReverseRule {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
    to: String,
}

impl ReverseRule {
    fn distinct(&self) -> usize {
        self.x.clone().count() *
            self.m.clone().count() *
            self.a.clone().count() *
            self.s.clone().count()
    }

    fn split(&self, rule: &Rule) -> Option<(ReverseRule, ReverseRule)> {
        if self.to == rule.to { return None; }
        match (rule.var, rule.op) {
            ('x', '>') => {
                if self.x.contains(&rule.val) {
                    let mut included = self.clone();
                    included.x = rule.val+1..=*self.x.end();
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.x = *self.x.start()..=rule.val;
                    Some((included, excluded))
                } else { None }
            }
            ('x', '<') => {
                if self.x.contains(&rule.val) {
                    let mut included = self.clone();
                    included.x = *self.x.start()..=rule.val-1;
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.x = rule.val..=*self.x.end();
                    Some((included, excluded))
                } else { None }
            }
            ('m', '>') => {
                if self.m.contains(&rule.val) {
                    let mut included = self.clone();
                    included.m = rule.val+1..=*self.m.end();
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.m = *self.m.start()..=rule.val;
                    Some((included, excluded))
                } else { None }
            }
            ('m', '<') => {
                if self.m.contains(&rule.val) {
                    let mut included = self.clone();
                    included.m = *self.m.start()..=rule.val-1;
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.m = rule.val..=*self.m.end();
                    Some((included, excluded))
                } else { None }
            }
            ('a', '>') => {
                if self.a.contains(&rule.val) {
                    let mut included = self.clone();
                    included.a = rule.val+1..=*self.a.end();
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.a = *self.a.start()..=rule.val;
                    Some((included, excluded))
                } else { None }
            }
            ('a', '<') => {
                if self.a.contains(&rule.val) {
                    let mut included = self.clone();
                    included.a = *self.a.start()..=rule.val-1;
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.a = rule.val..=*self.a.end();
                    Some((included, excluded))
                } else { None }
            }
            ('s', '>') => {
                if self.s.contains(&rule.val) {
                    let mut included = self.clone();
                    included.s = rule.val+1..=*self.s.end();
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.s = *self.s.start()..=rule.val;
                    Some((included, excluded))
                } else { None }
            }
            ('s', '<') => {
                if self.s.contains(&rule.val) {
                    let mut included = self.clone();
                    included.s = *self.s.start()..=rule.val-1;
                    included.to = rule.to.clone();
                    let mut excluded = self.clone();
                    excluded.s = rule.val..=*self.s.end();
                    Some((included, excluded))
                } else { None }
            }
            _ => panic!("unknown splitting combination {} {} {}", rule.var, rule.op, rule.val)
        }
    }
}

impl Default for ReverseRule {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
            to: String::from("in"),
        }
    }
}

impl Debug for ReverseRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x -> {}..={}, m -> {}..={}, a -> {}..={}, s -> {}..={}, -> {}",
                                 self.x.start(), self.x.end(),
                                 self.m.start(), self.m.end(),
                                 self.a.start(), self.a.end(),
                                 self.s.start(), self.s.end(),
                                 self.to,
        ))
    }
}

#[derive(Debug)]
struct Rule {
    var: char,
    op: char,
    val: u64,
    to: String,
}

impl Rule {
    fn accepts(&self, part: &Part) -> bool {
        let rating = match self.var {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("unknown part variable {}", self.var),
        };
        match self.op {
            '>' => rating > self.val,
            '<' => rating < self.val,
            _ => panic!("unknown part operation {}", self.op),
        }
    }
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
