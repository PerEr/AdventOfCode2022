use std::{fs, collections::HashMap};
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    flow: i32,
    connections: Vec<String>,
}


fn parse_indata(indata: &str) -> HashMap<String, Valve> {
    Regex::new(r"Valve (\w+) .*=(\d+).*valve[s]* (.*)")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            let connections: Vec<String> = cap[3].split(",").map(|s| String::from(s.trim())).collect();
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
    println!("Part1: {:?}", valves);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
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
        let valves = parse_indata(&TEST_DATA);
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
