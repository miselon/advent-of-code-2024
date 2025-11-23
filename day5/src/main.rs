use std::fs;
use std::collections::{HashMap, HashSet};

struct InputData {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn main() {
    
    let data = get_input_data();

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    for update in &data.updates {

        if is_invalid_update(update, &data.rules) {

            let fixed = fix_update(update, &data.rules);
            sum_part2 += fixed[fixed.len() / 2];

        } else {

            sum_part1 += update[update.len() / 2];
        }
    }

    println!("Part 1 - Sum of correct middle page numbers: {}", sum_part1);
    println!("Part 2 - Sum of corrected middle page numbers: {}", sum_part2);
}

fn is_invalid_update(update: &[usize], rules: &[(usize, usize)]) -> bool {

    for rule in rules {

        let left_index = update.iter().position(|page| page == &rule.0);
        let right_index = update.iter().position(|page| page == &rule.1);

        if left_index.is_some() && right_index.is_some() && left_index.unwrap() > right_index.unwrap() {
            return true;
        }
    }

    false
}

fn fix_update(update: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {

    let update_set: HashSet<usize> = update.iter().cloned().collect();

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut indegree: HashMap<usize, usize> = HashMap::new();

    for &page in update {

        graph.entry(page).or_default();
        indegree.entry(page).or_insert(0);
    }

    for &(a, b) in rules {

        if update_set.contains(&a) && update_set.contains(&b) {

            graph.entry(a).or_default().push(b);
            *indegree.entry(b).or_insert(0) += 1;
        }
    }

    let mut result = Vec::new();

    let mut no_incoming: Vec<usize> = indegree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    no_incoming.sort_by_key(|p| update.iter().position(|x| x == p).unwrap());

    while !no_incoming.is_empty() {

        let node = no_incoming.remove(0);
        result.push(node);

        if let Some(children) = graph.get(&node) {

            for &child in children {

                let entry = indegree.get_mut(&child).unwrap();
                *entry -= 1;

                if *entry == 0 {

                    no_incoming.push(child);
                    no_incoming.sort_by_key(|p| update.iter().position(|x| x == p).unwrap());
                }
            }
        }
    }

    if result.len() != update.len() {

        let remaining: Vec<usize> = update
            .iter()
            .cloned()
            .filter(|p| !result.contains(p))
            .collect();

        result.extend(remaining);
    }

    result
}

fn get_input_data() -> InputData {

    let file_contents = fs::read_to_string("input-data")
        .expect("cant read input file");

    let data: Vec<&str> = file_contents.split("\n\n").collect();

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
    (split[0], split[1])
}

fn split_and_parse_to_usize(line: &str, delimiter: char) -> Vec<usize> {

    line.split(delimiter)
        .map(str::parse::<usize>)
        .map(|result| result.expect("failed to parse usize"))
        .collect()
}
