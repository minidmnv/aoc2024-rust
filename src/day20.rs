use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input, 100, 2);
    part_two(input, 100, 20);
    //535084 low
    //539179 low
    //855535 low
    //976351 wrong
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn neighbours(&self) -> HashSet<Coordinate> {
        let mut neighbours = HashSet::new();

        if self.y > 0 { neighbours.insert(Coordinate {x: self.x, y: self.y - 1}); };
        if self.x > 0 { neighbours.insert(Coordinate {x: self.x - 1, y: self.y}); };

        neighbours.insert(Coordinate {x: self.x + 1, y: self.y});
        neighbours.insert(Coordinate {x: self.x, y: self.y + 1});

        neighbours
    }

    pub fn neighbours_cheated(&self, cheat_distance: usize) -> HashSet<Coordinate> {
        let mut neighbours = HashSet::new();
        for i in 0..=cheat_distance {

            neighbours.insert(Coordinate {x: self.x + i, y: self.y.saturating_sub(cheat_distance) + i});
            neighbours.insert(Coordinate {x: self.x + cheat_distance - i, y: self.y + i});
            neighbours.insert(Coordinate {x: self.x.saturating_sub(i), y: self.y + cheat_distance - i});
            neighbours.insert(Coordinate {x: self.x.saturating_sub(cheat_distance) + i, y: self.y.saturating_sub(i)});

        }

        neighbours
    }
}
#[derive(Debug, Clone)]
struct Grid {
    walls: HashSet<Coordinate>,
    start: Coordinate,
    end: Coordinate,
}

impl Grid {

    fn new(input: &str) -> Self {
        let grid_data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut start: Option<Coordinate> = None;
        let mut end: Option<Coordinate> = None;
        let mut walls: HashSet<Coordinate> = HashSet::new();

        for y in 0..grid_data.len() {
            for x in 0..grid_data[0].len() {
                match grid_data[y][x] {
                    'S' => { start = Some(Coordinate {x, y})},
                    'E' => { end = Some(Coordinate {x, y})},
                    '#' => { walls.insert(Coordinate {x, y}); },
                    _ => {}

                }
            }
        }

        Self {
            walls,
            start: start.expect("Start must be found"),
            end: end.expect("End must be found"),
        }
    }

    fn distance_map(self) -> HashMap<Coordinate, usize> {
        let mut posistion: Option<Coordinate> = Some(self.start);
        let mut distance: usize = 0;
        let mut distance_map: HashMap<Coordinate, usize> = HashMap::new();

        while let Some(pos) = posistion {
            distance_map.insert(pos.clone(), distance);
            posistion = pos
                .clone()
                .neighbours()
                .iter()
                .find(|coords| {
                    !self.walls.contains(*coords) && !distance_map.contains_key(*coords)
                })
                .cloned();
            distance += 1;
        }

        distance_map
    }

    fn count_cheats(self, distance_map: HashMap<Coordinate, usize>, cheat_time: usize, disable_collision_offset: usize ) -> usize {
        let mut cheats: HashMap<Vec<Coordinate>, usize> = HashMap::new();
        for entry in distance_map.iter() {
            let cheat_start: &Coordinate = entry.0;
            let distance = entry.1;
            for cheat_duration in 2..=disable_collision_offset {
                for cheat_end in cheat_start.neighbours_cheated(cheat_duration) {
                    let new_distance = distance_map.get(&cheat_end);
                    if let Some(cheated_distance) = new_distance {
                        cheats.insert(Vec::from([cheat_start.clone(), cheat_end]), cheated_distance.saturating_sub(*distance).saturating_sub(cheat_duration));
                    }
                }
            }
        }

        cheats.iter().filter(|cheat| cheat.1 >= &cheat_time).count()
    }
}

#[measure_time]
pub fn part_one(input: &str, cheat_time: usize, disable_collision_offset: usize) -> usize {
    let grid: Grid = parse_input(input);
    let distance_map = grid.clone().distance_map();
    let result = grid.clone().count_cheats(distance_map, cheat_time, disable_collision_offset);

    println!("Day 20, part 1 result: {:?}", result);

    result
}

fn parse_input(input: &str) -> Grid {
    Grid::new(input)
}

#[measure_time]
pub fn part_two(input: &str, cheat_time: usize, disable_collision_offset: usize) -> usize {
    let grid: Grid = parse_input(input);
    let distance_map = grid.clone().distance_map();
    let result = grid.clone().count_cheats(distance_map, cheat_time, disable_collision_offset);

    println!("Day 20, part 2 result: {:?}", result);

    result
}

#[cfg(test)]
mod day20 {
    use crate::day20::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1, 30, 2), 4)
    }

    #[test]
    fn test_part_two_case_one() {
        assert_eq!(part_two(TEST_CASE_1, 75, 20), 3)
    }

}
