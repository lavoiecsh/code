use crate::solver::AdventSolver;
use itertools::Itertools;
use num_integer::Integer;
use std::cmp::Ordering;
use std::fmt::{Debug, Write};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

pub struct Advent2025Day10Solver {
    machines: Vec<Machine>,
}

impl Advent2025Day10Solver {
    pub fn new(input: &str) -> Self {
        Self {
            machines: input.lines().map(Machine::from).collect(),
        }
    }
}

impl AdventSolver for Advent2025Day10Solver {
    fn solve_part1(&self) -> usize {
        self.machines
            .iter()
            .map(Machine::fewest_indicator_lights_button_presses)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.machines
            .iter()
            // .take(1)
            .map(Machine::fewest_joltage_requirements_button_presses)
            .sum()
    }
}

struct Machine {
    buttons: Buttons,
    lights: LightRequirements,
    joltages: JoltageRequirements,
}

type Buttons = Vec<Vec<usize>>;
type LightRequirements = Vec<bool>;
type JoltageRequirements = Vec<i64>;

macro_rules! remove_wrap {
    ($input: expr) => {
        $input
            .strip_prefix(['[', '(', '{'])
            .unwrap()
            .strip_suffix([']', ')', '}'])
            .unwrap()
    };
}

impl Machine {
    fn from(input: &str) -> Self {
        let split = input.split(' ').collect_vec();
        let lights = remove_wrap!(split[0]).chars().map(|c| c == '#').collect();
        let buttons = split[1..split.len() - 1]
            .iter()
            .map(|w| {
                remove_wrap!(w)
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let joltages = remove_wrap!(split[split.len() - 1])
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        Self {
            buttons,
            lights,
            joltages,
        }
    }

    fn fewest_indicator_lights_button_presses(&self) -> usize {
        let maximum = 2usize.pow(self.buttons.len() as u32);
        let mut best = usize::MAX;
        for test in 0..maximum {
            let mut lights = vec![false; self.lights.len()];
            let mut test = test;
            let mut count = 0;
            let mut index = 0;
            while test > 0 {
                if test % 2 == 1 {
                    count += 1;
                    self.buttons[index]
                        .iter()
                        .for_each(|&i| lights[i] = !lights[i]);
                }
                test /= 2;
                index += 1;
            }
            if lights == self.lights && count < best {
                best = count;
            }
        }
        best
    }

    fn fewest_joltage_requirements_button_presses(&self) -> usize {
        let initial = Simplex::new(&self.buttons, &self.joltages);
        let mut simplex = initial.clone();
        if let Ok(count) = simplex.solve() {
            count.0 as usize
        } else {
            let mut best = Fractional::MAX;
            for variable in 0..simplex.variable_maximums.len() {
                for value in 0..=simplex.variable_maximums[variable] {
                    let mut new_simplex = initial.clone();
                    new_simplex.fix(variable, Fractional::new(value));
                    if let Ok(count) = new_simplex.solve() {
                        if count < best {
                            best = count;
                        }
                    }
                }
            }
            best.round_up().0 as usize
        }
    }
}

#[derive(Clone, Copy)]
struct Fractional(i64, i64);

#[derive(Clone)]
struct Simplex {
    constraints: Vec<SimplexRow>,
    solution: SimplexRow,
    variable_maximums: Vec<i64>,
    fixed_variables: Vec<Option<Fractional>>,
}

#[derive(Clone)]
struct SimplexRow {
    target: Fractional,
    buttons: Vec<Fractional>,
}

impl Simplex {
    fn new(buttons: &Buttons, requirements: &JoltageRequirements) -> Self {
        let constraints: Vec<SimplexRow> = requirements
            .iter()
            .enumerate()
            .map(|(row, &target)| SimplexRow {
                target: Fractional::new(target),
                buttons: buttons
                    .iter()
                    .map(|b| Fractional::from_bool(b.contains(&row)))
                    .collect_vec(),
            })
            .collect();
        let variable_maximums = buttons
            .iter()
            .map(|b| b.iter().map(|&i| requirements[i]).min().unwrap())
            .collect_vec();
        Self {
            solution: SimplexRow {
                target: constraints.iter().map(|c| c.target).sum(),
                buttons: (0..buttons.len())
                    .map(|i| constraints.iter().map(|c| c.buttons[i]).sum())
                    .collect(),
            },
            fixed_variables: vec![None; variable_maximums.len()],
            variable_maximums,
            constraints,
        }
    }

    fn is_valid_solution(&self) -> bool {
        self.solution.target == Fractional::ZERO
            && self.constraints.iter().all(|c| c.target.is_whole())
            && self.columns_are_one_or_less()
    }

    fn count(&self) -> Fractional {
        self.constraints_sum() + self.fixed_variables_sum()
    }

    fn constraints_sum(&self) -> Fractional {
        self.constraints.iter().map(|c| c.target).sum()
    }

    fn fixed_variables_sum(&self) -> Fractional {
        self.fixed_variables.iter().filter_map(|&v| v).sum()
    }

    fn solve(&mut self) -> Result<Fractional, &'static str> {
        let mut count = 0;
        while self.is_not_done() && count < 25 {
            self.solve_iteration()?;
            self.cleanup()?;
            count += 1;
        }
        if self.is_valid_solution() {
            Ok(self.count())
        } else {
            if self.constraints.is_empty() {
                Err("no constraints")
            } else if self.solution.target == Fractional::ZERO {
                Err("solution target not 0")
            } else if self.constraints.iter().any(|c| !c.target.is_whole()) {
                Err("fractional constraint target")
            } else {
                Err("no solution found")
            }
        }
    }

    fn solve_iteration(&mut self) -> Result<(), &'static str> {
        if let Some((column, row)) = self.select() {
            let normalized = self.constraints[row].normalize(column);
            self.solution.subtract(&normalized, column);
            for r in 0..self.constraints.len() {
                if r == row {
                    continue;
                }
                self.constraints[r].subtract(&normalized, column);
            }
        }
        Ok(())
    }

