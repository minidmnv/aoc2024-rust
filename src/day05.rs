use std::collections::HashMap;
use measure_time_macro::measure_time;
use crate::utils::{find_element, get_middle_number, split_around_pivot};

pub fn run(input: &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let instruction_orders: Vec<Vec<i32>> = parts[0].lines().map(|line| line.split("|").map(|num| num.parse::<i32>().unwrap()).collect()).collect();
    let instructions: Vec<Vec<i32>> = parts[1].lines().map(|line| line.split(",").map(|num| num.parse::<i32>().unwrap()).collect()).collect();

    part_one(instruction_orders.clone(), instructions.clone());
    part_two(instruction_orders.clone(), instructions.clone());
}

#[measure_time]
fn part_one(instruction_order: Vec<Vec<i32>>, instructions: Vec<Vec<i32>>) {
    let mut elements_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut result = 0;

    for k in 0..instructions.len() {
        let mut instruction_result: bool = true;

        for (index, &instruction) in instructions[k].iter().enumerate() {
            if !elements_map.contains_key(&instruction) {
                let mut elements_after = Vec::new();

                instruction_order.iter()
                    .filter(|order| order[0] == instruction)
                    .for_each(|order| elements_after.push(order[1]));
                elements_map.insert(instruction, elements_after);
            }

            let elements_after_order = elements_map.get(&instruction).unwrap();

            let (elements_before, _elements_after) = split_around_pivot(instructions[k].clone(), index);

            if elements_before.iter().any(|elem| find_element(elements_after_order, elem).is_some()) {
                instruction_result = false;
                break;
            }
        }

        if instruction_result { result += get_middle_number(&instructions[k]).unwrap() }
    }

    println!("Day 5, part 1 result: {}", result);
}

#[measure_time]
fn part_two(instruction_order: Vec<Vec<i32>>, instructions: Vec<Vec<i32>>) {
    let mut elements_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for order_pair in instruction_order {
        let mut orders = elements_map
            .get(&order_pair[0])
            .cloned()
            .unwrap_or_else(Vec::new);

        orders.push(order_pair[1]);

        elements_map.insert(order_pair[0], orders);
    }

    let mut result = 0;

    for k in 0..instructions.len() {
        for (index, &instruction) in instructions[k].iter().enumerate() {
            let elements_after_order = elements_map.get(&instruction).unwrap();

            let (elements_before, _elements_after) = split_around_pivot(instructions[k].clone(), index);

            if elements_before.iter().any(|elem| find_element(elements_after_order, elem).is_some()) {
                let correct_list = sort_pages(&instructions[k], &elements_map);
                result += get_middle_number(&correct_list).unwrap();
                break;
            }
        }
    }
    println!("Day 5, part 2 result: {}", result);
}

fn sort_pages(page_list: &[i32], page_order: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut sorted_list: Vec<i32> = page_list.to_vec();
    let mut index = 0;

    while index < sorted_list.len() {
        let number = sorted_list[index];

        let after_elements = match page_order.get(&number) {
            Some(elements) => elements,
            None => {
                index += 1;
                continue;
            }
        };

        if let Some((index_before, _element_before)) = sorted_list[..index]
            .iter()
            .enumerate()
            .find(|(_, element)| after_elements.contains(element))
        {
            sorted_list = {
                let mut new_list = sorted_list[..index_before].to_vec();
                new_list.push(number);
                new_list.extend_from_slice(&sorted_list[index_before..index]);
                new_list.extend_from_slice(&sorted_list[index + 1..]);
                new_list
            };

            index = 0;
        } else {
            index += 1;
        }
    }

    sorted_list
}

