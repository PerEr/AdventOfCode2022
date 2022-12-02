use std::fs;

fn parse_indata(indata: &str) -> Vec<(char, char)> {
    indata
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|b| {
            let mut ch = b.chars();
            (ch.next().unwrap(), {
                ch.next().unwrap();
                ch.next().unwrap()
            })
        })
        .collect()
}

fn score(t: &(char, char)) -> i32 {
    let p = match t {
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        _ => 0,
    };
    p + match t.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("illegal"),
    }
}

fn calc_choice(t: &(char, char)) -> (char, char) {
    match t {
        ('A', 'X') => ('A', 'Z'),
        ('A', 'Y') => ('A', 'X'),
        ('A', 'Z') => ('A', 'Y'),

        ('B', 'X') => ('B', 'X'),
        ('B', 'Y') => ('B', 'Y'),
        ('B', 'Z') => ('B', 'Z'),

        ('C', 'X') => ('C', 'Y'),
        ('C', 'Y') => ('C', 'Z'),
        ('C', 'Z') => ('C', 'X'),

        _ => panic!("cant happen"),
    }
}

fn process1(strategy: &Vec<(char, char)>) -> i32 {
    strategy.iter().map(|t| score(&t)).fold(0, |a, v| a + v)
}

fn process2(strategy: &Vec<(char, char)>) -> i32 {
    strategy
        .iter()
        .map(|t| calc_choice(&t))
        .map(|t| score(&t))
        .fold(0, |a, v| a + v)
}

fn main() {
    let indata = fs::read_to_string("data/day2.txt").expect("No indata");
    let strategy = parse_indata(&indata);
    println!("Part1: {}", process1(&strategy));
    println!("Part2: {}", process2(&strategy));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let test_data = indoc! {r#"
        A Y
        B X
        C Z
        "#
        };

        let strategy = super::parse_indata(&test_data);
        assert_eq!(3, strategy.len());

        assert_eq!(15, super::process1(&strategy));

        assert_eq!(12, super::process2(&strategy));
    }
}