    fn select(&self) -> Option<(usize, usize)> {
        if let Some((column, _)) = self
            .solution
            .buttons
            .iter()
            .enumerate()
            .filter(|&(_, v)| v > &Fractional::ZERO)
            .sorted_by_key(|(_, v)| v.invert())
            .next()
        {
            let mut m = Fractional::MAX;
            let mut row = usize::MAX;
            for j in 0..self.constraints.len() {
                if let Some(value) = self.constraints[j].value(column) {
                    if value < m {
                        m = value;
                        row = j;
                    }
                }
            }
            if row == usize::MAX {
                None
            } else {
                Some((column, row))
            }
        } else {
            None
        }
    }

    fn cleanup(&mut self) -> Result<(), &'static str> {
        for r in 0..self.constraints.len() {
            self.constraints[r].ensure_not_all_negative();
        }
        self.solution.ensure_not_all_negative();
        self.fix_variables()?;
        Ok(())
    }

    fn fix_variables(&mut self) -> Result<(), &'static str> {
        for r in 0..self.constraints.len() {
            if let Some((variable, value)) = self.constraints[r].has_fixed_variable() {
                if let Some(fixed_value) = self.fixed_variables[variable] {
                    if value != fixed_value {
                        return Err("variable value is different from fixed value");
                    } else {
                        continue;
                    }
                }
                if !value.is_whole() {
                    return Err("variable value is not whole");
                }
                if !value.is_positive() {
                    return Err("variable value is not positive");
                }
                if value.0 > self.variable_maximums[variable] {
                    return Err("variable value is greater than maximum");
                }
                self.fix(variable, value);
                self.cleanup()?;
            }
        }
        Ok(())
    }

    fn is_not_done(&self) -> bool {
        self.select().is_some() || self.solution.buttons.iter().any(|&v| v > Fractional::ZERO)
    }

    fn columns_are_one_or_less(&self) -> bool {
        for v in 0..self.fixed_variables.len() {
            if self.fixed_variables[v].is_some() {
                continue;
            }
            let sum: Fractional = self.constraints.iter().map(|c| c.buttons[v]).sum();
            if sum > Fractional::ONE {
                return false;
            }
        }
        true
    }

    fn fix(&mut self, variable: usize, value: Fractional) {
        for r in 0..self.constraints.len() {
            self.constraints[r].fix(variable, value);
        }
        self.solution.fix(variable, value);
        self.fixed_variables[variable] = Some(value);
    }
}

impl SimplexRow {
    fn has_fixed_variable(&self) -> Option<(usize, Fractional)> {
        let set_values = self
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, v)| v != &&Fractional::ZERO)
            .sorted_by_key(|(_, v)| v.invert())
            .map(|(i, _)| i)
            .collect_vec();
        if set_values.len() != 1 {
            None
        } else {
            Some((set_values[0], self.target / self.buttons[set_values[0]]))
        }
    }

    fn value(&self, column: usize) -> Option<Fractional> {
        if self.target == Fractional::ZERO {
            None
        } else {
            let value = self.target / self.buttons[column];
            match value.cmp(&Fractional::ZERO) {
                Ordering::Equal => None,
                Ordering::Greater => Some(value),
                Ordering::Less => None,
            }
        }
    }

    fn normalize(&mut self, column: usize) -> Self {
        let multiple = self.buttons[column];
        if multiple == Fractional::ONE {
            return self.clone();
        }
        for c in 0..self.buttons.len() {
            self.buttons[c] = (self.buttons[c] / multiple).simplify();
        }
        self.target = (self.target / multiple).simplify();
        self.clone()
    }

    fn subtract(&mut self, other: &Self, column: usize) {
        let multiple = self.buttons[column];
        if multiple == Fractional::ZERO {
            return;
        }
        for c in 0..self.buttons.len() {
            self.buttons[c] = (self.buttons[c] - (multiple * other.buttons[c])).simplify();
        }
        self.target = (self.target - (multiple * other.target)).simplify();
    }

    fn fix(&mut self, variable: usize, value: Fractional) {
        if self.buttons[variable] == Fractional::ZERO {
            return;
        }
        self.target = self.target - (self.buttons[variable] * value);
        self.buttons[variable] = Fractional::ZERO;
    }

    fn ensure_not_all_negative(&mut self) {
        if self.target < Fractional::ZERO {
            self.flip();
            return;
        }
        if self.buttons.iter().all(|v| v <= &Fractional::ZERO) && self.target == Fractional::ZERO {
            self.flip();
            return;
        }
    }

    fn flip(&mut self) {
        self.target = self.target.invert();
        for c in 0..self.buttons.len() {
            self.buttons[c] = self.buttons[c].invert();
        }
    }
}

