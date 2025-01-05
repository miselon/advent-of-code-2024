use std::fs;

struct InputData {

    rules:Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>
}

fn main() {

    let data = get_input_data();

    let mut sum = 0;

    for update in data.updates {

        if is_not_valid_update(&update, &data.rules) {

            continue;
        } 

        sum += update[update.len() / 2]
    }

    println!("Sum of correct middle page numbers is: {}", sum);
}

fn is_not_valid_update(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    
    for rule in rules {

        let left_index = update.iter().position(|page| page == &rule.0);
        let right_index = update.iter().position(|page| page == &rule.1);

        if left_index.is_some() && right_index.is_some() && left_index.unwrap() >= right_index.unwrap() {

            return true;
        }
    }

    false
}

fn get_input_data() -> InputData {

    let file_contents = fs::read_to_string("input-data").unwrap();

    let data: Vec<&str> = file_contents.split("\r\n\r\n").collect();

    let rules = data[0]
        .lines()
        .map(parse_rules)
        .collect();

    let updates = data[1]
        .lines()
        .map(|line| split_and_parse_to_usize(line, ','))
        .collect();

    InputData { rules, updates }
}

fn parse_rules(line: &str) -> (usize, usize) {

    let split: Vec<usize> = split_and_parse_to_usize(line, '|');

    ( split[0], split[1] )
}

fn split_and_parse_to_usize(line: &str, delimiter: char) -> Vec<usize> {

    line.split(delimiter)
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect()
}