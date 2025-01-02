use std::fs;

fn main() {

    let data = get_input_data();
    let directions: Vec<i32> = vec![-1, 0, 1];
    let mut xmas_occurrences = 0;


    for row_index in 0 .. data.len() as i32 {
        for column_index in 0 .. data[0].len() as i32 {

            let character = data[row_index as usize][column_index as usize];

            if character != 'X' { 

                continue 
            }

            for row_direction in &directions {
                for column_direction in &directions {

                    let m_position= (row_index + row_direction, column_index + column_direction);
                    let a_position = (row_index + 2 * row_direction, column_index + 2 * column_direction);
                    let s_position = (row_index + 3 * row_direction, column_index + 3 * column_direction);
                
                    if is_out_of_bounds(&s_position, &data) {

                        continue
                    }

                    if data[m_position.0 as usize][m_position.1 as usize] == 'M' 
                        && data[a_position.0 as usize][a_position.1 as usize] == 'A' 
                        && data[s_position.0 as usize][s_position.1 as usize] == 'S' {

                        xmas_occurrences += 1
                    }
                }
            }
        }
    }

    println!("XMAS occured {} times", xmas_occurrences);
}

fn is_out_of_bounds(position:&(i32, i32), data:&Vec<Vec<char>>) -> bool {

    position.0 < 0 || position.0 >= data.len() as i32 || position.1 < 0 || position.1 >= data[0].len() as i32
}

fn get_input_data() -> Vec<Vec<char>> {

    fs::read_to_string("input-data")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}