impl Fractional {
    const ONE: Self = Self(1, 1);
    const ZERO: Self = Self(0, 1);
    const MAX: Self = Self(i64::MAX, 1);

    fn new(num: i64) -> Self {
        Self(num, 1)
    }

    fn from_bool(value: bool) -> Self {
        if value {
            Self::ONE
        } else {
            Self::ZERO
        }
    }

    fn simplify(self) -> Self {
        if self.0 == 0 || self.1 == 0 {
            return Fractional::ZERO;
        }
        let sign = if (self.0 < 0) ^ (self.1 < 0) { -1 } else { 1 };
        let num = self.0.abs();
        let den = self.1.abs();
        let gcd = num.gcd(&den);
        Self(sign * (num / gcd), den / gcd)
    }

    fn is_whole(&self) -> bool {
        self.1 == 1
    }

    fn is_positive(&self) -> bool {
        self.0 >= 0
    }

    fn round_up(self) -> Self {
        let mut num = self.0;
        let den = self.1;
        while num % den != 0 {
            num += 1;
        }
        Self(num / den, 1)
    }

    fn invert(self) -> Self {
        Self(self.0 * -1, self.1)
    }
}

impl Sum for Fractional {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Fractional::ZERO;
        for i in iter {
            sum = sum + i;
        }
        sum
    }
}

impl Add for Fractional {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.1 == other.1 {
            Self(self.0 + other.0, self.1).simplify()
        } else {
            Self(self.0 * other.1 + other.0 * self.1, self.1 * other.1).simplify()
        }
    }
}

impl Sub for Fractional {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.1 == other.1 {
            Self(self.0 - other.0, self.1).simplify()
        } else {
            Self(self.0 * other.1 - other.0 * self.1, self.1 * other.1).simplify()
        }
    }
}

impl Mul for Fractional {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1).simplify()
    }
}

impl Div for Fractional {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 * other.1, self.1 * other.0).simplify()
    }
}

impl PartialEq<Self> for Fractional {
    fn eq(&self, other: &Self) -> bool {
        if self.1 == other.1 {
            self.0 == other.0
        } else {
            self.0 * other.1 == other.0 * self.1
        }
    }
}

impl Eq for Fractional {}
impl Ord for Fractional {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            self.0.cmp(&other.0)
        } else if self.0 == i64::MAX {
            Ordering::Greater
        } else if other.0 == i64::MAX {
            Ordering::Less
        } else {
            (self.0 * other.1).cmp(&(other.0 * self.1))
        }
    }
}

impl PartialOrd<Self> for Fractional {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Simplex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        f.write_str("v         ")?;
        for v in 0..self.fixed_variables.len() {
            f.write_fmt(format_args!(" {:?}", Fractional::new(v as i64)))?;
        }
        f.write_char('\n')?;
        for c in &self.constraints {
            f.write_char(' ')?;
            c.fmt(f)?;
        }
        f.write_char('*')?;
        self.solution.fmt(f)?;
        // f.write_str("min:      ")?;
        // for v in 0..self.variables.len() {
        //     f.write_fmt(format_args!(
        //         " {:?}",
        //         self.variables[v].first().unwrap_or(&Fractional::ZERO)
        //     ))?;
        // }
        f.write_str("\nmax:      ")?;
        for v in 0..self.variable_maximums.len() {
            f.write_fmt(format_args!(
                " {:?}",
                Fractional::new(self.variable_maximums[v])
            ))?;
        }
        f.write_fmt(format_args!("\ncount: {:?}\n", self.count()))
    }
}

impl Debug for SimplexRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} |", self.target))?;
        for e in &self.buttons {
            f.write_fmt(format_args!(" {:?}", e))?;
        }
        f.write_char('\n')
    }
}

impl Debug for Fractional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == 1 {
            f.write_fmt(format_args!("{:7}", self.0))
        } else {
            f.write_fmt(format_args!("{:4}/{:2}", self.0, self.1))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn finds_fewest_button_presses_for_light_configuration() {
        let solver = Advent2025Day10Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 7);
    }

    #[test]
    fn finds_fewest_button_presses_for_joltage_configuration() {
        let solver = Advent2025Day10Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 33);
    }
}
