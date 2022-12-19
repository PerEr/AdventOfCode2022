use std::{fs};
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Op {
    Add(i32),
    Mul(i32),
    Sq,
    X2,
}
#[derive(Debug, PartialEq)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    op: Op,
    div_by: i32,
    monkey_iftrue: i32,
    monkey_iffalse: i32,
}

fn parse_indata(indata: &str) -> Vec<Monkey> {
    Regex::new(r"Monkey (\d+):\n[ ]*Starting items: (.*)\n[ ]*Operation: new = old (\*|\+) (\d+|old)\n[ ]*Test: divisible by (\d+)\n[ ]*If true: throw to monkey (\d+)\n[ ]*If false: throw to monkey (\d+)[\n]+")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            let items: Vec<i32> = cap[2].split(",").map(|i| i.trim().parse().unwrap()).collect();
            let op = match &cap[3] {
                "*" => if &cap[4] == "old" {Op::Sq} else {Op::Mul(cap[4].trim().parse().unwrap())},
                "+" => if &cap[4] == "old" {Op::X2} else {Op::Add(cap[4].trim().parse().unwrap())},
                _ => panic!("Can't happen"),
            };
            let div_by = cap[5].parse().unwrap();
            let monkey_iftrue = cap[6].parse().unwrap();
            let monkey_iffalse = cap[7].parse().unwrap();
            Monkey { id: cap[1].parse().unwrap(), items, op, div_by, monkey_iftrue, monkey_iffalse }
        })
        .collect()
}



fn main() {
    let indata = fs::read_to_string("data/day11.txt").expect("No indata");
    let monkeys = parse_indata(&indata);
    println!("{:?}", monkeys);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;


    const TEST_DATA: &'static str = indoc! {r#"
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "#
    };

    #[test]
    fn test_part1() {
        let monkeys = parse_indata(&TEST_DATA);
    }


}
