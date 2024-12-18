use std::fs::{self};
use regex::Regex;


fn main() {

    let instructions = get_instructions();

	let valid_sorted_instructions = get_valid_sorted_instructions(&instructions);

    println!(
		"All uncorrupted and toggled multiplications add up to: {}",
		parse_and_calculate_instructions(valid_sorted_instructions)
	);
}

fn get_valid_sorted_instructions(instructions: &str) -> Vec<&str> {

	let vaild_instructions_pattern = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

	vaild_instructions_pattern.captures_iter(instructions)
		.map(|function_capture| function_capture.get(0).unwrap().as_str())
		.collect()
}

fn parse_and_calculate_instructions(instructions: Vec<&str>) -> i32 {

	let mut calculations_enabled = true;
	let mut sum = 0;

	for instruction in instructions {

		if instruction.starts_with("do(") {

			calculations_enabled = true;
			continue;
		}

		if instruction.starts_with("don") {

			calculations_enabled = false;
			continue;
		}

		if calculations_enabled {

			sum += calculate_multiplication_instruction(instruction);
		}
	}

	sum
}

fn calculate_multiplication_instruction(mul_instruction: &str) -> i32 {

	let opening_paranthesis_index = 3;
	let closing_paranthesis_index = mul_instruction.find(')').unwrap();

	let values_to_multiply: Vec<i32> = mul_instruction[opening_paranthesis_index+1..closing_paranthesis_index]
		.split(',')
		.map(|string_value| string_value.parse::<i32>().unwrap())
		.collect();

	values_to_multiply[0] * values_to_multiply[1]
}

fn get_instructions() -> String {

    fs::read_to_string("input-data").unwrap()
}
