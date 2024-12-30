use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
pub fn part_one(input: &str) -> usize {
    let mut result = 0;
    let (keys, locks) = parse_input(input);

    keys.iter()
        .for_each(|key| {
            locks.iter().for_each(|lock| {
                if key_lock_fit(key.clone(), lock.clone()) { result += 1 }
            })
    });

    println!("Day 25, part 1 result: {:?}", result);

    result
}

fn key_lock_fit(key: Vec<usize>, lock: Vec<usize>) -> bool {
    println!("Checking key {:?} with lock {:?}", key, lock);
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 { return false; }
    }

    println!("Fitting");
    true
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut locks: Vec<Vec<usize>> = Vec::new();
    let mut schemas = input.trim().split("\n\n");

    schemas.for_each(|schema| {
        let mut key_locks: Vec<Vec<char>> = vec![Vec::new();5];

        schema.lines()
            .for_each(|line| line.chars()
                .enumerate()
                .for_each(|(j, c)| {
                    key_locks[j].push(c);
        }));

        let is_key_vec = is_key(key_locks[0].clone());

        let heights = key_locks.iter().map(|key_lock| {
            if is_key_vec {
                6 - key_lock.iter().position(|c| *c == '#').expect("Must be one # in every key column")
            } else {
                key_lock.iter().position(|c| *c == '.').expect("Must be one . in every lock column") - 1
            }
        }).collect();

        match is_key_vec {
            true => {keys.push(heights)}
            false => {locks.push(heights)}
        }
    });

    (keys, locks)
}

fn is_key(key_lock: Vec<char>) -> bool {
    key_lock.iter().position(|c| *c == '.').expect("Must be one . in every column") < key_lock.iter().position(|c| *c == '#').expect("Must be one # in every column")
}

#[measure_time]
pub fn part_two(input: &str) -> usize {
    let result = 0;

    println!("Day 25, part 2 result: {:?}", result);

    result
}

#[cfg(test)]
mod day25 {
    use crate::day25::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), 3)
    }

}
