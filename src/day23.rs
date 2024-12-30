use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use measure_time_macro::measure_time;

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}

#[measure_time]
pub fn part_one(input: &str) -> usize {
    let computers_map: HashMap<&str, HashSet<&str>> = parse_input(input);
    let mut interconnected_computers: HashSet<Vec<&&str>> = HashSet::new();

    computers_map
        .keys()
        .filter(|computer| computer.starts_with("t"))
        .for_each(|computer| {
            for peer in computers_map.get(*computer).unwrap() {
                for peer_other in computers_map.get(*computer).unwrap() {
                    if peer == peer_other { continue; }

                    if computers_map.get(peer).unwrap().contains(peer_other) {
                        let mut computer_set = Vec::from([computer, peer, peer_other]);
                        computer_set.sort();
                        interconnected_computers.insert(computer_set);
                    }
                }
            }
        });
    println!("{:?}", interconnected_computers);
    println!("Day 23, part 1 result: {:?}", interconnected_computers.len());

    interconnected_computers.len()
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut result: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.lines().map(|line| line.trim()).for_each(|line| {
        let mut splitted = line.split("-");
        let comp_a = splitted.next().unwrap();
        let comp_b = splitted.next().unwrap();
        if let Some(computer) = result.get_mut(comp_a) {
            computer.insert(comp_b);
        } else {
            result.insert(comp_a, HashSet::from([comp_b]));
        }
        if let Some(computer) = result.get_mut(comp_b) {
            computer.insert(comp_a);
        } else {
            result.insert(comp_b, HashSet::from([comp_a]));
        }
    });

    result
}

#[measure_time]
pub fn part_two(input: &str) -> String {
    let computers_map: HashMap<&str, HashSet<&str>> = parse_input(input);
    let mut interconnected_computers: HashMap<&str, Vec<&str>> = HashMap::new();

    computers_map
        .keys()
        .filter(|computer| computer.starts_with("t"))
        .for_each(|computer| {
            interconnected_computers.insert(computer, Vec::from([*computer]));

            let peers = computers_map.get(*computer).unwrap();
            let mut peers_map: HashMap<&str, usize> = HashMap::new();

            for peer in peers {
                let mut result = 0;
                for interconnected_computer in peers {
                    if peer == interconnected_computer { continue; }

                    let connected_computers = computers_map.get(*interconnected_computer).unwrap();
                    if !connected_computers.contains(peer) { continue;}
                    result += 1;
                }

                //dodaj tego peera do listy
                peers_map.insert(peer, result);
            }

            println!("{}, peers: {:?}, peers_map: {:?}", computer, peers, peers_map);

            let sorted_peers = peers_map.iter().sorted_by(|a, b| a.1.cmp(b.1).reverse());
            'outer: for (peer, _) in sorted_peers {
                for interconnected_computer in interconnected_computers.get(computer).unwrap() {
                    if peer == interconnected_computer { continue 'outer; }

                    let connected_computers = computers_map.get(*interconnected_computer).unwrap();
                    if !connected_computers.contains(peer) { continue 'outer;}
                }

                let mut comps = interconnected_computers.get_mut(computer).unwrap();
                comps.push(peer.clone());
                comps.sort();
            }

        });

    println!("Long values: {:?}", interconnected_computers.values().filter(|vec| vec.len() > 13));
    let result = interconnected_computers.values().max_by(|a, b| a.len().partial_cmp(&b.len()).unwrap()).unwrap().into_iter().join(",");
    println!("Day 23, part 2 result: {:?}", result);

    result
}

#[cfg(test)]
mod day23 {
    use crate::day23::*;
    use std::assert_eq;

    const TEST_CASE_1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_one_case_one() {
        assert_eq!(part_one(TEST_CASE_1), 7)
    }

    #[test]
    fn test_part_two_case_one() {
        assert_eq!(part_two(TEST_CASE_1), "co,de,ka,ta")
    }

}
