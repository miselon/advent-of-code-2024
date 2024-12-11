use std::fs::File;
use std::io::{self, BufRead};

fn main() {

    let lists = get_sorted_input_data();

    println!("Total distance between location IDs is: {}", get_total_distance_between_lists(&lists));
    println!("Similarity score of location IDs is: {}", get_similarity_score(lists));
}

fn get_similarity_score(lists:(Vec<i32>, Vec<i32>)) -> i32 {

    let lists_length = lists.0.len();
    let mut similarity_score = 0;

    for left_index in 0 .. lists_length {

        let left = lists.0[left_index];
        let mut occurences = 0;

        for right_index in 0 .. lists_length {

            let right = lists.1[right_index];
    
            if left == right {

                occurences += 1;
            }
        }

        similarity_score += left * occurences;
    }

    similarity_score
}

fn get_total_distance_between_lists(lists:&(Vec<i32>, Vec<i32>)) -> i32 {

    let mut sum = 0;

    for index in 0 .. lists.0.len() {

        let left = lists.0[index];
        let right = lists.1[index];

        let mut diff = left - right;

        if diff < 0 {

            diff = diff * -1;
        }

        sum = sum + diff;
    }

    sum
}

fn get_sorted_input_data() -> (Vec<i32>, Vec<i32>) {

    let file = File::open("input-data").unwrap();
    let reader = io::BufReader::new(file);

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for line in reader.lines() {

        let line = line.unwrap();
        let parts: Vec<&str> = line.split_terminator("   ").collect();

        left_list.push(parts[0].parse().unwrap());
        right_list.push(parts[1].parse().unwrap());
    }

    left_list.sort(); 
    right_list.sort();

    (left_list, right_list)
}
