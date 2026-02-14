use std::collections::HashSet;
use std::fs;

fn main() {

    let input_data = fs::read_to_string("input-data").unwrap();
    let result = solve(&input_data);
    println!("{}", result);
}

fn solve(input_data: &str) -> usize {

    let mut grid: Vec<Vec<char>> = input_data.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut start_row = 0;
    let mut start_col = 0;
    let mut direction = 0;

    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                '^' => {
                    start_row = r;
                    start_col = c;
                    direction = 0;
                }
                '>' => {
                    start_row = r;
                    start_col = c;
                    direction = 1;
                }
                'v' => {
                    start_row = r;
                    start_col = c;
                    direction = 2;
                }
                '<' => {
                    start_row = r;
                    start_col = c;
                    direction = 3;
                }
                _ => {}
            }
        }
    }

    let mut visited = HashSet::new();
    let mut row = start_row as isize;
    let mut col = start_col as isize;

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    loop {

        visited.insert((row, col));
        let (dr, dc) = directions[direction];
        let next_row = row + dr;
        let next_col = col + dc;

        if next_row < 0
            || next_col < 0
            || next_row >= rows as isize
            || next_col >= cols as isize
        {
            break;
        }

        if grid[next_row as usize][next_col as usize] == '#' {
            
            direction = (direction + 1) % 4;
        } else {
            
            row = next_row;
            col = next_col;
        }
    }

    visited.len()
}
