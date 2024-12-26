use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::BitXor;
use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
pub fn part_one(input: &str) -> i128 {
    let secrets = parse_input(input);

    let result: i128 = secrets.iter().map(|secret| calculate_nth_secret_number(*secret, 2000)).sum();

    println!("Day 22, part 1 result: {:?}", result);

    result
}

fn parse_input(input: &str) -> Vec<i128> {
    input.lines().map(|secret| secret.parse::<i128>().expect("All numbers are correct in input")).collect()
}

fn get_price(number: i128) -> i8 {
    (number % 10) as i8
}

#[measure_time]
pub fn part_two(input: &str) -> i32 {
    let secrets = parse_input(input);

    let sequence_maps: Vec<HashMap<[i8;4], i8>> = secrets.iter().map(|secret| generate_sequences(*secret, 2000)).collect();
    let sequence_results = group_and_sum(sequence_maps);
    let result = sequence_results.values().max().cloned().unwrap();

    println!("Day 22, part 2 result: {:?}", result);

    result
}

fn group_and_sum(vec_of_maps: Vec<HashMap<[i8; 4], i8>>) -> HashMap<[i8; 4], i32> {
    let mut grouped: HashMap<[i8; 4], i32> = HashMap::new();

    for map in vec_of_maps {
        for (key, value) in map {
            // Sum the values for each key
            *grouped.entry(key).or_insert(0) += value as i32;
        }
    }

    grouped
}

fn generate_sequences(seed: i128, repetitions: usize) -> HashMap<[i8;4], i8> {
    let mut result_sequence: HashMap<[i8;4], i8> = HashMap::new();
    let mut prices: Vec<i8> = Vec::new();
    let mut current = seed;

    for _ in 0..repetitions {
        current = calculate_secret_number(current);
        prices.push(get_price(current));
    }

    for prices in prices.windows(5) {
        let sequence = [prices[1] - prices[0], prices[2] - prices[1], prices[3] - prices[2], prices[4] - prices[3]];
        match result_sequence.entry(sequence) {
            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(prices[4]);
            }
            std::collections::hash_map::Entry::Occupied(_) => {
            }
        }
    }

    result_sequence
}

fn calculate_nth_secret_number(seed: i128, repetitions: i32) -> i128 {
    let mut result = seed;
    for _ in 0..repetitions {
        result = calculate_secret_number(result);
    }

    result
}

fn calculate_secret_number(seed: i128) -> i128 {
    let first_op = seed * 64;
    let mut new_secret = first_op.bitxor(seed) % 16777216;
    new_secret = ((new_secret as f64 / 32_f64).floor() as i128).bitxor(new_secret) % 16777216;
    new_secret = (new_secret * 2048).bitxor(new_secret) % 16777216;

    new_secret
}

fn calculate_price_difference(seed: i128) -> i8 {
    let first_op = seed * 64;
    let mut new_secret = first_op.bitxor(seed) % 16777216;
    new_secret = ((new_secret as f64 / 32_f64).floor() as i128).bitxor(new_secret) % 16777216;
    new_secret = (new_secret * 2048).bitxor(new_secret) % 16777216;

    get_price(seed) - get_price(new_secret)
}


fn hash_vec(vec: &Vec<i8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod day22 {
    use crate::day22::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "1
10
100
2024";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), 8685429 + 4700978 + 15273692 + 8667524)
    }

    #[test]
    fn test_calculate_secret_number() {
        assert_eq!(calculate_secret_number(123), 15887950);
        assert_eq!(calculate_secret_number(15887950), 16495136);
        assert_eq!(calculate_secret_number(16495136), 527345);
        assert_eq!(calculate_secret_number(527345), 704524);
        assert_eq!(calculate_secret_number(704524), 1553684);
        assert_eq!(calculate_secret_number(1553684), 12683156);
        assert_eq!(calculate_secret_number(12683156), 11100544);
        assert_eq!(calculate_secret_number(11100544), 12249484);
        assert_eq!(calculate_secret_number(12249484), 7753432);
        assert_eq!(calculate_secret_number(7753432), 5908254);
    }

}
