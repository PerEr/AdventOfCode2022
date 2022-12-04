use regex::Regex;

use std::fs;

fn parse_indata<'a>(indata: &'a str) -> Vec<(i32, i32, i32, i32)> {
    Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            )
        })
        .collect()
}

fn complete_overlap(d: &(i32, i32, i32, i32)) -> bool {
    match d {
        (a, b, c, d) if a >= c && b <= d => true,
        (a, b, c, d) if c >= a && d <= b => true,
        _ => false,
    }
}

fn some_overlap(d: &(i32, i32, i32, i32)) -> bool {
    match d {
        (a, _b, c, d) if a >= c && a <= d => true,
        (_a, b, c, d) if b >= c && b <= d => true,
        (a, b, c, d) if a <= c && b >= d => true,
        _ => false,
    }
}

fn process(data: &Vec<(i32, i32, i32, i32)>, f: fn(&(i32, i32, i32, i32)) -> bool) -> usize {
    data.iter().filter(|d| f(*d)).count()
}

fn main() {
    let indata = fs::read_to_string("data/day4.txt").expect("No indata");
    let data = parse_indata(&indata);
    println!("Part1: {:?}", process(&data, complete_overlap));
    println!("Part2: {:?}", process(&data, some_overlap));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "#
    };

    #[test]
    fn test_part1() {
        let data = parse_indata(&TEST_DATA);
        let score = process(&data, complete_overlap);
        assert_eq!(2, score);
    }

    #[test]
    fn test_part2() {
        let data = parse_indata(&TEST_DATA);
        let score = process(&data, some_overlap);
        assert_eq!(4, score);
    }
}
