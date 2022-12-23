use std::{fs};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1},
    IResult, combinator::{map_res}, sequence::{tuple}, multi::{separated_list0, many0},
};

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_tuple(input: &str) -> IResult<&str, (i32,i32)> {
    let r = tuple((parse_integer, tag(","), parse_integer))(input);
    match r {
        Ok((s,val)) => IResult::Ok((s, (val.0, val.2))),
        Err(s) => IResult::Err(s),
    }
}

fn parse_indata(input: &str) -> IResult<&str, Vec<Vec<(i32, i32)>>> {
    match many0(
        tuple(
            (
                separated_list0(tag(" -> "), parse_tuple), 
                tag("\n")
            )
        )
    )(input) {
        Ok((str, val)) => IResult::Ok((str, val.into_iter().map(|v| v.0).collect())),
        Err(e) => IResult::Err(e),
    }
}

fn main() {
    let indata = fs::read_to_string("data/day14.txt").expect("No indata");
    let lst = parse_indata(&indata).ok().unwrap().1;
    println!("{}", lst.len());
}

// https://github.com/tumdum/aoc2022/blob/main/src/day12.rs
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
    "#};

    #[test]
    fn test_part1() {
        let res = parse_indata(&TEST_DATA);
        let lst = res.ok().unwrap().1;
        assert_eq!(2, lst.len());
        assert_eq!(vec!((498,4),(498,6),(496,6)), lst[0]);
        assert_eq!(vec!((503,4),(502,4),(502,9),(494,9)), lst[1]);
    }


}
