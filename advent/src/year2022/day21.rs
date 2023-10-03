use std::collections::HashMap;

use crate::solver::AdventSolver;

#[derive(Clone)]
struct Monkey {
    value: Option<isize>,
    operation: Option<(char, String, String)>,
}

impl Monkey {
    fn new_value(value: isize) -> Self {
        Self { value: Some(value), operation: None }
    }

    fn new_operation(first: String, operation: char, second: String) -> Self {
        Self { value: None, operation: Some((operation, first, second)) }
    }

    fn evaluate(&self, value_map: &HashMap<String, isize>) -> Option<isize> {
        if self.value.is_some() {
            return Some(self.value.unwrap());
        }
        let o1 = value_map.get(self.operation.clone().unwrap().1.as_str());
        let o2 = value_map.get(self.operation.clone().unwrap().2.as_str());
        match (o1, o2) {
            (Some(a), Some(b)) => Some(match self.operation.clone().unwrap().0 {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                '=' => if a == b { 1 } else { 0 },
                _ => panic!("unknown operation"),
            }),
            _ => None
        }
    }
}

pub struct Advent2022Day21Solver {
    monkeys: HashMap<String, Monkey>,
}

impl Advent2022Day21Solver {
    pub fn new(input: String) -> Self {
        Self {
            monkeys: input
                .lines()
                .map(|l| {
                    let mut s = l.split(": ");
                    let n = s.next().unwrap().to_string();
                    let v = s.next().unwrap();
                    match v.parse::<isize>() {
                        Ok(value) => (n, Monkey::new_value(value)),
                        _ => {
                            let mut s2 = v.split(" ");
                            (n, Monkey::new_operation(s2.next().unwrap().to_string(),
                                                      s2.next().unwrap().chars().next().unwrap(),
                                                      s2.next().unwrap().to_string()))
                        }
                    }
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day21Solver {
    fn solve_part1(&self) -> usize {
        let mut values: HashMap<String, isize> = HashMap::new();
        evaluate(&self.monkeys, &mut values);
        *values.get("root").unwrap() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut values: HashMap<String, isize> = HashMap::new();
        let mut monkeys = self.monkeys.clone();
        let (_, a_name, b_name) = monkeys.get("root").unwrap().operation.clone().unwrap();
        monkeys.insert(String::from("root"), Monkey::new_operation(a_name.clone(), '=', b_name.clone()));

        evaluate(&monkeys, &mut values);
        let compare = isize::cmp(values.get(&a_name).unwrap(), values.get(&b_name).unwrap());

        let mut human = 0;
        let mut human_jump = 1000000000000;
        while values.get("root").unwrap() != &1 {
            human += human_jump;
            values.clear();
            values.insert(String::from("humn"), human);
            evaluate(&monkeys, &mut values);
            if isize::cmp(values.get(&a_name).unwrap(), values.get(&b_name).unwrap()) != compare {
                human -= human_jump;
                human_jump /= 10;
            }
        }
        *values.get("humn").unwrap() as usize
    }
}

fn evaluate(monkeys: &HashMap<String, Monkey>, values: &mut HashMap<String, isize>) -> () {
    while !values.contains_key("root") {
        for (name, monkey) in monkeys {
            if values.contains_key(name) { continue; }
            let value = monkey.evaluate(&values);
            if value.is_some() {
                values.insert(name.clone(), value.unwrap());
            }
        }
    }
}
