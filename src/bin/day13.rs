use std::{fs, cmp::Ordering};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1},
    IResult, combinator::{map_res}, sequence::delimited, multi::{separated_list0},
};

#[derive(Debug, Clone, PartialEq)]
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
                Err(_n) => panic!("Should not happen"),
            }
    }).collect()
    }).collect()
}
fn check_list_order(list0: &Item, list1: &Item) -> Option<bool> {
    if let (Item::List(l0), Item::List(l1)) = (list0, list1) {
        let mut it0 = l0.into_iter();
        let mut it1 = l1.into_iter();
        loop {
            match (it0.next(),it1.next()) {
                (None, None) => return None,
                (None, _) => return Some(true),
                (_, None) => return Some(false),
                (Some(Item::Int(i0)), Some(Item::Int(i1))) => if i0 != i1 {
                    return Some(i0 < i1);
                },
                (Some(Item::Int(i0)), Some(Item::List(l1))) => {
                    let r = check_list_order(&Item::List(vec!(Item::Int(*i0))), &Item::List(l1.to_vec()));
                    if r.is_some() {
                        return r;
                    }
                },
                (Some(Item::List(l0)), Some(Item::Int(i1))) => {
                    let r = check_list_order(&Item::List(l0.to_vec()), &Item::List(vec!(Item::Int(*i1))));
                    if r.is_some() {
                        return r;
                    }
                },
                (Some(Item::List(l0)), Some(Item::List(l1))) => {
                    let r = check_list_order(&Item::List(l0.to_vec()), &Item::List(l1.to_vec()));
                    if r.is_some() {
                        return r;
                    }
                },
            }
        }
    } else {
        panic!("Should not happen");
    }
}

fn add_indicies(lst: Vec<bool>) -> i32 {
    let mut sum = 0;
    let mut index = 0;
    for l in lst {
        index += 1;
        if l {
            sum += index;
        }
    }
    sum
}

fn mul_delim_indicies(delims: &Vec<Item>, res: &Vec<Item>) -> i32 {
    let mut index = 0;
    let mut indicies: Vec<i32> = Vec::new();
    for ii in res {
        index += 1;
        if delims.contains(&ii) {
            indicies.push(index);
        }
    }
    indicies.into_iter().product()
}

fn item_cmp(a: &Item, b: &Item) -> Ordering {
    match check_list_order(a, b) {
        Some(b) => if b { Ordering::Less } else { Ordering::Greater },
        None => Ordering::Equal,
    }
}

fn append_and_sort(lists: Vec<&str>, res: &mut Vec<Item>) -> Vec<Item> {
    let mut delims: Vec<Item> = Vec::new();
    for l in lists {
        match parse_list(l) {
            Ok((_, item)) => {
                delims.push(item.clone());
                res.push(item)
            },
            _ => panic!("Should not happen"),
        }
    }
    res.sort_by(item_cmp);
    delims
}

fn main() {
    let indata = fs::read_to_string("data/day13.txt").expect("No indata");
    {
        let res: Vec<bool> = parse_indata(&indata).into_iter().map(|p| check_list_order(&p[0], &p[1])).map(|o| o.unwrap()).collect();
        println!("Part 1: {:?}", add_indicies(res));
    }
    {
        let mut res: Vec<Item> = parse_indata(&indata).into_iter().flatten().collect();
        let delims: Vec<Item> = append_and_sort(vec!("[[2]]", "[[6]]"), &mut res);
        println!("Part 2: {}", mul_delim_indicies(&delims, &res));
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
        let res: Vec<bool> = res.into_iter().map(|p| check_list_order(&p[0], &p[1])).map(|o| o.unwrap()).collect();
        assert_eq!(vec!(true, true, false, true, false, true, false, false), res);
        assert_eq!(13, add_indicies(res));
    }

    #[test]
    fn test_part2() {
        let mut res: Vec<Item> = parse_indata(&TEST_DATA).into_iter().flatten().collect();
        let delims: Vec<Item> = append_and_sort(vec!("[[2]]", "[[6]]"), &mut res);
        let mut index = 0;
        let mut indicies: Vec<i32> = Vec::new();
        for ii in &res {
            index += 1;
            if delims.contains(&ii) {
                indicies.push(index);
            }
        }
        assert_eq!(140, mul_delim_indicies(&delims, &res));

    }

}
