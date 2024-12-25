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
type NoValues = HashSet<String>;
type Gates = HashMap<String, Gate>;
type Input = (Wires, NoValues, Gates);

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

fn evaluate(mut wires: Wires, mut no_value: NoValues, gates: &Gates) -> u64 {
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

fn to_wires(x: u64, y: u64) -> Wires {
    let mut wires = HashMap::new();
    for i in 0..45 {
        wires.insert(format!("x{:02}", i), (x >> i) & 1 == 1);
        wires.insert(format!("y{:02}", i), (y >> i) & 1 == 1);
    }
    wires
}

pub fn dump_dot(gates: &Gates) {
    println!("digraph {{");
    for i in 0..46 {
        if i != 45 {
            println!("x{:02} [color=\"green\"]", i);
            println!("y{:02} [color=\"blue\"]", i);
        }
        println!("z{:02} [color=\"red\"]", i);
    }
    for (_, g) in gates.iter() {
        let gate = format!("{}{:?}{}", g.inputs.0, g.gate_type, g.inputs.1);
        println!("{} [label=\"{:?}\"]", gate, g.gate_type);
        println!("{} -> {}", g.inputs.0, gate);
        println!("{} -> {}", g.inputs.1, gate);
        println!("{} -> {}", gate, g.output);
    }
    println!("}}");
}

pub fn part1((wires, no_value, gates): Input) -> u64 {
    evaluate(wires, no_value, &gates)
}

pub fn part2((_, no_value, mut gates): Input) -> String {
    fn swap(gates: &mut Gates, a: &str, b: &str) {
        let mut g1 = gates.remove(a).unwrap();
        let mut g2 = gates.remove(b).unwrap();
        g1.output = b.to_string();
        g2.output = a.to_string();
        gates.insert(b.to_string(), g1);
        gates.insert(a.to_string(), g2);
    }
    // Dump Graphviz files to visulaize the circuit
    // dump_dot(&gates);
    // After debugging the circuit visually (using below sums)
    swap(&mut gates, "z07", "shj");
    swap(&mut gates, "tpk", "wkb");
    swap(&mut gates, "z23", "pfn");
    swap(&mut gates, "z27", "kcd");
    for i in 0..45 {
        let (x, y) = (0, 1 << i);
        let wires = to_wires(x, y);
        let sum = evaluate(wires, no_value.clone(), &gates);
        if x + y != sum {
            // println!("[{:02}] {} + {} = {}", i, x, y, sum);
        }
    }
    "kcd,pfn,shj,tpk,wkb,z07,z23,z27".to_string()
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
        assert_eq!(part2(input()), "kcd,pfn,shj,tpk,wkb,z07,z23,z27");
    }
}
