use std::{fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1},
    IResult, combinator::{map_res}, sequence::delimited, multi::{separated_list0},
};

#[derive(Debug)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

fn parse_integer(input: &str) -> IResult<&str, Item> {
    let parse = |s: &str| match s.parse() {
        Ok(n) => Result::Ok::<Item, String>(Item::Int(n)),
        Err(n) => Result::Err(n.to_string()),
    };
    map_res(digit1, parse)(input)
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    let res = delimited(tag("["),
    separated_list0(tag(","), alt((parse_integer, parse_list))),
    tag("]"))(input);
    match res {
        Ok(r) => IResult::Ok((r.0, Item::List(r.1))),
        Err(s) => IResult::Err(s),
    }
}

fn parse_indata(indata: &str) -> Vec<Vec<Item>> {
    indata.split("\n\n")
        .map(|t| {
        t.split("\n").map(|l| {
            match parse_list(l) {
                Ok(r) => r.1,
                Err(_n) => panic!("cant happen"),
            }
    }).collect()
    }).collect()
}

fn main() {
    let indata = fs::read_to_string("data/day13.txt").expect("No indata");
    let res = parse_indata(&indata);
    for pair in res {
        assert_eq!(2, pair.len());
        println!("first: {:?}", pair[0]);
        println!("second: {:?}", pair[1]);
    }

}

// https://github.com/tumdum/aoc2022/blob/main/src/day12.rs
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    [1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]"#
    };

    #[test]
    fn test_part1() {
        let res = parse_indata(&TEST_DATA);
        assert_eq!(8, res.len());
    }


}
