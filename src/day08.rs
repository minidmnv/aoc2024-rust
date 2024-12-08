use std::collections::{HashMap, HashSet};
use measure_time_macro::measure_time;

#[derive(Debug, Clone)]
struct Grid {
    matrix: Vec<Vec<char>>,
}

impl Grid {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        Self {
            matrix
        }
    }

    fn count_antinodes(&self, harmonics: bool) -> HashSet<(usize, usize)>{
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
        let antennas = self.prepare_antennas_map();

        for antenna in antennas.keys() {
            let antenna_positions = antennas.get(antenna).unwrap();
            if antenna_positions.len() < 2 {
                continue;
            }

            for i in 0..antenna_positions.len() {
                for j in 0.. antenna_positions.len() {
                    if i == j { continue };

                    let new_antinodes =
                        if harmonics {
                            self.get_harmonic_antinodes(antenna_positions[i], antenna_positions[j])
                        } else {
                            self.get_antinodes(antenna_positions[i], antenna_positions[j])
                        };
                    if new_antinodes.len() > 0 {
                        for antinode in new_antinodes {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }
        antinodes
    }

    fn get_antinodes(&self, a1: (usize, usize), a2: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        let compute_coords = |x1: usize, x2: usize| -> Option<usize> {
            if x2 >= x1 {
                x1.checked_sub(x2.checked_sub(x1)?)
            } else {
                x1.checked_sub(x2)?.checked_add(x1)
            }
        };

        if let (Some(new_x), Some(new_y)) = (
            compute_coords(a1.0, a2.0),
            compute_coords(a1.1, a2.1),
        ) {
            if new_x < self.matrix[0].len() && new_y < self.matrix.len() {
                result.push((new_x, new_y));
            }
        }

        if let (Some(new_x), Some(new_y)) = (
            compute_coords(a2.0, a1.0),
            compute_coords(a2.1, a1.1),
        ) {
            if new_x < self.matrix[0].len() && new_y < self.matrix.len() {
                result.push((new_x, new_y));
            }
        }

        result
    }

    fn get_harmonic_antinodes(&self, a1: (usize, usize), a2: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        let step_x = if a2.0 > a1.0 { a2.0 - a1.0 } else { a1.0 - a2.0 };
        let step_y = if a2.1 > a1.1 { a2.1 - a1.1 } else { a1.1 - a2.1 };

        let mut current = a1;

        loop {

            if current.0 < self.matrix[0].len() && current.1 < self.matrix.len() {
                result.push(current);
            } else {
                break;
            }

            current.0 = if a2.0 > a1.0 {
                match current.0.checked_sub(step_x) {
                    Some(val) => val,
                    None => break,
                }
            } else if a2.0 < a1.0 {
                match current.0.checked_add(step_x) {
                    Some(val) => val,
                    None => break,
                }
            } else {
                current.0
            };

            current.1 = if a2.1 > a1.1 {
                match current.1.checked_sub(step_y) {
                    Some(val) => val,
                    None => break,
                }
            } else if a2.1 < a1.1 {
                match current.1.checked_add(step_y) {
                    Some(val) => val,
                    None => break,
                }
            } else {
                current.1
            };
        }

        result
    }

    fn prepare_antennas_map(&self) -> HashMap<char, Vec<(usize, usize)>> {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for i in 0..self.matrix[0].len() {
            for j in 0..self.matrix.len() {
                if let Some(antenna) = match self.matrix[j][i] {
                    '.' => None,
                    other => Some(other),
                } {
                    antennas.entry(antenna).or_insert_with(Vec::new).push((j, i));
                }
            }
        }

        antennas
    }
}

pub fn run(input: &str) {
    let grid = Grid::new(input.lines().map(
        |line| line.chars().collect()
    ).collect());

    part_one(grid.clone());
    part_two(grid.clone());
}

#[measure_time]
fn part_one(grid: Grid) {
    let result = grid.count_antinodes(false);

    println!("Day 8, part 1 result: {}", result.len());
}

#[measure_time]
fn part_two(grid: Grid) {
    let result = grid.count_antinodes(true);
    println!("Day 8, part 2 result: {}", result.len());
}
