use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

use crate::solver::AdventSolver;

#[derive(Clone, Debug)]
struct Element {
    value: Option<usize>,
    list: Option<Vec<Element>>,
}

impl Element {
    fn new_value(value: usize) -> Self {
        Self {
            value: Some(value),
            list: None,
        }
    }

    fn new_list(elements: Vec<Element>) -> Self {
        Self {
            value: None,
            list: Some(elements.clone()),
        }
    }

    fn compare_to(&self, other: &Self) -> Ordering {
        if self.is_value() && other.is_value() {
            if self.as_value() < other.as_value() {
                return Less;
            }
            if self.as_value() == other.as_value() {
                return Equal;
            }
            return Greater;
        }
        if self.is_value() && other.is_list() {
            return self.to_list().compare_to(other);
        }
        if self.is_list() && other.is_value() {
            return self.compare_to(&other.to_list());
        }
        let self_list = self.as_list();
        let other_list = other.as_list();
        for i in 0..(usize::min(self_list.len(), other_list.len())) {
            match self_list[i].compare_to(&other_list[i]) {
                Less => return Less,
                Equal => {}
                Greater => return Greater,
            }
        }
        if self_list.len() < other_list.len() {
            return Less;
        }
        if self_list.len() == other_list.len() {
            return Equal;
        }
        return Greater;
    }

    fn is_value(&self) -> bool {
        self.value.is_some()
    }

    fn is_list(&self) -> bool {
        self.list.is_some()
    }

    fn as_value(&self) -> usize {
        self.value.unwrap()
    }

    fn as_list(&self) -> Vec<Element> {
        self.list.clone().unwrap()
    }

    fn to_list(&self) -> Self {
        Self::new_list(vec!(self.clone()))
    }
}

pub struct Advent2022Day13Solver {
    packets: Vec<Element>,
}

fn elementize(s: &String) -> Option<Element> {
    if s.is_empty() {
        return None;
    }
    let mut chars: Vec<char> = s.chars().collect();
    if chars[0] != '[' {
        return Some(Element::new_value(s.parse().unwrap()));
    }
    chars.remove(0);
    chars.pop();
    let mut list_depth = 0;
    let mut elements: Vec<String> = vec!();
    let mut current_element: Vec<char> = vec!();
    for c in chars {
        match c {
            '[' => {
                list_depth += 1;
                current_element.push(c);
            }
            ']' => {
                list_depth -= 1;
                current_element.push(c);
            }
            ',' => {
                if list_depth == 0 {
                    elements.push(current_element.iter().collect());
                    current_element = vec!();
                } else {
                    current_element.push(c);
                }
            }
            x => current_element.push(x),
        }
    }
    elements.push(current_element.iter().collect());
    Some(Element::new_list(elements
        .iter()
        .map(|e| elementize(&e.to_string()))
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect()))
}

impl Advent2022Day13Solver {
    pub fn new(input: String) -> Self {
        Self {
            packets: input
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| elementize(&l.to_string()).unwrap())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2022Day13Solver {
    fn solve_part1(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.packets.len() / 2 {
            if self.packets[i * 2].compare_to(&self.packets[i * 2 + 1]) == Less {
                sum += i + 1;
            }
        }
        sum
    }

    fn solve_part2(&self) -> usize {
        let divider2 = elementize(&"[[2]]".to_string()).unwrap();
        let divider6 = elementize(&"[[6]]".to_string()).unwrap();
        let mut packets: Vec<(usize, &Element)> = self.packets
            .iter()
            .enumerate()
            .map(|(i, e)| (i + 2, e))
            .collect();
        packets.insert(0, (0, &divider2));
        packets.insert(1, (1, &divider6));
        packets.sort_by(|(_, e1), (_, e2)| e1.compare_to(e2));
        let index2 = packets.iter().position(|(i, _)| *i == 0).unwrap() + 1;
        let index6 = packets.iter().position(|(i, _)| *i == 1).unwrap() + 1;
        index2 * index6
    }
}
