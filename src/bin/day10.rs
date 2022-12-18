use std::{fs};
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Command {
    Noop,
    AddX(i32),
}
fn parse_indata(indata: &str) -> Vec<Command> {
    Regex::new(r"(noop|addx)( [-]?\d+)?")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            match &cap[1] {
                "noop" => Command::Noop,
                "addx" => {           
                    Command::AddX(cap[2].trim().parse().unwrap())},
                _ => panic!("Not allowed"),
            }
        })
        .collect()
}


fn play_commands(commands: &Vec<Command>)  {
    for cmd in commands {
        match cmd {
            Command::Noop => {
            },
            Command::AddX(_nr) => {
            },
        };
    }
}
fn main() {
    let indata = fs::read_to_string("data/day10.txt").expect("No indata");
    let commands = parse_indata(&indata);
    play_commands(&commands);
    println!("Part1: {:?}", commands);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;


    #[test]
    fn test_part1() {
        let test_data: &'static str = indoc! {r#"
        noop
        addx 3
        addx -5
        "#
        };
        let commands = parse_indata(&test_data);
        play_commands(&commands);
    }


}
