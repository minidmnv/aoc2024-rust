use measure_time_macro::measure_time;

pub fn run(input: &str) {

    // part_one(graph.clone());
    // part_two(graph.clone());
}

#[measure_time]
fn part_one(graph: Graph) {
    println!("Day 10, part 1 result: {:?}", result);
}

#[measure_time]
fn part_two(graph: Graph) {
    println!("Day 10, part 2 result: {}", result);
}
