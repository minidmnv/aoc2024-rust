use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
fn part_one(input: &str) {
    let (mut column1, mut column2) = prepare_numbers_columns(input);

    column1.sort();
    column2.sort();

    let sum_of_differences: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Day 1, part 1 result: {}", sum_of_differences);
}

#[measure_time]
fn part_two(input: &str) {
    let (column1, column2) = prepare_numbers_columns(input);
    let sum_of_weights: i32 = column1.iter().map(|&number| number * (column2.iter().filter(|&&x| x == number).count() as i32)).sum();

    println!("Day 1, part 2 result: {}", sum_of_weights);
}

fn prepare_numbers_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if let Ok(num1) = numbers[0].parse::<i32>() {
            column1.push(num1);
        }
        if let Ok(num2) = numbers[1].parse::<i32>() {
            column2.push(num2);
        }
    }
    (column1, column2)
}
