use measure_time_macro::measure_time;

#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

pub fn run(input: &str) {

    // part_one(input);
    // part_two(&equations);
}

#[measure_time]
fn part_one(equations: &Vec<Equation>) {

    println!("Day 8, part 1 result: {}", result);
}

#[measure_time]
fn part_two(equations: &Vec<Equation>) {

    println!("Day 8, part 2 result: {}", result);
}
