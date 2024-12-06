use std::ops::{Index, RangeBounds};
use std::option::IterMut;

pub fn get_middle_number(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None; // Return None if the Vec is empty
    }
    let middle_index = numbers.len() / 2;
    Some(numbers[middle_index])
}

pub fn split_around_pivot(vec: Vec<i32>, pivot: usize) -> (Vec<i32>, Vec<i32>) {
    if pivot >= vec.len() {
        panic!("Pivot index out of bounds");
    }

    let left = vec[..pivot].to_vec();
    let right = vec[pivot + 1..].to_vec();

    (left, right)
}

pub fn find_element<T, I>(collection: I, target: T) -> Option<T>
    where
        T: PartialEq + Copy,
        I: IntoIterator<Item = T>,
{
    collection.into_iter().find(|&item| item == target)
}

pub fn parse_to_grid(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    grid
}

pub fn get_element_from_grid(collection: &Vec<Vec<char>>, coords: (usize, usize)) -> char
{
    collection[coords.0][coords.1]
}

pub fn count_char_in_grid(grid: &Vec<Vec<char>>, target: char) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == target)
        .count()
}

pub fn print_grid_with_delimiters(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("|{}|", row.iter().map(|&c| c.to_string()).collect::<Vec<_>>().join("|"));
    }

    println!("{}", "-".repeat(grid[0].len()));
}

