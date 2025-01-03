use std::fs;

fn main() {

    let data = get_input_data();
    let mut xmas_occurrences = 0;

    for row_index in 1 .. (data.len() - 1) as i32 {
        for column_index in 1 .. (data[0].len() -1) as i32 {

            let character = data[row_index as usize][column_index as usize];

            if character != 'A' { 

                continue 
            }

            let string_to_check: String = vec![
                    data[(row_index - 1) as usize][(column_index - 1) as usize],
                    data[(row_index - 1) as usize][(column_index + 1) as usize],
                    data[(row_index + 1) as usize][(column_index - 1) as usize],
                    data[(row_index + 1) as usize][(column_index + 1) as usize]
                ]
                .into_iter()
                .collect();

            if vec!["MSMS", "MMSS", "SSMM", "SMSM"].iter().any(|pattern| pattern == &string_to_check) {

                xmas_occurrences += 1;
            }
        }
    }

    println!("X-MAS occured {} times", xmas_occurrences);
}

fn get_input_data() -> Vec<Vec<char>> {

    fs::read_to_string("input-data")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}