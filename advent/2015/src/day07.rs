use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs;
use crate::problem_solver::ProblemSolver;

pub struct Problem07Solver {
    operations: HashMap<String, String>
}

impl Problem07Solver {
    pub fn new() -> Self {
        Self {
            operations: fs::read_to_string("inputs/day07.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l: &str| -> (String, String) {
                    let mut s = l.split(" -> ");
                    let operation: String = String::from(s.next().unwrap());
                    let wire: String = String::from(s.next().unwrap());
                    (wire, operation)
                })
                .collect()
        }
    }
}

impl ProblemSolver for Problem07Solver {
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
            if number_result.is_ok() {
                values.insert(operation.0.clone(), number_result.unwrap());
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
                let left = if left_number.is_ok() { left_number.unwrap() } else { *left_value.unwrap() };
                let right = if right_number.is_ok() { right_number.unwrap() } else { *right_value.unwrap() };
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
                let left = if left_number.is_ok() { left_number.unwrap() } else { *left_value.unwrap() };
                let right = if right_number.is_ok() { right_number.unwrap() } else { *right_value.unwrap() };
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
