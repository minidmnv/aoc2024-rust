use std::collections::{HashMap, HashSet, VecDeque};
use measure_time_macro::measure_time;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: i32,
    value: i32,
}

impl Node {
    fn new(id: i32, value: i32) -> Self {
        Self { id, value }
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

    fn add_node(&mut self, id: i32, value: i32) {
        self.nodes.insert(id, Node::new(id, value));
    }

    fn add_edge(&mut self, from_id: i32, to_id: i32) {
        self.adj_list.entry(from_id).or_insert_with(HashSet::new).insert(to_id);
        self.adj_list.entry(to_id).or_insert_with(HashSet::new).insert(from_id);
    }

    pub fn list_start_nodes(&self) -> Vec<i32> {
        self.nodes.keys().filter(|key| self.nodes.get(key).unwrap().value == 0).cloned().collect::<Vec<i32>>()
    }

    pub fn get_possible_trailheads(&self, start_id: i32) -> HashSet<i32> {
        let mut queue = VecDeque::new();
        let mut peaks: HashSet<i32> = HashSet::new();

        if let Some(start_node) = self.nodes.get(&start_id) {
            queue.push_back(start_node.clone());
        }

        while let Some(current_node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&current_node.id) {
                for &neighbor_id in neighbors {
                    if let Some(neighbor_node) = self.nodes.get(&neighbor_id) {
                        if neighbor_node.value - current_node.value == 1
                        {
                            if neighbor_node.value == 9 {
                                peaks.insert(neighbor_node.id);
                            } else {
                                queue.push_back(neighbor_node.clone());
                            }
                        }
                    }
                }
            }
        }

        peaks
    }

    pub fn get_unique_trailheads(&self, start_id: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut result: i32 = 0;

        if let Some(start_node) = self.nodes.get(&start_id) {
            queue.push_back(start_node.clone());
        }

        while let Some(current_node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&current_node.id) {
                for &neighbor_id in neighbors {
                    if let Some(neighbor_node) = self.nodes.get(&neighbor_id) {
                        if neighbor_node.value - current_node.value == 1
                        {
                            if neighbor_node.value == 9 {
                                result += 1;
                            } else {
                                queue.push_back(neighbor_node.clone());
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

fn create_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    let mut node_ids = HashMap::new();
    let mut current_id = 0;

    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect()
        })
        .collect();

    for (row, line) in lines.iter().enumerate() {
        for (col, &value) in line.iter().enumerate() {
            graph.add_node(current_id, value);
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
    let graph = create_graph(input);

    part_one(graph.clone());
    part_two(graph.clone());
}

#[measure_time]
fn part_one(graph: Graph) {
    let result: i32 = graph.list_start_nodes().iter().map(|start_node| graph.get_possible_trailheads(*start_node).len() as i32).sum();

    println!("Day 10, part 1 result: {:?}", result);
}

#[measure_time]
fn part_two(graph: Graph) {
    let result: i32 = graph.list_start_nodes().iter().map(|start_node| graph.get_unique_trailheads(*start_node)).sum();

    println!("Day 10, part 2 result: {}", result);
}
