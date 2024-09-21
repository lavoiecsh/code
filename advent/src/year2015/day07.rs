use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::solver::AdventSolver;

pub struct Advent2015Day07Solver {
    operations: HashMap<String, String>
}

impl Advent2015Day07Solver {
    pub fn new(input: String) -> Self {
        Self {
            operations: input
                .lines()
                .map(|l| {
                    let mut s = l.split(" -> ");
                    let operation = String::from(s.next().unwrap());
                    let wire = String::from(s.next().unwrap());
                    (wire, operation)
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2015Day07Solver {
    fn solve_part1(&self) -> usize {
        let mut values: HashMap<String, usize> = HashMap::new();
        let mut operations = self.operations.clone();
        compute_a(&mut operations, &mut values);
        *values.get("a").unwrap_or(&0)
    }

    fn solve_part2(&self) -> usize {
        let mut values: HashMap<String, usize> = HashMap::new();
        let mut operations = self.operations.clone();
        compute_a(operations.clone().borrow_mut(), &mut values);
        let mut values2: HashMap<String, usize> = HashMap::new();
        values2.insert(String::from("b"), *values.get("a").unwrap());
        operations.remove("b");
        compute_a(&mut operations, &mut values2);
        *values2.get("a").unwrap_or(&0)
    }
}

fn compute_a(operations: &mut HashMap<String, String>, values: &mut HashMap<String, usize>) {
    while values.get("a").is_none() {
        let prev_size = values.len();
        for operation in operations.clone() {
            let number_result = operation.1.parse::<usize>();
            if let Ok(number) = number_result {
                values.insert(operation.0.clone(), number);
                operations.remove(&operation.0);
                break;
            }
            let split: Vec<&str> = operation.1.split(" ").collect();
            if split.len() == 1 {
                let value = values.get(split[0]);
                if value.is_none() {
                    continue;
                }
                values.insert(operation.0.clone(), *value.unwrap());
                operations.remove(&operation.0);
                break;
            }
            if split[0] == "NOT" {
                let value = values.get(split[1]);
                if value.is_none() {
                    continue;
                }
                values.insert(operation.0.clone(), !*value.unwrap());
                operations.remove(&operation.0);
                break;
            }
            if split[1] == "AND" {
                let left_number = split[0].parse::<usize>();
                let left_value = values.get(split[0]);
                let right_number = split[2].parse::<usize>();
                let right_value = values.get(split[2]);
                if (left_number.is_err() && left_value.is_none()) || (right_number.is_err() && right_value.is_none()) {
                    continue
                }
                let left = if let Ok(n) = left_number { n } else { *left_value.unwrap() };
                let right = if let Ok(n) = right_number { n } else { *right_value.unwrap() };
                values.insert(operation.0.clone(), left & right);
                operations.remove(&operation.0);
                break;
            }
            if split[1] == "OR" {
                let left_number = split[0].parse::<usize>();
                let left_value = values.get(split[0]);
                let right_number = split[2].parse::<usize>();
                let right_value = values.get(split[2]);
                if (left_number.is_err() && left_value.is_none()) || (right_number.is_err() && right_value.is_none()) {
                    continue
                }
                let left = if let Ok(n) = left_number { n } else { *left_value.unwrap() };
                let right = if let Ok(n) = right_number { n } else { *right_value.unwrap() };
                values.insert(operation.0.clone(), left | right);
                operations.remove(&operation.0);
                break;
            }
            if split[1] == "RSHIFT" {
                let value = values.get(split[0]);
                if value.is_none() {
                    continue;
                }
                let amount: usize = split[2].parse().unwrap();
                values.insert(operation.0.clone(), value.unwrap() >> amount);
                operations.remove(&operation.0);
                break;
            }
            if split[1] == "LSHIFT" {
                let value = values.get(split[0]);
                if value.is_none() {
                    continue;
                }
                let amount: usize = split[2].parse().unwrap();
                values.insert(operation.0.clone(), value.unwrap() << amount);
                operations.remove(&operation.0);
                break;
            }
        }
        if values.len() == prev_size {
            break;
        }
    }
}
