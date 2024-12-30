use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};
use itertools::Itertools;
use measure_time_macro::measure_time;

#[derive(Clone, PartialEq)]
enum Operation {
    AND,
    OR,
    XOR
}

struct Device {
    wires: HashMap<String, i8>,
    gates: HashMap<String, (Operation, [String; 2])>,
}

impl Device {

    fn add_wire(&mut self, name: String, initial_value: i8) -> () {
        self.wires.insert(name, initial_value);
    }

    fn add_gate(&mut self, name: String, op: Operation, inputs: [String; 2]) -> () {
        self.gates.insert(name, (op, inputs));
    }

    fn calculates_output_gates(&self) -> Vec<i8> {
        let mut gates_values: HashMap<String, i8> = HashMap::new();
        let mut output_gates: HashMap<String, Option<i8>> = self.gates
            .keys()
            .filter(|key| key.starts_with('z'))
            .map(|key| (key.clone(), None))
            .collect();

        loop {
            // logic goes here and brrrrrr
            for gate in self.gates.clone().into_iter() {
                match (
                    self.get_wire_or_gate_value(&gate.1.1[0].clone(), gates_values.clone()),
                    self.get_wire_or_gate_value(&gate.1.1[1].clone(), gates_values.clone()),
                ) {
                    (Some(in1), Some(in2)) => {
                        let value = calculate_value(in1, in2, gate.1.0);
                        gates_values.insert(gate.0.clone(), value);
                        if gate.0.starts_with('z') { output_gates.insert(gate.0.clone(), Some(value));}
                    }
                    _ => {
                        println!("One or both values are missing");
                    }
                }
            }

            if output_gates.iter().all(|(_, val)| val.is_some()) { break; }
        }

        output_gates.iter()
            .sorted_by_key(|(key, _)| *key)
            .map(|(_, val)| val.unwrap_or(0))
            .collect()
    }

    fn get_wire_or_gate_value(&self, name: &String, gates: HashMap<String, i8>) -> Option<i8> {
        if self.wires.contains_key(name) {
            return self.wires.get(name).cloned();
        }

        gates.get(name).cloned()
    }

}

fn calculate_value(in1: i8, in2: i8, operation: Operation) -> i8 {
    match operation {
        Operation::AND => { in1.bitand(in2) }
        Operation::OR => { in1.bitor(in2) }
        Operation::XOR => { in1.bitxor(in2) }
    }
}


fn change_to_binary_number(binary_number: Vec<i8>) -> isize {
    let src: String = binary_number.iter().map(|num| num.to_string()).collect();
    let reversed: String = src.chars().rev().collect();
    isize::from_str_radix(&reversed, 2).unwrap()
}

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
pub fn part_one(input: &str) -> isize {
    let result = parse_input(input).calculates_output_gates();

    println!("Day 24, part 1 result: {:?}", change_to_binary_number(result.clone()));

    change_to_binary_number(result)
}

fn parse_input(input: &str) -> Device {

    let mut parts = input.trim().split("\n\n");
    let raw_wires = parts.next().unwrap();
    let raw_gates = parts.next().unwrap();

    let wires = raw_wires.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap().trim();
            (key, value.parse::<i8>().expect("Value needs to be binary number"))
        })
        .collect();

    let gates = raw_gates.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split("->");
            let first_part = parts.next().unwrap().to_string();
            let gate_name = parts.next().unwrap().trim().to_string();

            let mut gate_input = first_part.split(" ");
            let in1 = gate_input.next().unwrap().to_string();
            let operation = gate_input.next().unwrap();
            let in2 = gate_input.next().unwrap().to_string();

            let op = match operation {
                "AND" => Operation::AND,
                "OR" => Operation::OR,
                "XOR" => Operation::XOR,
                _ => panic!("Unexpected value: {}", operation),
            };
            (gate_name, (op, [in1, in2]))
        })
        .collect();

    Device {wires, gates}
}

#[measure_time]
pub fn part_two(input: &str) -> String {
    let mut bad = vec![];

    let device = parse_input(input);
    for (output, (op, ins)) in device.gates.iter() {

        let mut ins = ins.clone();
        ins.sort();
        let [in0, in1] = ins;

        if output.starts_with("z") && !output.ends_with("45") {
            if *op != Operation::XOR {
                bad.push(output.clone());
            }
        } else if !(in0.starts_with("x") || in1.starts_with("y")) {
            if *op == Operation::XOR {
                bad.push(output.clone());
            }
        } else if in0.starts_with("x") && in1.starts_with("y")
            || in0.starts_with("y") && in1.starts_with("x")
        {
            if in0.ends_with("00") || in1.ends_with("00") {
                continue;
            }

            let mut ops = vec![];

            for (_, (opb, ins_l2)) in device.gates.iter() {
                if ins_l2.contains(output) {
                    ops.push(opb);
                }
            }

            if *op == Operation::XOR && !ops.contains(&&Operation::XOR)
                || *op == Operation::AND && !ops.contains(&&Operation::OR)
            {
                bad.push(output.clone());
            }
        }
    }

    bad.sort();
    let result = bad.iter().join(",");

    println!("Day 24, part 2 result: {:?}", result);

    result
}

#[cfg(test)]
mod day24 {
    use crate::day24::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "x00: 1
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
tnw OR pbm -> gnj";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), 2024)
    }

}
