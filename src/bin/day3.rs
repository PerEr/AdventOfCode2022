use itertools::Itertools;
use std::{collections::HashSet, fs};

fn to_priority(item: u8) -> i32 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as i32,
        b'A'..=b'Z' => (item - b'A' + 27) as i32,
        _ => 0,
    }
}

fn parse_indata(indata: &str) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    indata
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            (
                left.bytes().map(to_priority).collect::<HashSet<_>>(),
                right.bytes().map(to_priority).collect::<HashSet<_>>(),
            )
        })
        .collect()
}

fn process1(data: &Vec<(HashSet<i32>, HashSet<i32>)>) -> i32 {
    data.iter()
        .map(|(l, r)| l.intersection(r).next().unwrap())
        .sum()
}

fn process2(data: &Vec<(HashSet<i32>, HashSet<i32>)>) -> i32 {
    data.iter()
        .map(|(l, r)| l | r)
        .tuples()
        .map(|(s1, s2, s3)| (&(&s1 & &s2) & &s3).into_iter().next().unwrap())
        .sum()
}

fn main() {
    let indata = fs::read_to_string("data/day3.txt").expect("No indata");
    let data = parse_indata(&indata);
    println!("Part1: {:?}", process1(&data));
    println!("Part2: {:?}", process2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    "#
    };
    #[test]
    fn test_part1() {
        let data = parse_indata(&TEST_DATA);
        let score = process1(&data);
        assert_eq!(157, score);
    }

    #[test]
    fn test_part2() {
        let data = parse_indata(&TEST_DATA);
        let score = process2(&data);
        assert_eq!(70, score);
    }
}
