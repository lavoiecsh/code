use crate::solver::AdventSolver;
use itertools::Itertools;

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
            .map(Machine::fewest_joltage_requirements_button_presses)
            .sum()
    }
}

struct Machine {
    lights: IndicatorLights,
    buttons: Vec<Button>,
    joltages: JoltageRequirements,
}

struct IndicatorLights(Vec<bool>);
struct ButtonPresses {
    max_presses: Vec<usize>,
    current_presses: Vec<usize>,
}
struct Button(Vec<usize>);
struct ButtonPressCounts(Vec<usize>);
struct JoltageRequirements(Vec<usize>);

impl Iterator for ButtonPresses {
    type Item = ButtonPressCounts;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        self.current_presses[i] += 1;
        while self.current_presses[i] > self.max_presses[i] {
            self.current_presses[i] = 0;
            i += 1;
            if i == self.current_presses.len() {
                return None;
            }
            self.current_presses[i] += 1;
        }
        Some(ButtonPressCounts(self.current_presses.clone()))
    }
}

impl ButtonPresses {
    fn new(max_presses: Vec<usize>) -> Self {
       Self { current_presses: vec![0; max_presses.len()], max_presses }
    }
}

impl Machine {
    fn from(input: &str) -> Self {
        let split = input.split(' ').collect_vec();
        let indicator_lights = remove_wrap(split[0])
            .chars()
            .map(|c| c == '#')
            .collect();
        let button_wirings = split[1..split.len() - 1]
            .iter()
            .map(|w| remove_wrap(w).split(',').map(|s| s.parse::<usize>().unwrap()).collect())
            .map(Button)
            .collect();
        let joltage_requirements = remove_wrap(split[split.len() - 1])
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Self {
            lights: IndicatorLights(indicator_lights),
            buttons: button_wirings,
            joltages: JoltageRequirements(joltage_requirements),
        }
    }

    fn fewest_indicator_lights_button_presses(&self) -> usize {
        (1..self.buttons.len())
            .find(|k| self.buttons.iter()
                .combinations(*k)
                .any(|c| self.lights.obtainable_from(&c)))
            .unwrap()
    }

    fn fewest_joltage_requirements_button_presses(&self) -> usize {
        self.joltages.least_button_presses(&self.buttons)
    }
}

impl IndicatorLights {
    fn init(&self) -> Self {
        Self(vec![false; self.0.len()])
    }

    fn obtainable_from(&self, buttons: &[&Button]) -> bool {
        let mut test = self.init();
        buttons.iter().for_each(|b| b.apply_lights(&mut test));
        test.0 == self.0
    }
}

impl JoltageRequirements {
    fn init(&self) -> Self {
        Self(vec![0; self.0.len()])
    }

    fn least_button_presses(&self, buttons: &[Button]) -> usize {
        let max_button_presses = buttons.iter()
            .map(|b| self.max_button_presses(b))
            .collect_vec();
        ButtonPresses::new(max_button_presses)
            .filter(|b| b.fits(buttons, &self))
            .map(|c| c.count())
            .min()
            .unwrap_or(0)
    }

    fn max_button_presses(&self, button: &Button) -> usize {
        let mut test = self.init();
        let mut count = 0;
        while self.is_valid(&test) {
            button.apply_joltage(&mut test);
            count += 1;
        }
        count - 1
    }

    fn is_valid(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a >= b)
    }
}

impl Button {
    fn apply_lights(&self, lights: &mut IndicatorLights) {
        self.0.iter().for_each(|&i| lights.0[i] = !lights.0[i]);
    }

    fn apply_joltage_n(&self, n: usize, joltages: &mut JoltageRequirements) {
        self.0.iter().for_each(|&i| joltages.0[i] += n);
    }

    fn apply_joltage(&self, joltages: &mut JoltageRequirements) {
        self.0.iter().for_each(|&i| joltages.0[i] += 1);
    }
}

impl ButtonPressCounts {
    fn count(&self) -> usize {
        self.0.iter().sum()
    }

    fn fits(&self, buttons: &[Button], joltages: &JoltageRequirements) -> bool {
        let mut test = joltages.init();
        self.0.iter()
            .enumerate()
            .for_each(|(i, c)| buttons[i].apply_joltage_n(*c, &mut test));
        test.0 == joltages.0
    }
}

fn remove_wrap(input: &str) -> &str {
    input
        .strip_prefix(['[', '(', '{']).unwrap()
        .strip_suffix([']', ')', '}']).unwrap()
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
        let m = &solver.machines[0];
        assert_eq!(m.joltages.max_button_presses(&m.buttons[0]), 7);
        assert_eq!(m.joltages.max_button_presses(&m.buttons[1]), 5);
        assert_eq!(m.joltages.max_button_presses(&m.buttons[2]), 4);
        assert_eq!(m.joltages.max_button_presses(&m.buttons[3]), 4);
        assert_eq!(m.joltages.max_button_presses(&m.buttons[4]), 3);
        assert_eq!(m.joltages.max_button_presses(&m.buttons[5]), 3);
        assert_eq!(m.fewest_joltage_requirements_button_presses(), 10);
        assert_eq!(solver.solve_part2(), 33);
    }
}
