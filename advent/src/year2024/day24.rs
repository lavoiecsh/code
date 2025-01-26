use crate::solver::AdventSolver;
use itertools::Itertools;
use regex::{Match, Regex};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub struct Advent2024Day24Solver {
    logic_circuit: LogicCircuit,
}

impl Advent2024Day24Solver {
    pub fn new(input: &str) -> Self {
        let gate_re = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        let mut lines = input.lines();
        let x = lines
            .take_while_ref(|l| l.starts_with("x"))
            .map(|l| l.split(' ').nth(1).unwrap() == "1")
            .collect();
        let y = lines
            .take_while_ref(|l| l.starts_with("y"))
            .map(|l| l.split(' ').nth(1).unwrap() == "1")
            .collect();
        let gates = lines
            .filter_map(|l| gate_re.captures(l))
            .map(|c| {
                Gate::new(
                    Wire::from(c.get(1)),
                    GateOperation::from(c.get(2)),
                    Wire::from(c.get(3)),
                    Wire::from(c.get(4)),
                )
            })
            .collect();
        Self {
            logic_circuit: LogicCircuit::new(x, y, gates),
        }
    }
}

impl AdventSolver for Advent2024Day24Solver {
    fn solve_part1(&self) -> usize {
        self.logic_circuit.clone().resolve().unwrap()
    }

    fn solve_part2_string(&self) -> String {
        self.logic_circuit.find_swap().name()
    }
}

#[derive(Clone)]
struct LogicCircuit {
    x: Vec<bool>,
    y: Vec<bool>,
    largest_z: usize,
    gates: HashMap<Wire, Gate>,
}

#[derive(Clone)]
struct Gate {
    a: Wire,
    b: Wire,
    result: Wire,
    operation: GateOperation,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Wire {
    X(usize),
    Y(usize),
    Z(usize),
    Other(String),
}

#[derive(Clone)]
enum GateOperation {
    And,
    Or,
    Xor,
}

struct LogicCircuitSolver<'a> {
    logic_circuit: &'a LogicCircuit,
    z: Vec<bool>,
    values: HashMap<String, bool>,
    swaps: &'a Swaps,
}

#[derive(Clone)]
struct Swaps {
    pairs: Vec<(Wire, Wire)>,
}

impl Swaps {
    fn new(pairs: Vec<(&Wire, &Wire)>) -> Self {
        Self {
            pairs: pairs.iter().map(|&(a, b)| (a.clone(), b.clone())).collect(),
        }
    }

    fn resolve_wire(&self, wire: &Wire) -> Wire {
        self.pairs
            .iter()
            .filter_map(|(a, b)| {
                if wire == a {
                    Some(b.clone())
                } else if wire == b {
                    Some(a.clone())
                } else {
                    None
                }
            })
            .next()
            .unwrap_or_else(|| wire.clone())
    }

    fn name(&self) -> String {
        self.pairs
            .iter()
            .flat_map(|(a, b)| [a, b])
            .collect_vec()
            .name()
    }
}

impl PartialEq for Swaps {
    fn eq(&self, other: &Self) -> bool {
        self.pairs.len() == other.pairs.len()
            && self.pairs.iter().all(|sp| {
                other
                    .pairs
                    .iter()
                    .any(|op| (sp.0 == op.0 && sp.1 == op.1) || (sp.0 == op.1 && sp.1 == op.0))
            })
    }
}

impl<'a> LogicCircuitSolver<'a> {
    fn new(logic_circuit: &'a LogicCircuit, swaps: &'a Swaps) -> Self {
        Self {
            logic_circuit,
            swaps,
            z: vec![false; logic_circuit.largest_z],
            values: HashMap::new(),
        }
    }

    fn solve(&mut self) -> Option<usize> {
        let mut gates = self.logic_circuit.gates.values().collect_vec();
        while !gates.is_empty() {
            let unsolved = gates
                .iter()
                .filter(|gate| !self.solve_gate(gate))
                .cloned()
                .collect_vec();
            if unsolved.len() == gates.len() {
                return None;
            }
            gates = unsolved;
        }
        Some(self.z.value())
    }

