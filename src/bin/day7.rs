use std::fs;
use nom::{
    bytes::complete::tag,
    character::{complete::{anychar, digit1}},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug)]
struct Command {
    line: String,
}

impl From<&str> for Command {
    fn from(str: &str) -> Self {
        Self { line: String::from(str) }
    }
}

fn parse_line(line: &str) -> Command {
    Command { line: String::from(line) }
}

fn parse_indata(indata: &str) -> Vec<Command> {
    indata
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn main() {
    let indata = fs::read_to_string("data/day7.txt").expect("No indata");
    let commands = parse_indata(&indata);
    println!("{:?}", &commands);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_example() {
        let test_data = indoc! {r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "#
        };
        let indata = fs::read_to_string("data/day7.txt").expect("No indata");
        let commands = parse_indata(&indata);
    
    }
}
