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
                _ => ()
            }
        }
        println!("Result: {}, {}, {} ", result, &self.result, result.eq(&self.result));
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


    part_one(equations);
    // part_two(&mut grid.clone());
}

fn part_one(equations: Vec<Equation>) {
    let possible_operations = vec!['+', '*'];

    let result = equations.iter().filter(|equation| {
        let mut is_valid = false;
        let operation_count = equation.operands.len() - 1;
        let operation_permutations = generate_combinations(possible_operations.clone(), operation_count);

        for operation_sequence in operation_permutations {
            if equation.is_valid(operation_sequence) { is_valid = true; break; }
        }

        println!("Returning: {}", is_valid);
        is_valid
    }).count();

    println!("Day 7, part 1 result: {}", result);
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

