use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use std::fs;

fn parse_crates(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(
        tag(" "),
        map(delimited(tag("["), anychar, tag("]")), Some).or(map(tag("   "), |_t: &str| None)),
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |num: &str| num.parse())(input)
}

fn parse_command(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag("move ")(input)?;
    let (input, nr) = parse_number(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = parse_number(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = parse_number(input)?;

    Ok((input, (nr, from, to)))
}
fn parse_indata<'a>(indata: &'a str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut it = indata.split("\n").filter(|l| !l.is_empty());
    let mut stacks = vec![];
    while let Ok((_, row_result)) = parse_crates(&it.next().unwrap()) {
        if stacks.is_empty() {
            stacks = std::iter::repeat(vec![]).take(row_result.len()).collect();
        }
        for (ix, res) in row_result.into_iter().enumerate() {
            if let Some(crte) = res {
                stacks[ix].push(crte);
            }
        }
    }

    let commands: Vec<(usize, usize, usize)> = it
        .map(|input| {
            let (_, command) = parse_command(&input).unwrap();
            command
        })
        .collect();

    (
        stacks
            .into_iter()
            .map(|v| v.into_iter().rev().collect())
            .collect(),
        commands,
    )
}

fn main() {
    let indata = fs::read_to_string("data/day5.txt").expect("No indata");
    let (stack, moves) = parse_indata(&indata);
    println!("Part1: {:?}", stack);
    // println!("Part2: {:?}", process(&data, some_overlap));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
        [D]    
    [N] [C]    
    [Z] [M] [P]
    1   2   3 

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "#
    };

    #[test]
    fn test_part1() {
        let (_stack, _moves) = parse_indata(&TEST_DATA);
        let x = 1;
    }

    #[test]
    fn test_part2() {}
}
