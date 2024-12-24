use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gate {
    And,
    Or,
    Xor,
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Gate::And),
            "OR" => Ok(Gate::Or),
            "XOR" => Ok(Gate::Xor),
            _ => Err(format!("Unknown gate type: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Option<bool>>,
    gates: Vec<(String, String, Gate, String)>, // (input1, input2, gate_type, output)
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            wires: HashMap::new(),
            gates: Vec::new(),
        }
    }

    fn add_initial_wire(&mut self, name: &str, value: bool) {
        self.wires.insert(name.to_string(), Some(value));
    }

    fn add_gate(&mut self, input1: &str, input2: &str, gate: Gate, output: &str) {
        self.gates.push((
            input1.to_string(),
            input2.to_string(),
            gate,
            output.to_string(),
        ));
        // Initialize output wire if it doesn't exist
        self.wires.entry(output.to_string()).or_insert(None);
        // Initialize input wires if they don't exist
        self.wires.entry(input1.to_string()).or_insert(None);
        self.wires.entry(input2.to_string()).or_insert(None);
    }

    fn evaluate_gate(&self, input1: bool, input2: bool, gate: Gate) -> bool {
        match gate {
            Gate::And => input1 && input2,
            Gate::Or => input1 || input2,
            Gate::Xor => input1 ^ input2,
        }
    }

    fn simulate(&mut self) {
        let mut changed = true;
        let mut evaluated = HashSet::new();

        while changed {
            changed = false;
            for (input1, input2, gate, output) in self.gates.clone() {
                if evaluated.contains(&output) {
                    continue;
                }

                if let (Some(val1), Some(val2)) = (self.wires[&input1], self.wires[&input2]) {
                    let result = self.evaluate_gate(val1, val2, gate);
                    self.wires.insert(output.clone(), Some(result));
                    evaluated.insert(output);
                    changed = true;
                }
            }
        }
    }

    fn get_output(&self) -> u64 {
        let mut z_wires: Vec<_> = self
            .wires
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .collect();

        // Sort by the number after 'z' to get correct bit order
        z_wires.sort_by_key(|(k, _)| k[1..].parse::<u32>().unwrap_or(0));
        dbg!(&z_wires);

        let mut result = 0;
        for (_, &value) in z_wires.iter().rev() {
            result = (result << 1) | (value.unwrap_or(false) as u64);
        }
        result
    }
}

fn parse_input(input: &str) -> Circuit {
    let mut circuit = Circuit::new();
    let mut parsing_initial = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            parsing_initial = false;
            continue;
        }

        if parsing_initial {
            // Parse initial wire values
            if let Some((wire, value)) = line.split_once(": ") {
                circuit.add_initial_wire(wire, value == "1");
            }
        } else {
            // Parse gate definitions
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 5 && parts[3] == "->" {
                let input1 = parts[0];
                let gate = Gate::from_str(parts[1]).unwrap();
                let input2 = parts[2];
                let output = parts[4];
                circuit.add_gate(input1, input2, gate, output);
            }
        }
    }
    dbg!(&circuit);

    circuit
}
#[test]
fn large_example() {
    let input = r#"x00: 1
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
tnw OR pbm -> gnj"#;

    let mut circuit = parse_input(input);
    circuit.simulate();
    let result = circuit.get_output();
    println!("Output: {}", result);
}
#[test]
fn test_small() {
    let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    let mut circuit = parse_input(input);
    circuit.simulate();
    let result = circuit.get_output();
    println!("Output: {}", result);
    assert_eq!(result, 0b100);
}
#[test]
fn main() {
    let input = include_str!("d24.in");
    let mut circuit = parse_input(input);
    circuit.simulate();
    let result = circuit.get_output();
    println!("Output: {}", result);
}
