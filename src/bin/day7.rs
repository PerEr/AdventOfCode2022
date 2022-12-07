use std::collections::HashMap;
use std::fs;

fn parse_indata(input: &str) -> HashMap<Vec<&str>, usize> {
    let mut paths: Vec<&str> = Vec::new();

    input
        .lines()
        .filter(|l| !l.is_empty())
        .fold(HashMap::new(), |mut sizes, line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next(), parts.next()) {
                (Some("$"), Some("cd"), Some("..")) => {
                    paths.pop();
                }
                (Some("$"), Some("cd"), Some(dir)) => {
                    paths.push(dir);
                }
                (Some("$"), _, _) => return sizes,
                (Some("dir"), _, _) => return sizes,
                (Some(size), _, _) => {
                    (0..paths.len())
                        .map(|i| paths[0..=i].to_vec())
                        .for_each(|subpath| {
                            *sizes.entry(subpath).or_insert(0) += size.parse::<usize>().unwrap();
                        });
                }
                _ => return sizes,
            };
            sizes
        })
}

fn calc_part1(sizes: &HashMap<Vec<&str>, usize>, lim: usize) -> usize {
    sizes.values().filter(|&&v| v <= lim).sum()
}

fn calc_part2(sizes: &HashMap<Vec<&str>, usize>, tot: usize, free: usize) -> usize {
    *sizes
        .values()
        .filter(|&&v| v >= sizes[&vec!["/"]] + &free - tot)
        .min()
        .unwrap()
}

fn main() {
    let indata = fs::read_to_string("data/day7.txt").expect("No indata");
    let sizes = parse_indata(&indata);
    println!("Part1: {}", calc_part1(&sizes, 100000));
    println!("Part2: {}", calc_part2(&sizes, 70000000, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "#
    };

    #[test]
    fn test_part1() {
        let sizes = parse_indata(&TEST_DATA);
        assert_eq!(95437, calc_part1(&sizes, 100000));
    }

    #[test]
    fn test_part2() {
        let sizes = parse_indata(&TEST_DATA);
        assert_eq!(24933642, calc_part2(&sizes, 70000000, 30000000));
    }
}
