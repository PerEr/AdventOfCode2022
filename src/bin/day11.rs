use std::{fs, collections::{HashMap, hash_map::Entry}};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Add(i32),
    Mul(i32),
    Sq,
    X2,
}
#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    id: i32,
    op: Op,
    div_by: i32,
    monkey_iftrue: i32,
    monkey_iffalse: i32,
}

fn parse_indata(indata: &str) -> (Vec<Monkey>, HashMap<i32, Vec<i32>>) {
    let mut item_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let monkeys = Regex::new(r"Monkey (\d+):\n[ ]*Starting items: (.*)\n[ ]*Operation: new = old (\*|\+) (\d+|old)\n[ ]*Test: divisible by (\d+)\n[ ]*If true: throw to monkey (\d+)\n[ ]*If false: throw to monkey (\d+)[\n]+")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            let id: i32 = cap[1].parse().unwrap();
            let items: Vec<i32> = cap[2].split(",").map(|i| i.trim().parse().unwrap()).collect();
            item_map.insert(id, items);
            let op = match &cap[3] {
                "*" => if &cap[4] == "old" {Op::Sq} else {Op::Mul(cap[4].trim().parse().unwrap())},
                "+" => if &cap[4] == "old" {Op::X2} else {Op::Add(cap[4].trim().parse().unwrap())},
                _ => panic!("Can't happen"),
            };
            let div_by = cap[5].parse().unwrap();
            let monkey_iftrue = cap[6].parse().unwrap();
            let monkey_iffalse = cap[7].parse().unwrap();
            Monkey { id, op, div_by, monkey_iftrue, monkey_iffalse }
        })
        .collect();
        (monkeys,item_map)
}

fn play_once(monkeys: &Vec<Monkey>, items: &HashMap<i32, Vec<i32>>, inspections: &HashMap<i32, i32>) -> (HashMap<i32, Vec<i32>>, HashMap<i32, i32>) {
    let mut next_items: HashMap<i32, Vec<i32>> = items.clone();
    let mut next_inspections: HashMap<i32, i32> = inspections.clone();
    for m in monkeys {
        let changes: Vec<(i32, i32)> = if let Some(it) = next_items.get(&m.id) {
            let mut res: Vec<(i32, i32)> = vec!(); 
            for ii in  it {
                let mut worry: i32 = *ii;
                match m.op {
                    Op::Add(n) => worry = worry + n,
                    Op::Mul(n) => worry = worry * n,
                    Op::Sq => worry = worry * worry,
                    Op::X2 => worry = worry + worry,
                }
                worry = worry / 3;
                let monkey_id = if worry % m.div_by == 0 {
                    m.monkey_iftrue
                } else {
                    m.monkey_iffalse
                };
                res.push((monkey_id, worry));
                match next_inspections.get(&m.id) {
                    Some(count) => { next_inspections.insert(m.id, count + 1); }
                    None => { next_inspections.insert(m.id, 1); }
                }
            }
            res
        } else {
            vec!()
        };
        for (monkey_id, worry) in changes {
            next_items.remove(&m.id);
            match next_items.entry(monkey_id) {
                Entry::Vacant(e) => { e.insert(vec![worry]); },
                Entry::Occupied(mut e) => { e.get_mut().push(worry); }
            }
        }
    }
    (next_items, next_inspections)
}

fn play(monkeys: &Vec<Monkey>, items: &HashMap<i32, Vec<i32>>, nr: usize) -> (HashMap<i32, Vec<i32>>, HashMap<i32,i32>) {
    let mut mut_items = items.clone();
    let mut mut_inspections: HashMap<i32, i32> = HashMap::new();
    for _ in 0..nr {
        (mut_items, mut_inspections) = play_once(&monkeys, &mut_items, &mut_inspections);
    }
    (mut_items, mut_inspections)
}

fn calc_monkey_business(inspections: &HashMap<i32, i32>) -> i32 {
    inspections.values().copied().sorted().rev().take(2).fold(1i32, |acc, val| acc * val)
}

fn main() {
    let indata = fs::read_to_string("data/day11.txt").expect("No indata");
    let (monkeys, items) = parse_indata(&indata);
    let (_, inspections) = play(&monkeys, &items, 20);
    println!("Part1: {:?}", calc_monkey_business(&inspections));
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
        let (monkeys, items) = parse_indata(&TEST_DATA);
        let (items, inspections) = play(&monkeys, &items, 20);
        assert_eq!(2, items.len());
        assert_eq!(vec!(10, 12, 14, 26, 34), items[&0]);
        assert_eq!(vec!(245, 93, 53, 199, 115), items[&1]);
        assert_eq!(10605, calc_monkey_business(&inspections));
    }


}
