use std::{fs, collections::HashSet, hash::Hash};
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Command {
    Vert(i32),
    Horiz(i32),
}
fn parse_indata(indata: &str) -> Vec<Command> {
    Regex::new(r"(\w) (\d+)")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            let delta: i32 = cap[2].parse().unwrap();
            match &cap[1] {
                "U" => Command::Vert(delta),
                "D" => Command::Vert(-delta),
                "L" => Command::Horiz(-delta),
                "R" => Command::Horiz(delta),
                _ => panic!("Not allowed"),
            }
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn play_commands(commands: &Vec<Command>) -> HashSet<Coord> {
    let mut res: HashSet<Coord> = HashSet::new();
    let mut head = Coord {x: 0, y:0};
    let mut tail = Coord {x: 0, y:0};
    for cmd in commands {
        match cmd {
            Command::Vert(nr) => {
                for _ in 0..nr.abs() {
                    let oldy = head.y;
                    head.y = head.y + nr.signum();
                    if (head.y - tail.y).abs() > 1 {
                        tail.y = oldy;
                        tail.x = head.x;
                    }
                    res.insert(tail.clone());
                }
            },
            Command::Horiz(nr) => {
                for _ in 0..nr.abs() {
                    let oldx = head.x;
                    head.x = head.x + nr.signum();
                    if (head.x - tail.x).abs() > 1 {
                        tail.x = oldx;
                        tail.y = head.y;
                    }
                    res.insert(tail.clone());
                }
            },
        };
    }
    res
}
fn main() {
    let indata = fs::read_to_string("data/day9.txt").expect("No indata");
    let commands = parse_indata(&indata);
    let res = play_commands(&commands);

    println!("Part1: {:?}", res.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
    "#
    };

    #[test]
    fn test_part1() {
        let commands = parse_indata(&TEST_DATA);
        let res = play_commands(&commands);
        assert_eq!(13, res.len());
    }

}
