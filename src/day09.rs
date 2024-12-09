use measure_time_macro::measure_time;

fn create_memory_array(input: &str) -> Vec<Option<i64>> {
    let mut result = Vec::new();
    let mut file_id = 0;
    let memory: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect();

    for pair in memory.chunks(2) {
        match *pair {
            [file_length, free_space_length] => {
                result.extend(vec![Some(file_id); file_length]);
                result.extend(vec![None; free_space_length]);
            }
            [file_length] => {
                result.extend(vec![Some(file_id); file_length]);
            }
            _ => unreachable!(),
        }

        file_id += 1;
    }

    result
}

pub fn run(input: &str) {
    let memory_array = create_memory_array(input);

    part_one(memory_array.clone());
    part_two(memory_array.clone());
}

#[measure_time]
fn part_one(memory: Vec<Option<i64>>) {
    let mut result_array = memory;
    loop {
        let first_free_space_index = result_array.iter().position(|memory| memory.is_none()).unwrap();
        let last_file_index = result_array.iter().rposition(|memory| memory.is_some()).unwrap();

        if first_free_space_index > last_file_index { break; };

        result_array[first_free_space_index] = result_array[last_file_index];
        result_array[last_file_index] = None;
    }

    let result: i64 = result_array.iter().filter(|file_id| file_id.is_some()).enumerate().map(|(index, file_id)| index as i64 * file_id.unwrap()).sum();

    println!("Day 9, part 1 result: {}", result);
}

#[measure_time]
fn part_two(memory: Vec<Option<i64>>) {
    let mut result_array = memory;
    let mut current_file_id = result_array.iter().rfind(|memory| memory.is_some()).unwrap().unwrap();

    loop {
        if current_file_id < 0 {
            break;
        }

        let file_size = result_array.iter().filter(|memory| memory.is_some()).filter(|memory| memory.unwrap() == current_file_id).count();
        let file_index = result_array.iter().position(|memory| memory.unwrap_or_else(||-1) == current_file_id).unwrap();

        let mut current_free_space_index = 0;

        loop {
            if let Some([free_space_index, free_space_size]) = get_next_free_space(&result_array, current_free_space_index) {
                if free_space_index > file_index { break };

                if file_size <= free_space_size {
                    for i in 0..file_size {
                        result_array[free_space_index + i] = Some(current_file_id);
                        result_array[file_index + file_size - 1 - i] = None;
                    }
                    break;
                }

                current_free_space_index = free_space_index + free_space_size;
            } else { break }


        }
        current_file_id -= 1;
    }

    let result: i64 = result_array.iter().enumerate().map(|(index, file_id)| index as i64 * file_id.unwrap_or_else(||0)).sum();
    println!("Day 9, part 2 result: {}", result);
}

fn get_next_free_space(array: &Vec<Option<i64>>, index: usize) -> Option<[usize; 2]> {
    if let Some(free_space_index) = array.into_iter().skip(index).position(|memory| memory.is_none()) {
        let current_index = free_space_index + index;
        let size = current_index + array.into_iter().skip(current_index + 1usize).position(|memory| memory.is_some()).unwrap_or_else(||0) - current_index + 1;
        return Some([current_index, size])
    }

    None
}
