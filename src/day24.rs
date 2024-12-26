use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
pub fn part_one(input: &str, cheat_time: usize, disable_collision_offset: usize) -> usize {
    let result = 0;

    println!("Day 22, part 1 result: {:?}", result);

    result
}

fn parse_input(input: &str) -> () {
}

#[measure_time]
pub fn part_two(input: &str, cheat_time: usize, disable_collision_offset: usize) -> usize {
    let result = 0;

    println!("Day 22, part 2 result: {:?}", result);

    result
}

#[cfg(test)]
mod day22 {
    use crate::day22::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), 4)
    }

    #[test]
    fn test_part_two_case_one() {
        assert_eq!(part_two(TEST_CASE_1), 3)
    }

}