    fn solve_gate(&mut self, gate: &Gate) -> bool {
        if let Some(a) = self.get(&gate.a) {
            if let Some(b) = self.get(&gate.b) {
                self.set(&gate.result, gate.solve(a, b));
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get(&self, wire: &Wire) -> Option<bool> {
        match wire {
            Wire::X(i) => Some(self.logic_circuit.x[*i]),
            Wire::Y(i) => Some(self.logic_circuit.y[*i]),
            Wire::Z(i) => Some(self.z[*i]),
            Wire::Other(s) => self.values.get(s).copied(),
        }
    }

    fn set(&mut self, wire: &Wire, value: bool) {
        let resolved_wire = self.swaps.resolve_wire(wire);

        match resolved_wire {
            Wire::X(_) => unreachable!("should not set x"),
            Wire::Y(_) => unreachable!("should not set y"),
            Wire::Z(i) => self.z[i] = value,
            Wire::Other(s) => {
                self.values.insert(s.clone(), value);
            }
        }
    }
}

impl LogicCircuit {
    fn new(x: Vec<bool>, y: Vec<bool>, gates: Vec<Gate>) -> Self {
        Self {
            largest_z: gates
                .iter()
                .filter_map(|g| g.result.z_index())
                .max()
                .unwrap()
                + 1,
            x,
            y,
            gates: gates.into_iter().map(|g| (g.result.clone(), g)).collect(),
        }
    }

    fn resolve(&self) -> Option<usize> {
        LogicCircuitSolver::new(self, &Swaps::new(vec![])).solve()
    }

    fn find_swap(&self) -> Swaps {
        let x = self.x.value();
        let y = self.y.value();
        let expected_z = x + y;

        let mut last_error = None;
        let mut swaps = Swaps::new(vec![]);
        for i in 2..self.largest_z-1 {
            if let Some(error) = self.is_adder(i) {
                if let Some(last) = last_error.clone() {
                    swaps.pairs.push((error, last));
                    last_error = None;
                } else {
                    last_error = Some(error);
                }
            }
        }

        if let Some(new_z) = LogicCircuitSolver::new(self, &swaps).solve() {
            if new_z == expected_z {
                return swaps;
            }
        }
        unreachable!("solution not found");
    }

    fn is_adder(&self, z_index: usize) -> Option<Wire> {
        let z_wire = Wire::Z(z_index);
        let gate = self.gates.get(&z_wire).unwrap();
        if !gate.is_xor() || gate.is_xy() {
            Some(z_wire)
        } else {
            match (
                self.is_adder_a(z_index, &gate.a),
                self.is_adder_a(z_index, &gate.b),
                self.is_adder_b(z_index, &gate.a),
                self.is_adder_b(z_index, &gate.b),
            ) {
                (None, _, _, None) => None,
                (_, None, None, _) => None,
                (None, _, _, Some(e)) => Some(e),
                (Some(e), _, _, None) => Some(e),
                (_, None, Some(e), _) => Some(e),
                (_, Some(e), None, _) => Some(e),
                (a_is_a, b_is_a, a_is_b, b_is_b) => {
                    unreachable!("unknown pattern", &gate, a_is_a, b_is_a, a_is_b, b_is_b);
                }
            }
        }
    }

    fn is_adder_a(&self, z_index: usize, wire: &Wire) -> Option<Wire> {
        let gate = self.gates.get(wire).unwrap();
        if let Some(xy_index) = gate.is_xy_xor() {
            if xy_index != z_index {
                Some(wire.clone())
            } else {
                None
            }
        } else {
            Some(wire.clone())
        }
    }

    fn is_adder_b(&self, z_index: usize, wire: &Wire) -> Option<Wire> {
        let gate = self.gates.get(wire).unwrap();
        // dbg!(z_index, &gate);
        if gate.is_xy() || !gate.is_or() {
            Some(wire.clone())
        } else {
            match (
                self.is_adder_c(z_index, &gate.a),
                self.is_adder_c(z_index, &gate.b),
                self.is_adder_d(z_index, &gate.a),
                self.is_adder_d(z_index, &gate.b),
            ) {
                (None, _, _, None) => None,
                (_, None, None, _) => None,
                (None, _, _, Some(e)) => Some(e),
                (Some(e), _, _, None) => Some(e),
                (_, None, Some(e), _) => Some(e),
                (_, Some(e), None, _) => Some(e),
                (a_is_c, b_is_c, a_is_d, b_is_d) => {
                    unreachable!("unknown pattern", &gate, a_is_c, b_is_c, a_is_d, b_is_d);
                }
            }
        }
    }

    fn is_adder_c(&self, z_index: usize, wire: &Wire) -> Option<Wire> {
        let gate = self.gates.get(wire).unwrap();
        if let Some(xy_index) = gate.is_xy_and() {
            if xy_index != z_index - 1 {
                Some(wire.clone())
            } else {
                None
            }
        } else {
            Some(wire.clone())
        }
    }

    fn is_adder_d(&self, _z_index: usize, wire: &Wire) -> Option<Wire> {
        let gate = self.gates.get(wire).unwrap();
        if !gate.is_and() {
            Some(wire.clone())
        } else {
            None
        }
    }
}

trait Digits {
    fn value(&self) -> usize;
}

impl Digits for Vec<bool> {
    fn value(&self) -> usize {
        self.iter()
            .rev()
            .fold(0, |acc, b| acc * 2 + if *b { 1 } else { 0 })
    }
}

trait SwappedWires {
    fn name(&self) -> String;
}

impl SwappedWires for Vec<&Wire> {
    fn name(&self) -> String {
        self.iter().map(|w| w.name()).sorted().join(",")
    }
}

impl SwappedWires for Vec<Wire> {
    fn name(&self) -> String {
        self.iter().map(|w| w.name()).sorted().join(",")
    }
}

impl Gate {
    fn new(a: Wire, operation: GateOperation, b: Wire, result: Wire) -> Self {
        Self {
            a,
            operation,
            b,
            result,
        }
    }

    fn solve(&self, a: bool, b: bool) -> bool {
        match self.operation {
            GateOperation::And => a && b,
            GateOperation::Or => a || b,
            GateOperation::Xor => a ^ b,
        }
    }

    fn is_xy(&self) -> bool {
        matches!(self.a, Wire::X(_) | Wire::Y(_)) && matches!(self.b, Wire::X(_) | Wire::Y(_))
    }

    fn is_and(&self) -> bool {
        matches!(self.operation, GateOperation::And)
    }

    fn is_xor(&self) -> bool {
        matches!(self.operation, GateOperation::Xor)
    }

    fn is_or(&self) -> bool {
        matches!(self.operation, GateOperation::Or)
    }

    fn is_xy_xor(&self) -> Option<usize> {
        match (&self.operation, &self.a, &self.b) {
            (GateOperation::Or, _, _) | (GateOperation::And, _, _) => None,
            (_, Wire::X(i), Wire::Y(_)) => Some(*i),
            (_, Wire::Y(i), Wire::X(_)) => Some(*i),
            _ => None,
        }
    }

    fn is_xy_and(&self) -> Option<usize> {
        match (&self.operation, &self.a, &self.b) {
            (GateOperation::Or, _, _) | (GateOperation::Xor, _, _) => None,
            (_, Wire::X(i), Wire::Y(_)) => Some(*i),
            (_, Wire::Y(i), Wire::X(_)) => Some(*i),
            _ => None,
        }
    }
}

impl Wire {
    fn z_index(&self) -> Option<usize> {
        if let Wire::Z(index) = self {
            Some(*index)
        } else {
            None
        }
    }

    fn name(&self) -> String {
        match self {
            Wire::X(i) => format!("x{:02}", i),
            Wire::Y(i) => format!("y{:02}", i),
            Wire::Z(i) => format!("z{:02}", i),
            Wire::Other(s) => s.clone(),
        }
    }
}

impl From<Option<Match<'_>>> for Wire {
    fn from(value: Option<Match<'_>>) -> Self {
        let str = String::from(value.unwrap().as_str());
        let s = str.split_at(1);
        match s.0 {
            "x" => {
                if let Ok(i) = s.1.parse::<usize>() {
                    Wire::X(i)
                } else {
                    Wire::Other(str)
                }
            }
            "y" => {
                if let Ok(i) = s.1.parse::<usize>() {
                    Wire::Y(i)
                } else {
                    Wire::Other(str)
                }
            }
            "z" => {
                if let Ok(i) = s.1.parse::<usize>() {
                    Wire::Z(i)
                } else {
                    Wire::Other(str)
                }
            }
            _ => Wire::Other(str),
        }
    }
}

impl From<Option<Match<'_>>> for GateOperation {
    fn from(value: Option<Match<'_>>) -> Self {
        match value.unwrap().as_str() {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _s => unreachable!("invalid operation {_s}"),
        }
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

impl Debug for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} = {:?} {:?} {:?}",
            self.result, self.a, self.operation, self.b
        ))
    }
}

impl Debug for GateOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::And => "AND",
            Self::Or => " OR",
            Self::Xor => "XOR",
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_1: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
    static EXAMPLE_2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn computes_output_as_decimal_number() {
        assert_eq!(Advent2024Day24Solver::new(EXAMPLE_1).solve_part1(), 4);
        assert_eq!(Advent2024Day24Solver::new(EXAMPLE_2).solve_part1(), 2024);
    }
}
