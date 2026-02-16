use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {

    let input_content = read_to_string("input-data")
        .expect("Should have been able to read the file");

    let grid_lines: Vec<String> = input_content
        .lines()
        .map(|line| line.to_string())
        .collect();

    if grid_lines.is_empty() {

        return;
    }

    let grid_height = grid_lines.len() as i32;

    let grid_width = grid_lines[0].len() as i32;

    let mut antennas_by_frequency: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row_index, line) in grid_lines.iter().enumerate() {

        for (column_index, character) in line.chars().enumerate() {

            if character != '.' {

                antennas_by_frequency
                    .entry(character)
                    .or_insert_with(Vec::new)
                    .push((row_index as i32, column_index as i32));
            }
        }
    }

    let mut unique_antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for frequency_locations in antennas_by_frequency.values() {

        for first_index in 0..frequency_locations.len() {

            for second_index in (first_index + 1)..frequency_locations.len() {

                let (first_row, first_column) = frequency_locations[first_index];

                let (second_row, second_column) = frequency_locations[second_index];

                let row_difference = second_row - first_row;

                let column_difference = second_column - first_column;

                let first_antinode_row = first_row - row_difference;

                let first_antinode_column = first_column - column_difference;

                let second_antinode_row = second_row + row_difference;

                let second_antinode_column = second_column + column_difference;

                if is_within_bounds(first_antinode_row, first_antinode_column, grid_height, grid_width) {

                    unique_antinode_locations.insert((first_antinode_row, first_antinode_column));
                }

                if is_within_bounds(second_antinode_row, second_antinode_column, grid_height, grid_width) {

                    unique_antinode_locations.insert((second_antinode_row, second_antinode_column));
                }
            }
        }
    }

    println!("{}", unique_antinode_locations.len());
}

fn is_within_bounds(row: i32, column: i32, height: i32, width: i32) -> bool {

    return row >= 0 && row < height && column >= 0 && column < width;
}