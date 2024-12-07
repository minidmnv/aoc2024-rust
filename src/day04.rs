use measure_time_macro::measure_time;
use crate::utils::parse_to_grid;

pub fn run(input: &str) {
    let grid = parse_to_grid(input);

    part_one(grid.clone());
    part_two(grid.clone());
}

#[measure_time]
fn part_one(grid: Vec<Vec<char>>) {
    const XMAS: &str = "XMAS";

    let result: i32 = grid.iter().enumerate().map(|(x, row)| row.iter().enumerate().map(|(y, _value)| match_word_line_in_grid(grid.clone(), x, y, XMAS)).sum::<i32>()).sum::<i32>();

    println!("Day 4, part 1 result: {}", result);
}

fn match_word_line_in_grid(grid: Vec<Vec<char>>, x: usize, y: usize, word: &str) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count: i32 = 0;

    if grid[x][y] != word.chars().next().unwrap() {
        return 0
    }

    let word_length = word.len();

    let x_dirs: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let y_dirs: [i32; 8] = [-1, 0, 1, 1, -1, -1, 0, 1];

    for dir_i in 0..x_dirs.len() {

        let mut curr_x: i32 = x as i32;
        let mut curr_y: i32 = y as i32;

        for k in 1..word_length {
            curr_x += x_dirs[dir_i];
            curr_y += y_dirs[dir_i];

            if curr_x < 0 || curr_x >= height as i32 || curr_y < 0 || curr_y >= width as i32 { break }
            if word.chars().nth(k).unwrap() != grid[curr_x as usize][curr_y as usize] { break }

            if k == word_length - 1 { count += 1 }
        }
    }
    count
}

fn match_x_mas_in_grid(grid: Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    const CENTER_CHAR: char = 'A';

    let height = grid.len();
    let width = grid[0].len();

    if grid[x][y] != CENTER_CHAR {
        return 0
    }

    let x_dirs: [i32; 4] = [-1, 1, -1, 1];
    let y_dirs: [i32; 4] = [-1, 1, 1, -1];

    let result_array: Vec<char> = (0..x_dirs.len()).map(|dir_i| {
        let curr_x: i32 = x as i32 + x_dirs[dir_i];
        let curr_y: i32 = y as i32 + y_dirs[dir_i];

        if curr_x < 0 || curr_x >= height as i32 || curr_y < 0 || curr_y >= width as i32 { return 'X' }

        grid[curr_x as usize][curr_y as usize]
    }).collect();

    if result_array[0] == result_array[1] || result_array[2] == result_array[3] || result_array.iter().filter(|&&cr| cr == 'M').count() != 2 || result_array.iter().filter(|&&cr| cr == 'S').count() != 2 { 0 } else { 1 }
}

#[measure_time]
fn part_two(grid: Vec<Vec<char>>) {

    let result: i32 = grid.iter().enumerate().map(|(x, row)| row.iter().enumerate().map(|(y, _value)| match_x_mas_in_grid(grid.clone(), x, y)).sum::<i32>()).sum::<i32>();

    println!("Day 4, part 2 result: {}", result);
}

