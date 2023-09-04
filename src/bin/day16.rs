use std::{fs, collections::{HashMap, BinaryHeap, HashSet}, cmp::Ordering};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    flow: i32,
    connections: Vec<String>,
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: i32,
    curr: String,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_cost(from: &String, to: &String, valves: &HashMap<String, Valve>) -> i32 {
    let mut prio_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    prio_queue.push(Node {
        cost: 0,
        curr: from.clone(),
    });
    visited.insert(from);

    while let Some(Node { cost, curr }) = prio_queue.pop() {
        if &curr == to {
            return cost;
        }

        for node_id in valves[&curr].connections.iter() {
            if visited.insert(node_id) {
                prio_queue.push(Node {
                    cost: cost + 1,
                    curr: node_id.clone(),
                });
            }
        }
    }
    i32::MAX
}

fn calculate_distances(start: &String, valves: &HashMap<String, Valve>) -> HashMap<(String, String), i32> {
    valves.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry((start.clone(), name1.clone()))
                .or_insert_with(|| calculate_cost(start, name1, valves));
            acc.entry((start.clone(), name2.clone()))
                .or_insert_with(|| calculate_cost(start, name2, valves));
            let dist = calculate_cost(name1, name2, valves);
            acc.insert((name1.clone(), name2.clone()), dist);
            acc.insert((name2.clone(), name1.clone()), dist);
            acc
        })
}
fn parse_indata(indata: &str) -> HashMap<String, Valve> {
    Regex::new(r"Valve (\w+) .*=(\d+).*valve[s]* (.*)")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            let connections: Vec<String> = cap[3].split(',').map(|s| String::from(s.trim())).collect();
            (String::from(&cap[1]), Valve {
                flow: cap[2].parse().unwrap(),
                connections,
         })
        })
        .collect::<HashMap<String, Valve>>()
}



fn main() {
    let indata = fs::read_to_string("data/day16.txt").expect("No indata");
    let valves = parse_indata(&indata);
    assert_eq!(51, valves.len());
    let dists = calculate_distances(&String::from("AA"), &valves);
    println!("Part1: {:?}", valves);
    println!("Part1: {:?}", dists);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &str = indoc! {r#"
    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    "#
    };

       
    #[test]
    fn test_part1() {
        let valves = parse_indata(TEST_DATA);
        assert_eq!(10, valves.len());
        assert_eq!(&Valve { 
            flow: 0,
            connections: vec![String::from("DD"), String::from("II"), String::from("BB")],
        }, valves.get(&String::from("AA")).unwrap());
        assert_eq!(&Valve { 
            flow: 21,
            connections: vec![String::from("II")],
        }, valves.get(&String::from("JJ")).unwrap());
    }

}
