use std::collections::HashMap;
use measure_time_macro::measure_time;

pub fn run(input: &str) {
    let stones: Vec<i64> = input.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect();

    part_one(stones.clone());
    part_two(stones.clone());
}

#[measure_time]
fn part_one(stones: Vec<i64>) {
    let mut result = stones;
    for _ in 0..25 {
        result = result.into_iter().flat_map(|stone| {
            match stone {
                engrave if engrave == 0 => vec![1],
                engrave if engrave.abs().to_string().len() % 2 == 0 => {
                    let s = engrave.abs().to_string();
                    let (left, right) = s.split_at(s.len() / 2);
                    vec![left.parse::<i64>().unwrap_or(0), right.parse::<i64>().unwrap_or(0)]
                }
                _ => vec![stone * 2024]
            }
        }).collect();
    }


    println!("Day 11, part 1 result: {:?}", result.len());
}

#[measure_time]
fn part_two(stones: Vec<i64>) {
    let mut result = stones.into_iter().flat_map(|stone| {
        match stone {
            engrave if engrave == 0 => vec![1],
            engrave if engrave.abs().to_string().len() % 2 == 0 => {
                let s = engrave.abs().to_string();
                let (left, right) = s.split_at(s.len() / 2);
                vec![left.parse::<i64>().unwrap_or(0), right.parse::<i64>().unwrap_or(0)]
            }
            _ => vec![stone * 2024]
        }
    }).fold(HashMap::new(), |mut acc, value| {
        *acc.entry(value).or_insert(0) += 1;
        acc
    });

    for _ in 0..74 {
        let new_result: HashMap<i64, usize> = result
            .into_iter()
            .flat_map(|(stone, count)| {
                let values = match stone {
                    engrave if engrave == 0 => vec![1],
                    engrave if engrave.abs().to_string().len() % 2 == 0 => {
                        let s = engrave.abs().to_string();
                        let (left, right) = s.split_at(s.len() / 2);
                        vec![
                            left.parse::<i64>().unwrap_or(0),
                            right.parse::<i64>().unwrap_or(0),
                        ]
                    }
                    _ => vec![stone * 2024],
                };

                values.into_iter().map(move |val| (val, count))
            })
            .fold(HashMap::new(), |mut acc, (value, count)| {
                *acc.entry(value).or_insert(0) += count;
                acc
            });

        result = new_result;
    }

    println!("Day 11, part 2 result: {}", result.values().sum::<usize>());
}
