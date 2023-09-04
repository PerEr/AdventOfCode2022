use std::{fs, cmp::max};

type Cell = (i32, bool);
type Grid = Vec<Vec<Cell>>;

fn parse_indata(input: &str) -> Grid {
    let mut grid = Vec::new();
    let to_cell = |c: char| -> Cell {
        (c.to_digit(10).unwrap() as i32, false)
    };
    for line in input.split('\n') {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(to_cell(c));
        }
        if !row.is_empty() {
            grid.push(row);
        }
    }
    grid
}

fn set_col_visibility(row: &mut Vec<Cell>, col_index: fn(cols: usize, ix:usize) -> usize) {
    let mut threshold: i32 = -1;
    for index in 0..row.len() {
        let col = col_index(row.len(), index);
        if row[col].0 > threshold {
            row[col].1 = true;
            threshold = row[col].0;
        }
    }
}

fn set_row_visibility(grid: &mut Grid, col: usize, row_index: fn(rows: usize, ix:usize) -> usize) {
    let mut threshold: i32 = -1;
    for index in 0..grid.len() {
        let row = row_index(grid.len(), index);
        if grid[row][col].0 > threshold {
            grid[row][col].1 = true;
            threshold = grid[row][col].0;
        }
    }
}

fn set_visibility(grid: &mut Grid) {
    for row in grid.iter_mut() {
        set_col_visibility(&mut *row, |_, ix| ix);
        set_col_visibility(&mut *row, |cols, ix| cols - ix -1);
    }

    for col in 0..grid[0].len() {
        set_row_visibility(grid, col, |_, ix| ix);
        set_row_visibility(grid, col, |rows: usize, ix| rows - ix - 1);
    }
}

fn count_trees(grid: &Grid, row: usize, col: usize, dr: i32, dc: i32) -> usize {
    let mut r = row as i32;
    let mut c = col as i32;
    let mut count = 0;
    loop {    
        r += dr;
        c += dc;
        if r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[r as usize].len() as i32 {    
            let h = grid[r as usize][c as usize].0;
            count += 1;
            if h >= grid[row][col].0 {
                break;
            }
        } else {
            break;
        }
    }
    count
}

fn calc_score(grid: &Grid, row: usize, col: usize) -> usize {
    count_trees(grid, row, col, -1, 0) * 
    count_trees(grid, row, col, 1, 0) * 
    count_trees(grid, row, col, 0, 1) * 
    count_trees(grid, row, col, 0, -1) 
}

fn get_max_score(grid: &Grid) -> usize {
    let mut max_score = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            max_score = max(max_score, calc_score(grid, row, col));
        }
    }
    max_score
}

fn count_visible(grid: & Grid) -> usize {
    grid.iter().flatten().filter(|c| c.1).count()
}

fn main() {
    let indata = fs::read_to_string("data/day8.txt").expect("No indata");
    let mut grid = parse_indata(&indata);
    set_visibility(&mut grid);
    println!("Part1: {:?}", count_visible(&grid));
    println!("Part2: {:?}", get_max_score(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &str = indoc! {r#"
    30373
    25512
    65332
    33549
    35390
    "#
    };

    #[test]
    fn test_part1() {
        let mut grid = parse_indata(TEST_DATA);
        set_visibility(&mut grid);
        assert_eq!(21, count_visible(&grid));
    }

    #[test]
    fn test_part2() {
        let grid = parse_indata(TEST_DATA);
        assert_eq!(8, get_max_score(&grid));
    }

}
