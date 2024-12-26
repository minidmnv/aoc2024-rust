use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input, 71, 1024);
    part_two(input, 71, 1024);
}

#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
    value: i8,
}

#[derive(Debug, Clone)]
struct Graph {
    edges: HashMap<usize, HashSet<usize>>,
    nodes: HashMap<usize, Node>,
}

impl Graph {

    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, coords: (usize, usize), value: i8) -> usize {
        let current_id = self.nodes.len();

        self.nodes.insert(current_id, Node { x: coords.0, y: coords.1, value });

        current_id
    }

    pub fn add_edge(&mut self, from_id: usize, to_id: usize) {
        self.edges.entry(from_id).or_insert_with(HashSet::new).insert(to_id);
        self.edges.entry(to_id).or_insert_with(HashSet::new).insert(from_id);
    }

    pub fn dijkstra(&self, start: usize, end: usize) -> usize {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut distance: Vec<usize> = vec![usize::MAX; self.nodes.len()];
        let mut previous: Vec<usize>  = vec![0; self.nodes.len()];

        queue.push_back((start, 0));

        loop {
            if let Some((current_node, current_distance)) = queue.pop_front() {

                if current_node == end {
                    break;
                }

                if let Some(neighbors) = self.edges.get(&current_node) {
                    for &neighbour in neighbors {
                        let new_distance = current_distance + 1usize;

                        if new_distance < *distance.get(neighbour).expect("All distances are here") {
                            distance[neighbour] = new_distance;
                            previous[neighbour] = current_node;
                            queue.push_back((neighbour, new_distance));
                        }
                    }
                }
            } else {
               break;
            }
        }

        distance.get(end).expect("Must be end in costs").clone()
    }
}

#[measure_time]
pub fn part_one(input: &str, size: usize, time: usize) -> usize {
    let walls: Vec<(usize, usize)> = input.lines().take(time).enumerate().map(|(y, line)| {
        let split: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().expect("There are only numbers here")).collect();
        (split.get(0).expect("Must be x").clone(), split.get(1).expect("Must be y").clone())
    }).collect();

    let graph: Graph = parse_input(walls, size);

    let result = graph.dijkstra(0, size.pow(2) - 1);

    println!("Day 18, part 1 result: {:?}", result);

    result
}

fn parse_input(walls: Vec<(usize, usize)>, size: usize) -> Graph {
    let mut node_ids: HashMap<(usize, usize), usize> = HashMap::new();

    let mut graph: Graph = Graph::new();

    for i in 0..size {
        for j in 0..size {
            let val = if walls.contains(&(j, i)) { 0_i8 } else { 1_i8 };
            let node_id = graph.add_node((j, i), val);
            node_ids.insert((j, i), node_id);
        }
    }

    for i in 0..size {
        for j in 0..size {
            if !walls.contains(&(j, i)) {
                if let Some(&node_id) = node_ids.get(&(j, i)) {
                    if let Some(&left_id) = node_ids.get(&(j.wrapping_sub(1), i)) {
                        if !walls.contains(&(j.wrapping_sub(1), i)) {
                            graph.add_edge(node_id, left_id);
                        }
                    }

                    if let Some(&right_id) = node_ids.get(&(j + 1, i)) {
                        if !walls.contains(&(j + 1, i)) {
                            graph.add_edge(node_id, right_id);
                        }
                    }

                    if let Some(&top_id) = node_ids.get(&(j, i.wrapping_sub(1))) {
                        if !walls.contains(&(j, i.wrapping_sub(1))) {
                            graph.add_edge(node_id, top_id);
                        }
                    }

                    if let Some(&bottom_id) = node_ids.get(&(j, i + 1)) {
                        if !walls.contains(&(j, i + 1)) {
                            graph.add_edge(node_id, bottom_id);
                        }
                    }
                }
            }
        }
    }

    graph
}

#[measure_time]
pub fn part_two(input: &str, size: usize, time: usize) -> String {
    let mut result = 0;
    let walls: Vec<(usize, usize)> = input.lines().enumerate().map(|(y, line)| {
        let split: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().expect("There are only numbers here")).collect();
        (split.get(0).expect("Must be x").clone(), split.get(1).expect("Must be y").clone())
    }).collect();

    for i in 0..walls.len() - 1 {
        let graph: Graph = parse_input(walls.iter().take(time + i).cloned().collect(), size);

        if graph.dijkstra(0, size.pow(2) - 1) == usize::MAX {
            result = time + i - 1;
            break;
        }
    }


    println!("Day 18, part 2 result: {:?}", walls.get(result));

    walls.get(result).map(|(x, y)| [x.to_string(), y.to_string()].join(",")).take().expect("Must be the result")
}

#[cfg(test)]
mod day18 {
    use crate::day18::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1, 7, 12), 22)
    }

    #[test]
    fn test_part_one_case_two() {
        assert_eq!(part_two(TEST_CASE_1, 7, 12), "6,1")
    }
}
