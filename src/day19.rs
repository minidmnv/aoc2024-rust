use std::collections::HashMap;
use itertools::Itertools;
use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
pub fn part_one(input: &str) -> i32 {
    let (available_towels, desired_designs) = parse_input(input);
    let mut result = 0;

    for design in desired_designs {
        if check_design(design, available_towels.clone()) { result += 1 }
    }

    println!("Day 19, part 1 result: {:?}", result);

    result
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = input.trim().split("\n\n");
    let raw_available_towels = parts.next().unwrap();
    let raw_desired_designs = parts.next().unwrap();

    let available_towels: Vec<&str> = raw_available_towels.split(",").map(|towel| towel.trim()).collect();
    let desired_designs: Vec<&str> = raw_desired_designs.lines().map(|design| design.trim()).collect();

    (available_towels, desired_designs)
}


fn check_design(design: &str, available_towels: Vec<&str>) -> bool {
    fn helper(design: &str, towels: &Vec<&str>, memo: &mut HashMap<String, bool>) -> bool {
        if design.is_empty() {
            return true;
        }

        if let Some(&result) = memo.get(design) {
            return result;
        }

        for towel in towels.iter() {
            if design.starts_with(towel) {
                let remaining_design = &design[towel.len()..];
                if helper(remaining_design, towels, memo) {
                    memo.insert(design.to_string(), true);
                    return true;
                }
            }
        }

        memo.insert(design.to_string(), false);
        false
    }

    let mut memo = HashMap::new();
    helper(design, &available_towels, &mut memo)
}

fn count_design_ways(design: &str, available_towels: Vec<&str>) -> usize {

    fn helper(design: &str, towels: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&cached_result) = memo.get(design) {
            return cached_result;
        }

        let mut ways = 0;

        for towel in towels.iter() {
            if design.starts_with(towel) {
                let remaining_design = &design[towel.len()..];
                ways += helper(remaining_design, towels, memo);
            }
        }

        memo.insert(design.to_string(), ways);
        ways
    }

    let mut memo = HashMap::new();
    helper(design, &available_towels, &mut memo)
}

#[measure_time]
pub fn part_two(input: &str) -> usize {
    let (available_towels, desired_designs) = parse_input(input);
    let mut result = 0;

    for design in desired_designs {
        result += count_design_ways(design, available_towels.clone());
    }

    println!("Day 19, part 2 result: {:?}", result);

    result
}

#[cfg(test)]
mod day19 {
    use crate::day19::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), 6)
    }

}
