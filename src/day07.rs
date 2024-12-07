use std::fmt::format;

#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn new(result: i64, operands: Vec<i64>) -> Self {
        Self {
            result,
            operands
        }
    }
    pub fn is_valid(&self, operations: Vec<char>) -> bool {
        let mut result = self.operands[0];

        for (&operand, operation) in self.operands[1..].iter().zip(operations) {
            match operation {
                '+' => result += operand,
                '*' => result *= operand,
                '|' => result = format!("{}{}", result, operand).parse::<i64>().expect("Failed to parse concatenated numbers"),
                _ => ()
            }
        }
        result.eq(&self.result)
    }
}

pub fn run(input: &str) {
    let equations = input.lines().map(|line| {
        let mut parts = line.split(":");
        let result = parts.next().unwrap().trim().parse::<i64>().expect("Invalid number format");
        let operands = parts.next().unwrap().split_whitespace().map(|n| n.parse::<i64>().expect("Invalid number format")).collect();
        Equation::new(result, operands)
    }).collect();


    part_one(&equations);
    part_two(&equations);
}

fn part_one(equations: &Vec<Equation>) {
    let possible_operations = vec!['+', '*'];

    let result = calculate_calibration_result(equations, possible_operations);

    println!("Day 7, part 1 result: {}", result);
}

fn part_two(equations: &Vec<Equation>) {
    let possible_operations = vec!['+', '*', '|'];

    let result = calculate_calibration_result(equations, possible_operations);

    println!("Day 7, part 2 result: {}", result);
}

fn calculate_calibration_result(equations: &Vec<Equation>, possible_operations: Vec<char>) -> i64 {
    let result: i64 = equations.iter().filter(|equation| {
        let mut is_valid = false;
        let operation_count = equation.operands.len() - 1;
        let operation_permutations = generate_combinations(possible_operations.clone(), operation_count);

        for operation_sequence in operation_permutations {
            if equation.is_valid(operation_sequence) {
                is_valid = true;
                break;
            }
        }

        is_valid
    }).map(|equation| equation.result).sum();

    result
}

fn generate_combinations(operations: Vec<char>, n: usize) -> Vec<Vec<char>> {
    let k = operations.len();
    let mut combinations = Vec::new();

    for i in 0..k.pow(n as u32) {
        let mut index = i;
        let mut combination = Vec::new();

        for _ in 0..n {
            let op_index = index % k;
            combination.push(operations[op_index]);
            index /= k;
        }

        combinations.push(combination);
    }

    combinations
}

