use std::collections::{HashMap, HashSet, VecDeque};
use measure_time_macro::measure_time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Side {
    direction: Direction,
    fends: Vec<(usize, usize)>
}

impl Side {
    fn new(direction: Direction, coords: (usize, usize)) -> Self {
        let mut fends: Vec<(usize, usize)> = Vec::new();
        fends.push(coords);
        Self { direction, fends }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: i32,
    value: char,
    coords: (usize, usize),
}

impl Node {
    fn new(id: i32, value: char, coords: (usize, usize)) -> Self {
        Self { id, value, coords }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    adj_list: HashMap<i32, HashSet<i32>>,
    nodes: HashMap<i32, Node>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: i32, value: char, coords: (usize, usize)) {
        self.nodes.insert(id, Node::new(id, value, coords));
    }

    fn add_edge(&mut self, from_id: i32, to_id: i32) {
        self.adj_list.entry(from_id).or_insert_with(HashSet::new).insert(to_id);
        self.adj_list.entry(to_id).or_insert_with(HashSet::new).insert(from_id);
    }

    pub fn get_region_with_perimeters(&self, start_id: i32) -> (HashSet<i32>, i32) {
        let mut queue = VecDeque::new();

        let mut region_area: HashSet<i32> = HashSet::new();
        let mut area_coords: Vec<(usize, usize)> = Vec::new();
        let mut region_perimeter: i32 = 0;

        if let Some(start_node) = self.nodes.get(&start_id) {
            queue.push_back(start_node.clone());
            region_area.insert(start_node.id);
            area_coords.push(start_node.coords);
        }

        while let Some(current_node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&current_node.id) {
                for &neighbor_id in neighbors {
                    if let Some(neighbor_node) = self.nodes.get(&neighbor_id) {
                        if !region_area.contains(&neighbor_node.id) && neighbor_node.value == current_node.value
                        {
                            region_area.insert(neighbor_node.id);
                            area_coords.push(neighbor_node.coords);

                            queue.push_back(neighbor_node.clone());
                        }
                    }
                }
            }
        }

        self.count_region_perimeters(&mut region_area, &mut area_coords, &mut region_perimeter);


        (region_area, region_perimeter)
    }

    pub fn get_region_with_sides(&self, start_id: i32) -> (HashSet<i32>, i32) {
        let mut queue = VecDeque::new();

        let mut region_area: HashSet<i32> = HashSet::new();
        let mut area_coords: Vec<(usize, usize)> = Vec::new();
        let mut region_sides: Vec<Vec<(usize, usize, Direction)>> = Vec::new();

        if let Some(start_node) = self.nodes.get(&start_id) {
            queue.push_back(start_node.clone());
            region_area.insert(start_node.id);
            area_coords.push(start_node.coords);
        }

        while let Some(current_node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&current_node.id) {
                for &neighbor_id in neighbors {
                    if let Some(neighbor_node) = self.nodes.get(&neighbor_id) {
                        if !region_area.contains(&neighbor_node.id) && neighbor_node.value == current_node.value
                        {
                            region_area.insert(neighbor_node.id);
                            area_coords.push(neighbor_node.coords);

                            queue.push_back(neighbor_node.clone());
                        }
                    }
                }
            }
        }

        self.count_region_sides(&mut region_area, &mut area_coords, &mut region_sides);


