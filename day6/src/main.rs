use std::collections::HashSet;
use std::fs;

fn main() {

    let input_data = fs::read_to_string("input-data").unwrap();
    let result = solve(&input_data);

    println!("{}", result);
}

fn solve(input_data: &str) -> usize {

    let grid: Vec<Vec<char>> = input_data.lines().map(|line| line.chars().collect()).collect();
    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut start_row_index = 0;
    let mut start_column_index = 0;
    let mut start_direction_index = 0;

    for row_index in 0..row_count {

        for column_index in 0..column_count {

            match grid[row_index][column_index] {
                '^' => {
                    start_row_index = row_index;
                    start_column_index = column_index;
                    start_direction_index = 0;
                }
                '>' => {
                    start_row_index = row_index;
                    start_column_index = column_index;
                    start_direction_index = 1;
                }
                'v' => {
                    start_row_index = row_index;
                    start_column_index = column_index;
                    start_direction_index = 2;
                }
                '<' => {
                    start_row_index = row_index;
                    start_column_index = column_index;
                    start_direction_index = 3;
                }
                _ => {}
            }
        }
    }

    let visited_positions = trace_guard_path(
        &grid,
        row_count,
        column_count,
        start_row_index,
        start_column_index,
        start_direction_index,
    );

    let mut loop_count = 0;

    for &(row_position, column_position) in &visited_positions {

        if row_position == start_row_index as isize && column_position == start_column_index as isize {

            continue;
        }

        if causes_loop_with_added_obstacle(
            &grid,
            row_count,
            column_count,
            start_row_index,
            start_column_index,
            start_direction_index,
            row_position as usize,
            column_position as usize,
        ) {
            loop_count += 1;
        }
    }

    loop_count
}

fn trace_guard_path(
    grid: &Vec<Vec<char>>,
    row_count: usize,
    column_count: usize,
    start_row_index: usize,
    start_column_index: usize,
    start_direction_index: usize,
) -> HashSet<(isize, isize)> {

    let direction_vectors = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut current_row_position = start_row_index as isize;
    let mut current_column_position = start_column_index as isize;
    let mut current_direction_index = start_direction_index;

    let mut visited_positions = HashSet::new();

    loop {

        visited_positions.insert((current_row_position, current_column_position));

        let (row_delta, column_delta) = direction_vectors[current_direction_index];
        let next_row_position = current_row_position + row_delta;
        let next_column_position = current_column_position + column_delta;

        if next_row_position < 0
            || next_column_position < 0
            || next_row_position >= row_count as isize
            || next_column_position >= column_count as isize
        {
            break;
        }

        if grid[next_row_position as usize][next_column_position as usize] == '#' {

            current_direction_index = (current_direction_index + 1) % 4;

        } else {

            current_row_position = next_row_position;
            current_column_position = next_column_position;
        }
    }

    visited_positions
}

fn causes_loop_with_added_obstacle(
    grid: &Vec<Vec<char>>,
    row_count: usize,
    column_count: usize,
    start_row_index: usize,
    start_column_index: usize,
    start_direction_index: usize,
    obstacle_row_index: usize,
    obstacle_column_index: usize,
) -> bool {

    let direction_vectors = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut current_row_position = start_row_index as isize;
    let mut current_column_position = start_column_index as isize;
    let mut current_direction_index = start_direction_index;

    let mut visited_states = HashSet::new();

    loop {

        if !visited_states.insert((current_row_position, current_column_position, current_direction_index,)) {

            return true;
        }

        let (row_delta, column_delta) = direction_vectors[current_direction_index];
        let next_row_position = current_row_position + row_delta;
        let next_column_position = current_column_position + column_delta;

        if next_row_position < 0
            || next_column_position < 0
            || next_row_position >= row_count as isize
            || next_column_position >= column_count as isize
        {
            return false;
        }

        let mut next_cell_value = grid[next_row_position as usize][next_column_position as usize];

        if next_row_position as usize == obstacle_row_index && next_column_position as usize == obstacle_column_index {

            next_cell_value = '#';
        }

        if next_cell_value == '#' {

            current_direction_index = (current_direction_index + 1) % 4;
        } else {
            
            current_row_position = next_row_position;
            current_column_position = next_column_position;
        }
    }
}
