use std::{fs, collections::{HashMap}};
use std::collections::VecDeque;

fn char_to_value(c: char) -> i32 {
    let a = 'A' as i32;
    (c as i32) - a
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Pos {
    r: i32,
    c: i32,
}

impl Pos {
    fn from(r: i32, c: i32) -> Self {
        Self {r, c}
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    rows: i32,
    cols: i32,
    data: Vec<Vec<i32>>,
}



impl Grid {

    fn from(data: Vec<Vec<i32>>) -> Self {
        Self {
            rows: data.len() as i32,
            cols: data[0].len() as i32,
            data,
        }
    }

    fn get(&self, p: &Pos) -> i32 {
        self.data[p.r as usize][p.c as usize]
    }

    fn set(&mut self, p: &Pos, v: i32) {
        self.data[p.r as usize][p.c as usize] = v;
    }

    fn neighbours(&self, p: &Pos) -> Vec<Pos> {
        vec!((0,1), (0, -1), (1, 0), (-1,0)).iter()
            .map(|(dr,dc)| (p.r+dr, p.c+dc))
            .filter(|(r,c)| *r >= 0 && *c >= 0 && *r < self.rows as i32 && *c < self.cols as i32)
            .map(|(r ,c)| Pos::from(r,c))
            .collect()
    }
}

fn parse_indata(indata: &str) -> Grid {
    Grid::from(indata.lines().map(|l| l.chars().map(char_to_value).collect()).collect())
}

fn find_char(grid: &Grid, ch: i32) -> Vec<Pos> {
    let mut res: Vec<Pos> = Vec::new();
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.get(&Pos::from(r,c)) == ch {
                res.push(Pos::from(r,c));
            }
        }
    }
    res
}



fn solve_maze(grid: &Grid, start: &Pos, end: &Pos) -> HashMap<Pos, (Pos, i32)> {
    let mut solution: HashMap<Pos, (Pos, i32)> = HashMap::new();
    let mut to_visit: VecDeque<(Pos, i32)> = VecDeque::with_capacity(128);

    to_visit.push_back((start.clone(), 0));
    solution.insert(start.clone(), (start.clone(), 0));

    while let Some((pos, path_len)) = to_visit.pop_front() {
        let height = grid.get(&pos);
        for p in grid.neighbours(&pos) {
            let n_height = grid.get(&p);
            if n_height - height <= 1 {
                let next_path_len = path_len + 1;
                if solution.get(&p).map(|(_,l)| l).unwrap_or(&i32::max_value()) <= &next_path_len {
                    continue;
                }
                solution.insert(p.clone(), (pos.clone(), next_path_len));
                to_visit.push_back((p.clone(), next_path_len));
                if pos == *end {
                    break;
                }
            }
        }
    }
    solution
}

fn main() {
    let indata = fs::read_to_string("data/day12.txt").expect("No indata");
    let mut grid = parse_indata(&indata);
    let start = find_char(&grid, char_to_value('S'))[0];
    let end = find_char(&grid, char_to_value('E'))[0];

    grid.set(&start, char_to_value('a'));
    grid.set(&end, char_to_value('z'));

    {
        let solution = solve_maze(&grid, &start, &end);
        let res = solution.get(&end).unwrap();
        println!("Part1: {:?}", res);
    }

    {
        let mut res: Vec<(Pos, i32)> = find_char(&grid, char_to_value('a')).into_iter().map(|sp| {
            let solution = solve_maze(&grid, &sp, &end);
            if let Some((_, dist)) = solution.get(&end) {
                Some((sp, *dist))
            } else {
                None
            }
        }).filter(|op| op.is_some()).map(|op| op.unwrap()).collect();
        res.sort_by(|a,b| a.1.cmp(&b.1));
        println!("Part2: {:?}", res[0]);
    }
}

// https://github.com/tumdum/aoc2022/blob/main/src/day12.rs
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    "#
    };

    #[test]
    fn test_part1() {
        let mut grid = parse_indata(&TEST_DATA);
        let start = find_char(&grid, char_to_value('S'))[0];
        let end = find_char(&grid, char_to_value('E'))[0];
        assert_eq!(Pos::from(0,0), start);
        assert_eq!(Pos::from(2,5), end);

        grid.set(&start, char_to_value('a'));
        grid.set(&end, char_to_value('z'));

        let solution = solve_maze(&grid, &start, &end);
        let res = solution.get(&end).unwrap();

        assert_eq!(31, res.1);
    }

    #[test]
    fn test_part2() {
        let mut grid = parse_indata(&TEST_DATA);
        let start = find_char(&grid, char_to_value('S'))[0];
        let end = find_char(&grid, char_to_value('E'))[0];
        assert_eq!(Pos::from(0,0), start);
        assert_eq!(Pos::from(2,5), end);

        grid.set(&start, char_to_value('a'));
        grid.set(&end, char_to_value('z'));

        let mut res: Vec<(Pos, i32)> = find_char(&grid, char_to_value('a')).into_iter().map(|sp| {
            let solution = solve_maze(&grid, &sp, &end);
            let (_, dist) = solution.get(&end).unwrap();
            (sp, *dist)
        }).collect();
        res.sort_by(|a,b| a.1.cmp(&b.1));

        assert_eq!(29, res[0].1);
    }

}
