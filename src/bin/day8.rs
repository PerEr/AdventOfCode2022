use std::fs;

type Grid = Vec<Vec<(i32, bool)>>;

fn parse_indata(input: &str) -> Grid {
    let mut grid = Vec::new();
    let to_cell = |c: char| -> (i32, bool) {
        (c.to_digit(10).unwrap() as i32, false)
    };
    for line in input.split("\n") {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(to_cell(c));
        }
        if row.len() > 0 {
            grid.push(row);
        }
    }
    grid
}

fn set_col_visibility(row: &mut Vec<(i32,bool)>, col_index: fn(cols: usize, ix:usize) -> usize) {
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
    let rows = grid.len();

    for row in 0..rows {
        set_col_visibility(&mut grid[row], |_, ix| ix);
        set_col_visibility(&mut grid[row], |cols, ix| cols - ix -1);
    }

    for col in 0..grid[0].len() {
        set_row_visibility(grid, col, |_, ix| ix);
        set_row_visibility(grid, col, |rows: usize, ix| rows - ix - 1);
    }
}

fn count_visible(grid: & Grid) -> usize {
    grid.iter().flatten().filter(|c| c.1).count()
}

fn main() {
    let indata = fs::read_to_string("data/day8.txt").expect("No indata");
    let mut grid = parse_indata(&indata);
    set_visibility(&mut grid);
    println!("Part1: {:?}", count_visible(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    30373
    25512
    65332
    33549
    35390
    "#
    };

    #[test]
    fn test_example() {
        let mut grid = parse_indata(&TEST_DATA);
        set_visibility(&mut grid);
        assert_eq!(21, count_visible(&grid));
    }

}
