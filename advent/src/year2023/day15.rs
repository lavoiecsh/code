use crate::solver::AdventSolver;

pub struct Advent2023Day15Solver {
    steps: Vec<String>,
}

impl Advent2023Day15Solver {
    pub fn new(input: String) -> Self {
        Self { steps: input.split(",").map(String::from).collect() }
    }
}

impl AdventSolver for Advent2023Day15Solver {
    fn solve_part1(&self) -> usize {
        self.steps.iter()
            .map(|step| hash(step))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut boxes = vec!();
        boxes.resize_with(256, Box::default);
        self.steps.iter()
            .for_each(|step| {
                if step.ends_with('-') {
                    let label = &step[0..step.len() - 1];
                    boxes[hash(&label)].remove(label);
                } else {
                    let label = &step[0..step.len() - 2];
                    boxes[hash(&label)].set(label, step[step.len() - 1..].parse().unwrap());
                }
            });
        boxes.iter().map(|b| b.power()).sum()
    }
}

#[derive(Default)]
struct Box {
    lenses: Vec<(String, usize)>,
}

impl Box {
    fn power(&self) -> usize {
        if self.lenses.len() == 0 { return 0; }
        let id = hash(&self.lenses[0].0) + 1;
        self.lenses.iter()
            .enumerate()
            .map(|(i,(_, p))| id * (i + 1) * p)
            .sum()
    }

    fn remove(&mut self, label: &str) {
        if let Some(index) = self.lenses.iter().position(|l| l.0 == label) {
            self.lenses.remove(index);
        }
    }

    fn set(&mut self, label: &str, power: usize) {
        if let Some(index) = self.lenses.iter().position(|l| l.0 == label) {
            self.lenses[index].1 = power;
        } else {
            self.lenses.push((label.to_string(), power));
        }
    }
}

fn hash(input: &str) -> usize {
    let mut value = 0;
    for c in input.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day15Solver {
    Advent2023Day15Solver::new(String::from("\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\
"))
}

#[test]
fn hashes() {
    let solver = test_solver_1();
    let hashes: Vec<usize> = solver.steps.iter().map(|s| hash(s)).collect();
    assert_eq!(hashes, vec!(30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231));
    assert_eq!(solver.solve_part1(), 1320);
}

#[test]
fn labels() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 145);
}
