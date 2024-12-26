use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::BitXor;
use itertools::Itertools;
use measure_time_macro::measure_time;
use regex::Regex;

#[derive(Debug, Clone)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: i128,
    register_b: i128,
    register_c: i128,
    program: Vec<i8>,
    instruction_pointer: usize,
}

impl Computer {
    fn combo_operand(&self, operand: i8) -> i128 {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            oper => oper as i128
        }
    }

    fn adv(&mut self, operand: i8) -> () {
        self.register_a = (self.register_a as f64 / 2_i32.pow(self.combo_operand(operand) as u32) as f64).floor() as i128
    }

    fn bxl(&mut self, operand: i8) -> () {
        self.register_b = (self.register_b as i8).bitxor(operand) as i128
    }

    fn bst(&mut self, operand: i8) -> () {
        self.register_b = self.combo_operand(operand) & 7
    }

    fn jnz(&mut self, operand: i8) -> () {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize
        }
    }

    fn bxc(&mut self, _operand: i8) -> () {
        self.register_b = self.register_b.bitxor(self.register_c)
    }

    fn out(&mut self, operand: i8) -> i128 {
        self.combo_operand(operand) & 7
    }

    fn bdv(&mut self, operand: i8) -> () {
        self.register_b = (self.register_a as f64 / 2_i32.pow(self.combo_operand(operand) as u32) as f64).floor() as i128
    }

    fn cdv(&mut self, operand: i8) -> () {
        self.register_c = (self.register_a as f64 / 2_i32.pow(self.combo_operand(operand) as u32) as f64).floor() as i128
    }

    pub fn reset(&mut self, register_a: i128) -> () {
        self.register_a = register_a;
        self.instruction_pointer = 0;
    }

    pub fn run_program(&mut self) -> String {
        let mut result: Vec<i128> = Vec::new();
        loop {
            if self.instruction_pointer >= self.program.len() {
                return result
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
            }

            let [opcode, operand] = [self.program[self.instruction_pointer], self.program[self.instruction_pointer + 1]];

            println!("{:?}, RES: {:?}, Program TICK: instr: {}, opcode: {}, operand: {}, registers: [{}, {}, {}] ", self.program, result, self.instruction_pointer, opcode, operand, self.register_a, self.register_b, self.register_c );

            self.instruction_pointer += 2;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => {
                    let output = self.out(operand);
                    result.push(output);
                }
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!(),
            }
        }
    }
}

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
    // 48744869 low
}

#[measure_time]
pub fn part_one(input: &str) -> String {
    let mut computer: Computer = parse_input(input);
    let result: String = computer.run_program();

    println!("Day 17, part 1 result: {:?}", result.clone());

    result
}

fn parse_input(input: &str) -> Computer {
    let re = Regex::new(r"Register A: (?P<register_a>\d+)
Register B: (?P<register_b>\d+)
Register C: (?P<register_c>\d+)

Program: (?P<program>[\d,]+)").unwrap();

    if let Some(captures) = re.captures(input) {
        let register_a = captures.name("register_a").unwrap().as_str().parse::<i128>().unwrap();
        let register_b = captures.name("register_b").unwrap().as_str().parse::<i128>().unwrap();
        let register_c = captures.name("register_c").unwrap().as_str().parse::<i128>().unwrap();
        let program: Vec<i8> = captures
            .name("program")
            .unwrap()
            .as_str()
            .split(',')
            .map(|x| x.parse::<i8>().unwrap())
            .collect();

        Computer { register_a, register_b, register_c, program, instruction_pointer: 0 }
    } else {
        panic!()
    }
}

#[measure_time]
pub fn part_two(input: &str) -> u64 {
    let mut computer: Computer = parse_input(input);
    let mut candidates: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    let target_program = computer.program.iter().map(|x| x.to_string()).join(",");


    for i in 1..8 {
        candidates.push(Reverse(i));
    }

    while let Some(Reverse(candidate)) = candidates.pop() {
        println!("Candidate: {}", candidate);

        computer.reset(candidate as i128);

        let output = computer.run_program();

        if output == target_program {
            println!("Day 17, part 2 result: {}", candidate);
            return candidate;
        }

        let len = output.len();

        if output == target_program[target_program.len() - len..] {
            for i in 0..8 {
                candidates.push(Reverse((candidate << 3) + i));
            }
        }
    }

    println!("Day 17, part 2 result: No result");
    0
}

fn simulate_loop(a: usize) -> usize {
    let mut b = a % 8;
    b = b ^ 3;
    let c = a >> b;
    b = b ^ c;
    b = b ^ 3;
    b % 8
}

#[cfg(test)]
mod day17 {
    use crate::day17::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), "4,6,3,5,6,3,5,2,1,0")
    }

    const TEST_CASE_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_one_case_two() {
        assert_eq!(part_two(TEST_CASE_2), 117440)
    }
}
