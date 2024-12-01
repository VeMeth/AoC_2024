use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let numbers: Vec<&str> = ip.split_whitespace().collect();
                if numbers.len() >= 2 {
                    if let Ok(left) = numbers[0].parse::<i32>() {
                        left_numbers.push(left);
                    }
                    if let Ok(right) = numbers[1].parse::<i32>() {
                        right_numbers.push(right);
                    }
                }
            }
        }
    }
 
    // Sort Arrayy
    left_numbers.sort();
    right_numbers.sort();
    let mut score = 0;
    for left in left_numbers.iter() {
        let count = right_numbers.iter()
            .filter(|&right| right == left)
            .count();
        println!("Value {} appears {} times in right list", left, count);
        let sim = left * count as i32;
        println!("Adding { } to score", sim,);
        score = score + sim;
    }

    println!("Total Score is: {}", score);

    Ok(())
}

//Read File
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}