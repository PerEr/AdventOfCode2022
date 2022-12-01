use std::fs;

fn parse_indata(indata: &str) -> Vec<Vec<i32>> {
    indata
        .split("\n\n")
        .map(|b| {
            b.split("\n")
                .filter(|v| v.len() > 0)
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let indata = fs::read_to_string("data/day1.txt").expect("No indata");
    let parsed = parse_indata(&indata);

    let mut sums: Vec<i32> = parsed
        .iter()
        .map(|v| v.iter().fold(0, |a, v| a + v))
        .collect();
    sums.sort_by(|a, b| b.cmp(a));
    println!("Part1: {}", sums[0]);
    println!("Part2: {}", sums[0..3].iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use vector_assertions::assert_vec_eq;

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
        let parsed = super::parse_indata(&test_data);
        assert_eq!(5, parsed.len());
        assert_vec_eq!(vec!(5000, 6000), &parsed[2]);
    }
}
