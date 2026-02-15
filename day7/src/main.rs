use std::fs::read_to_string;

fn main () {

    let input_content = read_to_string ("input-data")
        .expect ("Should have been able to read the file");

    let mut total_calibration_result: u64 = 0;

    for line in input_content.lines () {

        if line.is_empty () {

            continue;
        }

        let parts: Vec<&str> = line.split (": ").collect ();

        let target_value: u64 = parts[0].parse ().expect ("Invalid target value");

        let numbers: Vec<u64> = parts[1]
            .split_whitespace ()
            .map (|num| num.parse ().expect ("Invalid number"))
            .collect ();

        if is_equation_valid (target_value, &numbers) {

            total_calibration_result += target_value;
        }
    }

    println! ("{}", total_calibration_result);
}

fn is_equation_valid (target_value: u64, numbers: &[u64]) -> bool {

    if numbers.is_empty () {

        return false;
    }

    return evaluate_recursively (target_value, numbers[0], &numbers[1..]);
}

fn evaluate_recursively (target_value: u64, current_accumulated_value: u64, remaining_numbers: &[u64]) -> bool {

    if remaining_numbers.is_empty () {

        return current_accumulated_value == target_value;
    }

    if current_accumulated_value > target_value {

        return false;
    }

    let next_number = remaining_numbers[0];

    let rest_of_numbers = &remaining_numbers[1..];

    if evaluate_recursively (target_value, current_accumulated_value + next_number, rest_of_numbers) {

        return true;
    }

    if evaluate_recursively (target_value, current_accumulated_value * next_number, rest_of_numbers) {

        return true;
    }

    return false;
}