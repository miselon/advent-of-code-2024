use std::fs::File;
use std::io::{self, BufRead};

fn main() {

    let reports = get_reports();

    println!("Amount of safe reports: {}", get_amount_of_safe_reports(reports));
}

fn get_amount_of_safe_reports(reports: Vec<Vec<i32>>) -> usize {

    reports.into_iter()
        .filter(|report| is_report_safe(report) || is_report_safe_with_problem_dampener(report))
        .count()
}

fn is_report_safe_with_problem_dampener(report: &Vec<i32>) -> bool {
    
    for index in 0 .. report.len() {

        let mut dampened_report = report.clone();

        dampened_report.remove(index);

        if is_report_safe(&dampened_report) {

            return true;
        }
    }

    false
}

fn is_report_safe(report: &Vec<i32>) -> bool {

    let mut initially_increasing = report[0] < report[1];

    for index in 1 .. report.len() {

        let previous = report[index - 1];
        let current = report[index];

        if previous == current {

            return false;
        }

        let increasing = previous < current;

        if initially_increasing && !increasing || !initially_increasing && increasing {

            return false;
        }

        let difference = (previous - current).abs();

        if difference < 1 || difference > 3 {

            return false;
        }

        initially_increasing = increasing;
    }

    true
}

fn get_reports() -> Vec<Vec<i32>> {

    let file = File::open("input-data").unwrap();
    let reader = io::BufReader::new(file);

    reader.lines()
        .into_iter()
        .map(|line| get_single_report(line.unwrap()))
        .collect()
}

fn get_single_report(line: String) -> Vec<i32> {

    line.split_whitespace()
        .map(|level_as_string| level_as_string.parse::<i32>().unwrap())
        .collect()
}