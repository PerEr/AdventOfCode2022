use std::fs;

fn parse_indata(indata: &str) -> Vec<(Choice, Choice)> {
    indata
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|b| {
            let mut ch = b.chars();
            (Choice::from(ch.next().unwrap()), {
                ch.next();
                Choice::from(ch.next().unwrap())
            })
        })
        .collect()
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice {
    R,
    P,
    S,
}

impl From<char> for Choice {
    fn from(ch: char) -> Self {
        match ch {
            'A' => Choice::R,
            'B' => Choice::P,
            'C' => Choice::S,
            'X' => Choice::R,
            'Y' => Choice::P,
            'Z' => Choice::S,
            _ => panic!("cant happen"),
        }
    }
}

impl Choice {
    fn value(&self) -> i32 {
        match self {
            Self::R => 1,
            Self::P => 2,
            Self::S => 3,
        }
    }
    fn looses_to(&self) -> Self {
        match self {
            Self::R => Self::P,
            Self::P => Self::S,
            Self::S => Self::R,
        }
    }
}

fn score(c: &(Choice, Choice)) -> i32 {
    c.1.value()
        + if c.0 == c.1 {
            3
        } else if c.0.looses_to() == c.1 {
            6
        } else {
            0
        }
}

fn calc_choice(c: &(Choice, Choice)) -> (Choice, Choice) {
    match c.1 {
        Choice::R => (c.0, c.0.looses_to().looses_to()),
        Choice::P => (c.0, c.0),
        Choice::S => (c.0, c.0.looses_to()),
    }
}

fn process(strategy: &Vec<(Choice, Choice)>, f: fn(&(Choice, Choice)) -> (Choice, Choice)) -> i32 {
    strategy
        .iter()
        .map(f)
        .map(|t| score(&t))
        .fold(0, |a, v| a + v)
}

fn main() {
    let indata = fs::read_to_string("data/day2.txt").expect("No indata");
    let strategy = parse_indata(&indata);
    println!("Part1: {}", process(&strategy, |t| t.clone()));
    println!("Part2: {}", process(&strategy, calc_choice));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let test_data = indoc! {r#"
        A Y
        B X
        C Z
        "#
        };

        let strategy = parse_indata(&test_data);
        assert_eq!(3, strategy.len());

        assert_eq!(15, process(&strategy, |t| t.clone()));
        assert_eq!(12, process(&strategy, calc_choice));
    }
}
