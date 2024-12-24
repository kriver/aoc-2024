use std::collections::{HashMap, HashSet};

use crate::util::load;

#[derive(Debug)]
pub enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub struct Gate {
    gate_type: GateType,
    inputs: (String, String),
    output: String,
}

type Wires = HashMap<String, bool>;
type Input = (Wires, HashSet<String>, HashMap<String, Gate>);

impl Gate {
    /// Returns true if evaluation was succesful
    fn eval(&self, wires: &mut Wires) -> bool {
        if let Some(a) = wires.get(&self.inputs.0) {
            if let Some(b) = wires.get(&self.inputs.1) {
                let c = match self.gate_type {
                    GateType::And => a & b,
                    GateType::Or => a | b,
                    GateType::Xor => a ^ b,
                };
                wires.insert(self.output.clone(), c);
                return false;
            }
        }
        true
    }
}

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day24.txt");
    let mut wires = HashMap::new();
    let mut no_value = HashSet::new();
    let mut gates = Vec::new();
    let mut parsing_gates = false;
    for line in lines {
        if line.is_empty() {
            parsing_gates = true;
            continue;
        }
        if parsing_gates {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            let gate_type = match tokens[1] {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("Unknown gate type"),
            };
            let (a, b, c) = (
                tokens[0].to_string(),
                tokens[2].to_string(),
                tokens[4].to_string(),
            );
            if !wires.contains_key(&a) {
                no_value.insert(a.clone());
            }
            if !wires.contains_key(&b) {
                no_value.insert(b.clone());
            }
            if !wires.contains_key(&c) {
                no_value.insert(c.clone());
            }
            gates.push(Gate {
                gate_type,
                inputs: (a, b),
                output: c,
            });
        } else {
            let mut token_it = line.split(": ");
            let wire = token_it.next().unwrap().to_string();
            let value = token_it.next().unwrap().parse::<u8>().unwrap() == 1;
            wires.insert(wire, value);
        }
    }
    (
        wires,
        no_value,
        gates.into_iter().map(|g| (g.output.clone(), g)).collect(),
    )
}

pub fn part1((mut wires, mut no_value, gates): Input) -> u64 {
    loop {
        if no_value.is_empty() {
            break;
        }
        no_value.retain(|nv| {
            let g = gates.get(nv).unwrap();
            g.eval(&mut wires)
        });
    }
    let mut n = 0;
    for i in 0..=45 {
        let bit = if *wires.get(&format!("z{:02}", i)).unwrap() {
            1
        } else {
            0
        };
        n |= bit << i;
    }
    n
}

pub fn part2(values: Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 57588078076750);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}
