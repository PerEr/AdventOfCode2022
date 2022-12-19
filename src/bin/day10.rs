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
                "noop" => vec!(Command::Noop),
                "addx" => {         
                    vec!(Command::Noop, Command::AddX(cap[2].trim().parse().unwrap()))}  
                    ,
                _ => panic!("Not allowed"),
            }
        })
        .flatten()
        .collect()
}


fn play_commands(commands: &Vec<Command>) -> Vec<i32> {
    let mut x = 1;
    let mut rep = Vec::new();
    for cmd in commands {
        match cmd {
            Command::Noop => {
            },
            Command::AddX(nr) => {
                x = x + nr;
            },
        };
        rep.push(x);
    }
    rep
}

fn signal_strength(xs: &Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..=5 {
        let index = 40*i+20;
        let value = xs[index-2];
        res = res + value * (index as i32);
    }
    res
}

fn render_screen(xs: &Vec<i32>) -> Vec<String> {
    let mut res: Vec<String> = vec!();
    let mut line = String::new();
    let mut col = 0;
    let mut nextx = 1;
    for x in xs {
        let c: char = if col >= nextx-1 && col <= nextx+1 { '#' } else { '.' };
        line.push(c);
        col += 1;
        if col == 40 {
            col = 0;
            res.push(line.clone());
            line.clear();
        }
        nextx = x.clone();
    }
    res
}

fn main() {
    let indata = fs::read_to_string("data/day10.txt").expect("No indata");
    let commands = parse_indata(&indata);
    let xs = play_commands(&commands);
    println!("Part1: {:?}", signal_strength(&xs));
    println!("Part2:");
    for l in render_screen(&xs) {
        println!("{l}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;


    const TEST_DATA: &'static str = indoc! {r#"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
        "#
    };

    #[test]
    fn test_part1() {
        let commands = parse_indata(&TEST_DATA);
        let xs = play_commands(&commands);
        let res = signal_strength(&xs);
        assert_eq!(13140, res);
    }

    #[test]
    fn test_part2() {
        let commands = parse_indata(&TEST_DATA);
        let xs = play_commands(&commands);
        let screen = render_screen(&xs);

        let test_data: &'static str = indoc! {r#"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######....."#
        };

        assert_eq!(test_data, screen.join("\n"));
    }

}
