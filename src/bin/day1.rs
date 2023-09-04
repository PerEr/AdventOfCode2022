use std::fs;

fn parse_indata(indata: &str) -> Vec<Vec<i32>> {
    indata
        .split("\n\n")
        .map(|b| {
            b.split('\n')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn process(indata: &str) -> (i32, i32) {
    let mut sums: Vec<i32> = parse_indata(indata)
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .collect();
    sums.sort_by(|a, b| b.cmp(a));
    (sums[0], sums[0..3].iter().sum::<i32>())
}

fn main() {
    let indata = fs::read_to_string("data/day1.txt").expect("No indata");
    let (p1, p2) = process(&indata);
    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_example() {
        let test_data = indoc! {r#"
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        "#
        };
        let (p1, p2) = super::process(test_data);
        assert_eq!(24000, p1);
        assert_eq!(45000, p2);
    }
}
