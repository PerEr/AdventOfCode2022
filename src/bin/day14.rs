use std::{fs, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1},
    IResult, combinator::{map_res}, sequence::{tuple}, multi::{separated_list0, many0}, And,
};

#[derive(Debug, PartialEq)]
enum Content {
    Rock,
    Falling,
    Settled,
}
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


fn build_cave(lst: &Vec<Vec<(i32,i32)>>) -> HashMap<(i32,i32), Content> {
    let mut cave = HashMap::new();
    for l in lst {
        for ss in l.windows(2) {
            let mut x = ss[0].0;
            let mut y = ss[0].1;
            let dx = (ss[1].0 - x).signum();
            let dy = (ss[1].1 - y).signum();
            assert!(dx == 0 || dy == 0);
            loop {
                cave.insert((x,y), Content::Rock);
                if x == ss[1].0 && y == ss[1].1 {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
    }
    cave
}

fn draw_cave(cave: &HashMap<(i32,i32), Content>) -> Vec<String> {
    let mut res = Vec::new();
    let mut topLeft = (500, 0);
    let mut bottomRight = (500, 0);
    for p in cave.keys() {
        topLeft = (topLeft.0.min(p.0), topLeft.1.min(p.1));
        bottomRight = (bottomRight.0.max(p.0), bottomRight.1.max(p.1));
    }

    println!("topleft     {:?}", topLeft);
    println!("bottomRight {:?}", bottomRight);
    
    for y in topLeft.1..=bottomRight.1 {
        let mut line = String::from("");
        for x in topLeft.0..=bottomRight.0 {
            line.push(match cave.get(&(x,y)) {
                Some(i) if *i == Content::Rock => '#',
                _ => ' ',
            });
        }
        res.push(line);
    }
    res
}




fn main() {
    let indata = fs::read_to_string("data/day14.txt").expect("No indata");
    let res = parse_indata(&indata).ok().unwrap().1;
    assert_eq!(179, res.len());

    let cave = build_cave(&res);
    for l in draw_cave(&cave) {
        println!("{:?}", l);
    }
}

// https://github.com/tumdum/aoc2022/blob/main/src/day12.rs
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";

    #[test]
    fn test_part1() {
        let res = parse_indata(&TEST_DATA).ok().unwrap().1;
        assert_eq!(2, res.len());
        assert_eq!(vec!((498,4),(498,6),(496,6)), res[0]);
        assert_eq!(vec!((503,4),(502,4),(502,9),(494,9)), res[1]);

        let cave = build_cave(&res);
        assert_eq!(20, cave.iter().count());

        draw_cave(&cave);
    }
}
