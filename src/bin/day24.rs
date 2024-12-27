use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use util;

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    operation: i32,
    output: String
}

fn main() {
    let mut lines = util::read_lines("day24.txt").into_iter();

    let mut wires: HashMap<String, bool> = HashMap::new();
    // deal with initial wire values
    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        let (name, value) = line.split_once(": ").unwrap();
        let value = value == "1";
        wires.insert(name.to_owned(), value);
    }

    let mut gates: HashMap<String, Gate> = HashMap::new();
    // deal with gates
    while let Some(line) = lines.next() {
        let gate = parse_gate(&line);
        gates.insert(gate.output.to_owned(), gate);
    } 

    println!("Part 1: {}", part_1(&mut wires.clone(), &gates));
    println!("Part 2: {}", part_2(&gates).join(","));
}

fn part_1(wires: &mut HashMap<String, bool>, gates: &HashMap<String, Gate>) -> i64 {
    // by the end all wires starting with z need to have values
    let bits: Vec<&String> = gates.keys().filter(|it| it[0..1] == *"z").collect();
    for bit in &bits {
        process_gate(&gates[*bit], wires, gates);
    }
   
    // now sort the bits and convert to a decimal number
    let mut number: Vec<&str> = vec![];
    for bit in bits.into_iter().sorted().rev() {
        if wires[bit] { number.push("1"); } else { number.push("0"); }
    }
    i64::from_str_radix(&number.join(""), 2).unwrap()
}

fn part_2(gates: &HashMap<String, Gate>) -> Vec<String> {
    let mut suspicious_gates: HashSet<String> = HashSet::new();

    // because this is a ripple adder we can check a few scenarios
    for gate in gates.values() {
        if gate.output[0..1] == *"z" && gate.output != "z00" && gate.output != "z45" {
            // this is an output bit and not the first or last one
            // it must be an xor operation
            if gate.operation != 2 { 
                suspicious_gates.insert(gate.output.clone()); 
            }
        } 

        if gate.operation == 2 && gate.input1[0..1] != *"x" && gate.input2[0..1] != *"x" && gate.output != "z01" {
            // this is an "intermediate" and, make sure this outputs a final "z" bit
            if gate.output[0..1] != *"z" {
                suspicious_gates.insert(gate.output.clone());
            }
            // make sure no inputs come from and operations
            if gates[&gate.input1].operation == 0 {
                suspicious_gates.insert(gate.input1.clone());
            }
            if gates[&gate.input2].operation == 0 {
                suspicious_gates.insert(gate.input2.clone());
            }
        }

        if gate.operation == 1 {
            // this is an or for the carry bit, make sure no inputs come from xor operations
            if gates[&gate.input1].operation == 2 {
                suspicious_gates.insert(gate.input1.clone());
            }
            if gates[&gate.input2].operation == 2 {
                suspicious_gates.insert(gate.input2.clone());
            }
        }
    }
    
    suspicious_gates.into_iter().sorted().collect()
}

fn process_gate(gate: &Gate, wires: &mut HashMap<String, bool>, gates: &HashMap<String, Gate>) {
    if !wires.contains_key(&gate.input1) {
        // no value, recursively sovle for the gate that outputs to this wire
        process_gate(&gates[&gate.input1], wires, gates);
    }
    if !wires.contains_key(&gate.input2) {
        // no value, recursively sovle for the gate that outputs to this wire
        process_gate(&gates[&gate.input2], wires, gates);
    }

    // apply gate operation and save to output
    let result = match gate.operation {
        0 => wires[&gate.input1] & wires[&gate.input2],
        1 => wires[&gate.input1] | wires[&gate.input2],
        2 => wires[&gate.input1] ^ wires[&gate.input2],
        _ => unreachable!()
    };
    wires.entry(gate.output.to_owned()).and_modify(|it| *it = result).or_insert(result);
}

fn parse_gate(line: &str) -> Gate {
    let segments: Vec<&str> = line.split(" ").collect();
    let operation = match segments[1] {
        "AND" => 0,
        "OR" => 1,
        "XOR" => 2,
        _ => unreachable!()
    };
    Gate { input1: segments[0].to_owned(), input2: segments[2].to_owned(), operation, output: segments[4].to_owned() }
}
