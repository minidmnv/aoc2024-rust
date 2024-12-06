use std::collections::HashSet;
use crate::utils::{count_char_in_grid, get_element_from_grid, parse_to_grid, print_grid_with_delimiters};

const DIRECTIONS: [char; 4] = ['^', '>', 'v', '<'];
const DIRECTION_MAP: &[(char, (i32, i32))] = &[
    ('^', (-1, 0)),
    ('>', (0, 1)),
    ('v', (1, 0)),
    ('<', (0, -1)),
];

fn get_direction(c: char) -> Option<(i32, i32)> {
    DIRECTION_MAP.iter().find_map(|&(dir, vector)| {
        if dir == c {
            Some(vector)
        } else {
            None
        }
    })
}

pub fn run(input: &str) {

    let mut grid = parse_to_grid(input);

    part_one(&mut grid.clone());
    part_two(&mut grid.clone());
}

fn part_one(mut grid: &mut Vec<Vec<char>>) {
    let mut is_guard_on_map = true;
    let (mut direction, mut guard_current_position, mut guard_current_direction) = find_guard_position_with_direction(grid).unwrap();
    let mut current_direction_index = DIRECTIONS.iter().position(|&d| d == direction).unwrap();

    while is_guard_on_map {
        if move_guard(
            &mut grid,
            &mut direction,
            &mut guard_current_position,
            &mut guard_current_direction,
            &mut current_direction_index,
        ) {
            break;
        }
    }

    println!("Day 6, part 1 result: {}", count_char_in_grid(grid, 'X'));
}

fn move_guard(mut grid: &mut Vec<Vec<char>>, mut direction: &mut char, mut guard_current_position: &mut (usize, usize), mut guard_current_direction: &mut (i32, i32), mut current_direction_index: &mut usize) -> bool {
    let new_position = move_position_in_direction(guard_current_position, guard_current_direction);
    if new_position.is_none() || new_position.unwrap().1 >= grid.len() || new_position.unwrap().0 >= grid[0].len() {
        grid[guard_current_position.0][guard_current_position.1] = 'X';
        return true;
    }

    let map_element: char = get_element_from_grid(grid, new_position.unwrap());
    if map_element != '#' {
        grid[guard_current_position.0][guard_current_position.1] = 'X';
        *guard_current_position = new_position.unwrap();
    } else {
        *current_direction_index = (*current_direction_index + 1usize) % DIRECTIONS.len();
        *direction = DIRECTIONS[*current_direction_index];
        *guard_current_direction = get_direction(DIRECTIONS[*current_direction_index]).unwrap();
    }
    false
}

fn part_two(mut grid: &mut Vec<Vec<char>>) {
    let clean_grid = grid.clone();

    let (mut direction, mut guard_current_position, mut guard_current_direction) = find_guard_position_with_direction(&clean_grid).unwrap();
    let mut current_direction_index = DIRECTIONS.iter().position(|&d| d == direction).unwrap();
    let mut possible_blockers_positions: HashSet<(usize, usize)> = HashSet::new();
    let starting_position = guard_current_position;

    loop {
        if move_guard(
            &mut grid,
            &mut direction,
            &mut guard_current_position,
            &mut guard_current_direction,
            &mut current_direction_index,
        ) {
            break;
        }
        possible_blockers_positions.insert(guard_current_position);
    }
    possible_blockers_positions.remove(&starting_position);

    let response: usize = possible_blockers_positions.iter().map(|blocker_position|
        {
            let mut blocked_grid = clean_grid.clone();
            blocked_grid[blocker_position.0][blocker_position.1] = '#';
            blocked_grid
        }
    ).filter(| blocked_grid| has_loop(blocked_grid)).count();


    println!("Day 6, part 2 result: {}", response);
}

fn find_guard_position_with_direction(
    grid: &Vec<Vec<char>>,
) -> Option<(char, (usize, usize), (i32, i32))> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if let Some(direction) = get_direction(cell) {
                return Some((cell, (row_index, col_index), direction));
            }
        }
    }
    None
}

fn move_position_in_direction(
    position: &mut (usize, usize),
    direction: &mut (i32, i32),
) -> Option<(usize, usize)> {
    let val0 = position.0 as i32 + direction.0;
    let val1 = position.1 as i32 + direction.1;

    if val0 > 0 && val1 > 0 { Some((val0 as usize, val1 as usize)) } else { None }
}

fn has_loop(grid_template: &Vec<Vec<char>>) -> bool {
    let mut grid = grid_template.clone();
    let mut fast_grid = grid_template.clone();
    let mut is_loop = false;

    let (mut direction, mut guard_current_position, mut guard_current_direction) = find_guard_position_with_direction(&grid_template).unwrap();
    let mut current_direction_index = DIRECTIONS.iter().position(|&d| d == direction).unwrap();

    let (mut fast_direction, mut fast_guard_current_position, mut fast_guard_current_direction) = find_guard_position_with_direction(&grid_template).unwrap();
    let mut fast_current_direction_index = DIRECTIONS.iter().position(|&d| d == fast_direction).unwrap();

    loop {
        let guard_map = move_guard(
            &mut grid,
            &mut direction,
            &mut guard_current_position,
            &mut guard_current_direction,
            &mut current_direction_index,
        );
        let fast_guard_map = if !move_guard(
            &mut fast_grid,
            &mut fast_direction,
            &mut fast_guard_current_position,
            &mut fast_guard_current_direction,
            &mut fast_current_direction_index,
        ) {
            move_guard(
                &mut fast_grid,
                &mut fast_direction,
                &mut fast_guard_current_position,
                &mut fast_guard_current_direction,
                &mut fast_current_direction_index,
            )
        } else { true };

        match (guard_map, fast_guard_map) {
            (true, true) => {
                break;
            }
            (true, false) => {
                break;
            }
            (false, true) => {
                break;
            }
            (false, false) => {
                if guard_current_position.eq(&fast_guard_current_position) && guard_current_direction.eq(&fast_guard_current_direction)
                {
                    is_loop = true;
                    break;
                }
            }
        };
    }

    is_loop //1910 - 2992
}