        (region_area, region_sides.len() as i32)
    }

    // count region perimeter
    fn count_region_perimeters(&self, region_area: &mut HashSet<i32>, area_coords: &mut Vec<(usize, usize)>, region_perimeter: &mut i32) {
        for node_id in region_area.clone() {
            if let Some(node) = self.nodes.get(&node_id) {
                let starting_coords = node.coords.clone();
                if let None = area_coords.iter().find(|node_coords| starting_coords.1 != 0 && node_coords.0 == starting_coords.0 && node_coords.1 == starting_coords.1 - 1) {
                    *region_perimeter += 1; // top
                }
                if let None = area_coords.iter().find(|node_coords| node_coords.0 == starting_coords.0 && node_coords.1 == starting_coords.1.saturating_add(1)) {
                    *region_perimeter += 1; // down
                }
                if let None = area_coords.iter().find(|node_coords| starting_coords.0 != 0 && node_coords.0 == starting_coords.0 - 1 && node_coords.1 == starting_coords.1) {
                    *region_perimeter += 1; // left
                }
                if let None = area_coords.iter().find(|node_coords| node_coords.0 == starting_coords.0.saturating_add(1) && node_coords.1 == starting_coords.1) {
                    *region_perimeter += 1; // right
                }
            }
        }
    }

    fn count_region_sides(&self, region_area: &mut HashSet<i32>, area_coords: &mut Vec<(usize, usize)>, region_sides: &mut Vec<Vec<(usize, usize, Direction)>>) {
        //TODO: implement
        for node_id in region_area.clone() {
            if let Some(node) = self.nodes.get(&node_id) {
                let starting_coords = node.coords.clone();
                if let None = area_coords.iter().find(|node_coords| starting_coords.1 != 0 && node_coords.0 == starting_coords.0 && node_coords.1 == starting_coords.1 - 1) {
                    *region_perimeter += 1; // top
                }
                if let None = area_coords.iter().find(|node_coords| node_coords.0 == starting_coords.0 && node_coords.1 == starting_coords.1.saturating_add(1)) {
                    *region_perimeter += 1; // down
                }
                if let None = area_coords.iter().find(|node_coords| starting_coords.0 != 0 && node_coords.0 == starting_coords.0 - 1 && node_coords.1 == starting_coords.1) {
                    *region_perimeter += 1; // left
                }
                if let None = area_coords.iter().find(|node_coords| node_coords.0 == starting_coords.0.saturating_add(1) && node_coords.1 == starting_coords.1) {
                    *region_perimeter += 1; // right
                }
            }
        }
    }
}

fn create_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    let mut node_ids = HashMap::new();
    let mut current_id = 0;

    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect();

    for (row, line) in lines.iter().enumerate() {
        for (col, &value) in line.iter().enumerate() {
            graph.add_node(current_id, value, (col, row));
            node_ids.insert((row, col), current_id);
            current_id += 1;
        }
    }

    for (row, line) in lines.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            if let Some(&node_id) = node_ids.get(&(row, col)) {
                if let Some(&top_id) = node_ids.get(&(row.wrapping_sub(1), col)) {
                    graph.add_edge(node_id, top_id);
                }

                if let Some(&bottom_id) = node_ids.get(&(row + 1, col)) {
                    graph.add_edge(node_id, bottom_id);
                }

                if let Some(&left_id) = node_ids.get(&(row, col.wrapping_sub(1))) {
                    graph.add_edge(node_id, left_id);
                }

                if let Some(&right_id) = node_ids.get(&(row, col + 1)) {
                    graph.add_edge(node_id, right_id);
                }
            }
        }
    }

    graph
}

pub fn run(input: &str) {
    let grid = create_graph(input);

    part_one(grid.clone());
    part_two(grid.clone());
}

#[measure_time]
fn part_one(grid: Graph) {
    let mut counted_nodes: Vec<i32> = Vec::new();

    let result: i32 = grid.nodes.iter().map(|node| {
        if !counted_nodes.contains(node.0) {
            let (region_area, region_perimeters) = grid.get_region_with_perimeters(*node.0);
            counted_nodes.extend(region_area.clone());

            let res = region_area.len() as i32 * region_perimeters;
            println!("{}, {:?}, {} : {}", node.1.value, node.1.coords, region_area.len(), res);
            return res;
        }

        0
    }).sum();

    println!("Day 12, part 1 result: {:?}", result);

    // 1120660 is too low
    // 1121380 is too low      XX                  8 * 3
    // 1140736 is too low       X
    // 1451416 wrong
}

#[measure_time]
fn part_two(_grid: Graph) {

    println!("Day 12, part 2 result: {}", false);
}
