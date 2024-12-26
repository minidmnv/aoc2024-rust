use std::error::Error;
use measure_time_macro::measure_time;

type Warehouse = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Crate,
    BigCrateLeft,
    BigCrateRight,
}

use Tile::*;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

use Move::*;

fn get_input(input: &str) -> (Warehouse, (usize, usize), Vec<Move>) {
    let mut robot_pos = (0, 0);

    let mut parts = input.trim().split("\n\n");
    let raw_warehouse = parts.next().unwrap();
    let raw_movements = parts.next().unwrap();

    let warehouse = raw_warehouse
        .trim()
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(|(col_no, c)| match c {
                    '#' => Wall,
                    '.' => Empty,
                    'O' => Crate,
                    '@' => {
                        robot_pos = (line_no, col_no);
                        Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let movements = raw_movements
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|e| match e {
                    '<' => Left,
                    '^' => Up,
                    'v' => Down,
                    '>' => Right,
                    _ => unreachable!(),
                })
                .collect::<Vec<Move>>()
        })
        .collect();

    (warehouse, robot_pos, movements)
}

fn try_move(movement: &Move, position: (usize, usize), warehouse: &mut Warehouse) -> bool {
    let neighbor_pos = match movement {
        Up => (position.0 - 1, position.1),
        Down => (position.0 + 1, position.1),
        Left => (position.0, position.1 - 1),
        Right => (position.0, position.1 + 1),
    };
    match warehouse[neighbor_pos.0][neighbor_pos.1] {
        Empty => {
            warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
            warehouse[position.0][position.1] = Empty;
            true
        }
        Crate => {
            if try_move(movement, neighbor_pos, warehouse) {
                warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
                warehouse[position.0][position.1] = Empty;
                true
            } else {
                false
            }
        }
        Wall => false,
        _ => unreachable!(),
    }
}

fn display(warehouse: &Warehouse, robot: (usize, usize)) {
    for (line_no, line) in warehouse.iter().enumerate() {
        for (col_no, elem) in line.iter().enumerate() {
            if (line_no, col_no) == robot {
                print!("@");
            } else {
                print!(
                    "{}",
                    match elem {
                        Wall => '#',
                        Crate => 'O',
                        Empty => '.',
                        BigCrateLeft => '[',
                        BigCrateRight => ']',
                    }
                );
            }
        }
        println!();
    }
}

#[measure_time]
pub fn part_one(input: &str) -> Result<(), Box<dyn Error + 'static>> {
    let (mut warehouse, mut position, movements) = get_input(input);

    for movement in movements {
        if try_move(&movement, position, &mut warehouse) {
            position = match movement {
                Up => (position.0 - 1, position.1),
                Down => (position.0 + 1, position.1),
                Left => (position.0, position.1 - 1),
                Right => (position.0, position.1 + 1),
            };
        }
    }

    let mut gps = 0;

    for (line_no, line) in warehouse.iter().enumerate() {
        for (col_no, elem) in line.iter().enumerate() {
            if let Crate = elem {
                gps += 100 * line_no + col_no
            }
        }
    }

    println!("Day 15, part 1 result: {:?}", gps);

    Ok(())
}

fn enlarge_warehouse(small_warehouse: Warehouse) -> Warehouse {
    let mut larger_warehouse = vec![];
    for line in small_warehouse {
        let mut larger_line = vec![];
        for tile in line {
            if let Crate = tile {
                larger_line.push(BigCrateLeft);
                larger_line.push(BigCrateRight);
            } else {
                larger_line.push(tile);
                larger_line.push(tile);
            }
        }
        larger_warehouse.push(larger_line);
    }
    larger_warehouse
}

fn try_move_larger_crates(
    movement: &Move,
    position: (usize, usize),
    warehouse: &mut Warehouse,
    apply: bool,
) -> bool {
    let neighbor_pos = match movement {
        Up => (position.0 - 1, position.1),
        Down => (position.0 + 1, position.1),
        Left => (position.0, position.1 - 1),
        Right => (position.0, position.1 + 1),
    };
    match warehouse[neighbor_pos.0][neighbor_pos.1] {
        Empty => {
            if apply {
                warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
                warehouse[position.0][position.1] = Empty;
            }
            true
        }
        Wall => false,
        BigCrateLeft => match movement {
            Up | Down => {
                if try_move_larger_crates(movement, neighbor_pos, warehouse, apply)
                    & try_move_larger_crates(
                    movement,
                    (neighbor_pos.0, neighbor_pos.1 + 1),
                    warehouse,
                    apply,
                )
                {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Empty;
                    }
                    true
                } else {
                    false
                }
            }
            _ => {
                if try_move_larger_crates(movement, neighbor_pos, warehouse, apply) {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Empty;
                    }
                    true
                } else {
                    false
                }
            }
        },
        BigCrateRight => match movement {
            Up | Down => {
                if try_move_larger_crates(movement, neighbor_pos, warehouse, apply)
                    & try_move_larger_crates(
                    movement,
                    (neighbor_pos.0, neighbor_pos.1 - 1),
                    warehouse,
                    apply,
                )
                {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Empty;
                    }
                    true
                } else {
                    false
                }
            }
            _ => {
                if try_move_larger_crates(movement, neighbor_pos, warehouse, apply) {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Empty;
                    }
                    true
                } else {
                    false
                }
            }
        },
        _ => unreachable!(),
    }
}

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
fn part_two(input: &str) -> Result<(), Box<dyn Error + 'static>> {
    let (warehouse, mut position, movements) = get_input(input);
    let mut larger_warehouse = enlarge_warehouse(warehouse);
    position.1 *= 2;

    for movement in movements {
        if try_move_larger_crates(&movement, position, &mut larger_warehouse, false) {
            try_move_larger_crates(&movement, position, &mut larger_warehouse, true);
            position = match movement {
                Up => (position.0 - 1, position.1),
                Down => (position.0 + 1, position.1),
                Left => (position.0, position.1 - 1),
                Right => (position.0, position.1 + 1),
            };
        }
    }
    display(&larger_warehouse, position);

    let mut gps = 0;

    for (line_no, line) in larger_warehouse.iter().enumerate() {
        for (col_no, elem) in line.iter().enumerate() {
            if let BigCrateLeft = elem {
                gps += 100 * line_no + col_no
            }
        }
    }

    println!("Day 15, part 2 result: {:?}", gps);

    Ok(())
}
