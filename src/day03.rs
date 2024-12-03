use regex::{Regex};

pub fn run(input: &str) {
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &&str) {
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    let result: i32 = input.lines().flat_map(|line| {
        re.captures_iter(line).map(|caps| {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();

            num1 * num2
        })
    }).sum();

    println!("Day 3, part 1 result: {}", result);
}

fn part_two(input: &&str) {
    const DO_INSTRUCTION: &str = "do()";
    const DONT_INSTRUCTION: &str = "don't()";

    let lines_instructions: Vec<bool> = input.lines().map(|line| {
        let do_pos = line.rfind(DO_INSTRUCTION).unwrap();
        let dont_pos = line.rfind(DONT_INSTRUCTION).unwrap();

        do_pos > dont_pos
    }).collect();


    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let res: i32 = input.lines().enumerate().flat_map(|(instruction_index, instruction)| {
        re.captures_iter(instruction).map({
            let line_instruction_cloned = lines_instructions.clone();
            move |caps| {
                let num1: i32 = caps[1].parse().unwrap();
                let num2: i32 = caps[2].parse().unwrap();
                println!("Checking mul({},{})", num1, num2);
                let match_index: i32 = caps.get(0).unwrap().start() as i32;

                if let Some(substring) = instruction.get(..match_index as usize) {
                    return if let Some(do_pos) = substring.rfind(DO_INSTRUCTION) {
                        return if let Some(dont_pos) = substring.rfind(DONT_INSTRUCTION) {
                            return if do_pos > dont_pos { num1 * num2 } else { 0 };
                        } else { num1 * num2 };
                    } else if let Some(_dont_pos) = substring.rfind(DONT_INSTRUCTION) { 0 }
                    else if instruction_index > 0 {
                        let index = instruction_index - 1;
                        return if let Some(&previous_value) = line_instruction_cloned.get(index) {
                            return if previous_value { num1 * num2 } else { 0 };
                        } else { 0 };
                    } else { num1 * num2 };
                }
                0
            }
        })
    }).sum();

    println!("Day 3, part 2 result: {}", res);
}

