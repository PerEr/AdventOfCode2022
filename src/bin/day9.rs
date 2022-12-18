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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

fn next_coord(head: &Coord, tail: &Coord) -> Coord {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    let stay = dx.abs() <=1 && dy.abs() <= 1;

    return if stay {
        Coord { ..*tail }
    } else {
        Coord {x: tail.x + dx.signum(), y: tail.y + dy.signum() }
    }
}
fn modify_knots(nr: i32, knots: &mut Vec<Coord>, f: fn(c:Coord, nr: i32) -> Coord, res: &mut HashSet<Coord>) {
    for _ in 0..nr.abs() {
        knots[0] =f(knots[0], nr);
        for ix in 1..knots.len() {
            knots[ix] = next_coord(&knots[ix-1], &knots[ix]);
        }
        res.insert(knots[knots.len()-1].clone());
    }
}

fn play_commands(commands: &Vec<Command>, sz: usize) -> HashSet<Coord> {
    let mut res: HashSet<Coord> = HashSet::new();
    let mut knots: Vec<Coord> = Vec::new();
    knots.resize(sz, Coord {x:0, y:0});

    for cmd in commands {
        match cmd {
            Command::Vert(nr) => {
                modify_knots(*nr, &mut knots, |c, nr| 
                    Coord {x : c.x, y: c.y + nr.signum()}
                , &mut res);
            },
            Command::Horiz(nr) => {
                modify_knots(*nr, &mut knots, |c, nr| 
                    Coord {x : c.x  + nr.signum(), y: c.y}
                , &mut res);
            },
        };
    }
    res
}
fn main() {
    let indata = fs::read_to_string("data/day9.txt").expect("No indata");
    let commands = parse_indata(&indata);
    println!("Part1: {:?}", play_commands(&commands, 2).len());
    println!("Part2: {:?}", play_commands(&commands, 10).len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;


    #[test]
    fn test_part1() {
        let test_data: &'static str = indoc! {r#"
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
            let commands = parse_indata(&test_data);
        let res = play_commands(&commands, 2);
        assert_eq!(13, res.len());
    }

    #[test]
    fn test_part2() {
        let test_data: &'static str = indoc! {r#"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
        "#
        };
        let commands = parse_indata(&test_data);
        let res = play_commands(&commands, 10);
        assert_eq!(36, res.len());
    }

}
