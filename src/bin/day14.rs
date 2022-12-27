use std::{fs, collections::HashMap };

use nom::{
    bytes::complete::tag,
    character::complete::{digit1},
    IResult, combinator::{map_res}, sequence::{tuple}, multi::{separated_list0, many0},
};

#[derive(Debug, PartialEq)]
enum Content {
    Rock,
    Sand,
}

struct Cave {
    y_max: i32,
    data: HashMap<(i32,i32), Content>,
}

impl Cave {
    fn get(&self, p: &(i32, i32)) -> Option<&Content> {
        if p.1 < self.y_max + 2 { self.data.get(p) } else { Some(&Content::Rock) }
    }

    fn from(lst: &Vec<Vec<(i32,i32)>>) -> Self {
        let mut data = HashMap::new();
        for l in lst {
            for ss in l.windows(2) {
                let mut x = ss[0].0;
                let mut y = ss[0].1;
                let dx = (ss[1].0 - x).signum();
                let dy = (ss[1].1 - y).signum();
                assert!(dx == 0 || dy == 0);
                loop {
                    data.insert((x,y), Content::Rock);
                    if x == ss[1].0 && y == ss[1].1 {
                        break;
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
        let mut y_max = START.1;
        for p in data.keys() {
            y_max = y_max.max(p.1);
        }
    
        Self {y_max, data}
    }
}

const START: (i32, i32) = (500, 0);

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




fn draw_cave(cave: &Cave) -> Vec<String> {
    let mut res = Vec::new();
    let mut top_left = START;
    let mut bottom_right = START;
    for p in cave.data.keys() {
        top_left = (top_left.0.min(p.0), top_left.1.min(p.1));
        bottom_right = (bottom_right.0.max(p.0), bottom_right.1.max(p.1));
    }

    for y in top_left.1..=bottom_right.1+3 {
        let mut line = String::from("");
        for x in top_left.0..=bottom_right.0 {
            line.push(match cave.get(&(x,y)) {
                Some(i) if *i == Content::Rock => '#',
                Some(i) if *i == Content::Sand => 'o',
                _ => ' ',
            });
        }
        res.push(line);
    }
    res
}

fn print_cave(cave: &Cave) {
    for l in draw_cave(&cave) {
        println!("{:?}", l);
    }
}

#[derive(PartialEq)]
enum SearchResult {
    Pos((i32, i32)),
    Done,
}
fn find_resting_pos(cave: &Cave, pos: &(i32, i32)) -> SearchResult {
    let mut y = pos.1;
    loop {
        match cave.get(&(pos.0, y+1)) {
            None => {},
            _ => {
                let left = (pos.0-1, y+1);
                if cave.get(&left).is_none() {
                    match find_resting_pos(&cave, &left) {
                        SearchResult::Pos(left_pos) => { return SearchResult::Pos(left_pos); },
                        SearchResult::Done => { return SearchResult::Done; },
                    }
                } 
                let right = (pos.0+1, y+1);
                if cave.get(&right).is_none() {
                    match find_resting_pos(&cave, &right) {
                        SearchResult::Pos(right_pos) => { return SearchResult::Pos(right_pos); },
                        SearchResult::Done => { return SearchResult::Done; },
                    }
                }
                return SearchResult::Pos((pos.0, y));
            }
        }
        if y > cave.y_max {
            return SearchResult::Done;
        }
        y += 1;
    }    
}

fn drop_sand(cave: &mut Cave) -> SearchResult {

    let p = find_resting_pos(&cave, &START);
    match &p {
        &SearchResult::Pos(p) => { 
            cave.data.insert(p, Content::Sand); 
            if p == START {
                SearchResult::Done
            } else {
                SearchResult::Pos(p)
            }
        },
        _ => p,
    }
}

fn main() {
    let indata = fs::read_to_string("data/day14.txt").expect("No indata");
    let res = parse_indata(&indata).ok().unwrap().1;
    assert_eq!(179, res.len());

    let mut cave = Cave::from(&res);
    let mut count = 0;
    loop {
        let res = drop_sand(&mut cave);
        if count % 20 == 0 {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print_cave(&cave);
        }
        if res == SearchResult::Done {
            break;
        } 
        count += 1;
    }
    println!("Part2: {}", count+1);
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

        let cave = Cave::from(&res);
        assert_eq!(20, cave.data.iter().count());

        draw_cave(&cave);
    }
}
