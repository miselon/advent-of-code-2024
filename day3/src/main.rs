use std::fs::{self};
use regex::Regex;


fn main() {

    let instructions = get_instructions();

    println!("All uncorrupted multiplications add up to: {}", get_uncorrupted_multiplications_sum(&instructions));
}

fn get_uncorrupted_multiplications_sum(instructions: &str) -> i32 {

	let vaild_multiplication_pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

	vaild_multiplication_pattern.captures_iter(instructions)
		.map(|function_capture| function_capture.get(0))
		.map(|function_string| parse_multiplication_function(function_string.unwrap().as_str()))
		.map(|numbers| numbers.0 * numbers.1)
		.sum()
}

fn parse_multiplication_function(function_string: &str) -> (i32, i32) {

	let opening_parenthesis_index = 4;
	let closing_parenthesis_index = function_string.find(')').unwrap();

	let some: Vec<i32> = function_string[opening_parenthesis_index .. closing_parenthesis_index]
		.split(',')
		.map(|string| string.parse().unwrap())
		.collect();

	(some[0], some[1])
}

fn get_instructions() -> String {

    fs::read_to_string("input-data").unwrap()
}
