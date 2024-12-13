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

        self.count_region_sides(&mut region_area, &mut area_coords);


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

    fn count_region_sides(&self, region_area: &mut HashSet<i32>, area_coords: &mut Vec<(usize, usize)> ) -> Vec<Vec<(usize, usize, Direction)>> {

        let mut region_sides: Vec<Vec<(usize, usize, Direction)>> = Vec::new();

        //TODO: implement corners and fences
        for node_id in region_area.clone() {
            if let Some(node) = self.nodes.get(&node_id) {
                let starting_coords = node.coords.clone();
                if let None = area_coords.iter().find(|node_coords| starting_coords.1 != 0 && node_coords.0 == starting_coords.0 && node_coords.1 == starting_coords.1 - 1) {
                    // *region_perimeter += 1; // top
                }
                if let None = area_coords.iter().find(|node_coords| node_coords.0 == starting_coords.0 && node_coords.1 == starting_coords.1.saturating_add(1)) {
                    // *region_perimeter += 1; // down
                }
                if let None = area_coords.iter().find(|node_coords| starting_coords.0 != 0 && node_coords.0 == starting_coords.0 - 1 && node_coords.1 == starting_coords.1) {
                    // *region_perimeter += 1; // left
                }
                if let None = area_coords.iter().find(|node_coords| node_coords.0 == starting_coords.0.saturating_add(1) && node_coords.1 == starting_coords.1) {
                    // *region_perimeter += 1; // right
                }
            }
        }

        region_sides
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
    let garden_graph = create_graph(input);

    part_one(garden_graph.clone());
    part_two(input);
}

#[measure_time]
fn part_one(garden_graph: Graph) {
    let mut counted_nodes: Vec<i32> = Vec::new();

    let result: i32 = garden_graph.nodes.iter().map(|node| {
        if !counted_nodes.contains(node.0) {
            let (region_area, region_perimeters) = garden_graph.get_region_with_perimeters(*node.0);
            counted_nodes.extend(region_area.clone());

            let res = region_area.len() as i32 * region_perimeters;
            println!("{}, {:?}, {} : {}", node.1.value, node.1.coords, region_area.len(), res);
            return res;
        }

        0
    }).sum();

    println!("Day 12, part 1 result: {:?}", result);
}

fn is_valid(p: (i32, i32), max_x: i32, max_y: i32) -> bool {
    let (x, y) = p;
    0 <= x && x < max_x && 0 <= y && y < max_y
}

fn get_plant_area(grid: &Vec<Vec<char>>, p: (i32, i32)) -> HashSet<(i32, i32)> {
    let garden_width = grid[0].len() as i32;
    let garden_height = grid.len() as i32;

    let (start_x, start_y) = p;
    let plant = grid[start_y as usize][start_x as usize];

    let mut visited = HashSet::new();
    let mut to_visit = vec![p];

    while let Some((x, y)) = to_visit.pop() {
        visited.insert((x, y));

        for (step_x, step_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_x = x + step_x;
            let next_y = y + step_y;
            if is_valid((next_x, next_y), garden_width, garden_height)
                && grid[next_y as usize][next_x as usize] == plant
                && !visited.contains(&(next_x, next_y))
            {
                to_visit.push((next_x, next_y));
            }
        }
    }

    visited
}

fn get_region_boundaries(points: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    points
        .iter()
        .flat_map(|(x, y)| {
            deltas
                .iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .filter(|new_p| !points.contains(new_p))
        })
        .collect()
}

fn count_corners(region: &HashSet<(i32, i32)>) -> usize {
    let mut corners = HashSet::new();
    let possible_corners = [
        [(-1, 0), (0, -1), (-1, -1)],
        [(1, 0), (0, -1), (1, -1)],
        [(-1, 0), (0, 1), (-1, 1)],
        [(1, 0), (0, 1), (1, 1)],
    ];

    for &(x, y) in region {
        for (i, corner) in possible_corners.iter().enumerate() {
            let vals: Vec<(i32, i32)> = corner.iter().map(|(corner_x, corner_y)| (x + corner_x, y + corner_y)).collect();
            if vals.iter().all(|coords| !region.contains(coords)) {
                corners.insert((x, y, i));
            }
        }
    }

    let possible_inner_corners = [
        [(-1, 0), (0, -1)],
        [(-1, 0), (0, 1)],
        [(1, 0), (0, -1)],
        [(1, 0), (0, 1)],
    ];
    let mut inner_corners = HashSet::new();

    for &(x, y) in &get_region_boundaries(region) {
        for (i, corner) in possible_inner_corners.iter().enumerate() {
            let vals: Vec<(i32, i32)> = corner.iter().map(|(corner_x, corner_y)| (x + corner_x, y + corner_y)).collect();
            if vals.iter().all(|coords| region.contains(coords)) {
                let (dx, dy) = (corner[0].0 + corner[1].0, corner[0].1 + corner[1].1);
                if region.contains(&(x + dx, y + dy)) {
                    inner_corners.insert((x + dx, y + dy, i));
                } else {
                    let ((v1x, v1y), (v2x, v2y)) = (vals[0], vals[1]);
                    let (dx, dy) = (v1x - v2x, v1y - v2y);
                    let d1 = [(-dx, 0), (0, dy)];
                    let d2 = [(dx, 0), (0, -dy)];

                    inner_corners.insert((
                        v1x,
                        v1y,
                        possible_inner_corners.iter().position(|&x| x == d1).unwrap(),
                    ));
                    inner_corners.insert((
                        v2x,
                        v2y,
                        possible_inner_corners.iter().position(|&x| x == d2).unwrap(),
                    ));
                }
            }
        }
    }

    corners.len() + inner_corners.len()
}

#[measure_time]
fn part_two(input: &str) {

    let garden: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect();

    let mut crops: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut visited = HashSet::new();
    for (y, row) in garden.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let p = (x as i32, y as i32);
            if !visited.contains(&p) {
                let plant_area = get_plant_area(&garden, p);

                crops.push(plant_area.clone());
                visited.extend(plant_area);
            }
        }
    }

    let result: usize = crops.iter().map(|v| v.len() * count_corners(v)).sum();
    println!("Day 12, part 2 result: {}", result);
}
