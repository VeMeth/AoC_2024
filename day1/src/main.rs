use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::fs;

fn main() -> io::Result<()> {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("data/input.txt") {
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
 
    // Sort Array
    left_numbers.sort();
    right_numbers.sort();
    
    // Part 1
    let mut distance = 0;
    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        let diff = left - right; 
        distance = distance + diff.abs();
    }
    
    // Part 2
    let mut score = 0;
    for left in left_numbers.iter() {
        let count = right_numbers.iter()
            .filter(|&right| right == left)
            .count();
        let sim = left * count as i32;
        score = score + sim;
    }

    println!("Total Score is: {}", score);
    println!("The Distance is: {}", distance);

    // Create data directory if it doesn't exist
    fs::create_dir_all("data")?;
    
    // Write results to file
    write_results(score, distance)?;
    
    Ok(())
}

// Read File
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Write results to file
fn write_results(score: i32, distance: i32) -> io::Result<()> {
    let mut file = File::create("data/answer.txt")?;
    writeln!(file, "{}", score)?;
    writeln!(file, "{}", distance)?;
    Ok(())
}