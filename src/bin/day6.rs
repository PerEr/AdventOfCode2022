use std::{fs, collections::HashSet};

fn find_marker_index(input: &str, nr: usize) -> usize {
    let ch: Vec<char> = input.chars().collect();
    let mut index = nr;
    for ws in ch.windows(nr) {
        let s: HashSet<char> = ws.iter().copied().collect();
        if s.len() == nr {
            break;
        }
        index += &1;
    }
    index
}

fn main() {
    let indata = fs::read_to_string("data/day6.txt").expect("No indata");
    println!("Part1: {:?}", find_marker_index(&indata, 4));
    println!("Part2: {:?}", find_marker_index(&indata, 14));


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let len: usize = 4;
        assert_eq!(7, find_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", len));
        assert_eq!(5, find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", len));
        assert_eq!(6, find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", len));
        assert_eq!(10, find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", len));
        assert_eq!(11, find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", len));
    }

    #[test]
    fn test_part2() {
        const len: usize = 14;
        assert_eq!(19, find_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", len));
        assert_eq!(23, find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", len));
        assert_eq!(23, find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", len));
        assert_eq!(29, find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", len));
        assert_eq!(26, find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", len));
    }
}
