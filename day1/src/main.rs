use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {

    let file = File::open("C:\\Users\\Mateusz\\Documents\\workspace\\advent-of-code-2024\\day1\\src\\input")?;
    let reader = io::BufReader::new(file);

    let mut leftList: Vec<i32> = vec![];
    let mut rightList: Vec<i32> = vec![];

    for line in reader.lines() {

        let line = line?;
        let parts: Vec<&str> = line.split_terminator("   ").collect();

        leftList.push(parts[0].parse().unwrap());
        rightList.push(parts[1].parse().unwrap());
    }

    leftList.sort();
    rightList.sort();

    let mut sum = 0;

    for (index, value) in leftList.iter().enumerate() {

        let left = leftList[index];
        let right = rightList[index];

        let mut diff = left - right;

        if diff < 0 {

            diff = diff * -1;
        }

        sum = sum + diff;
    }

    println!("sum is: {}", sum);

    Ok(())
}

