use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read sequences from file
    let mut all_sequences: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let numbers: Vec<i32> = ip
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();
                all_sequences.push(numbers);
            }
        }
    }
    let mut hits = 0;   
    // Process each sequence
    for (_i, sequence) in all_sequences.iter().enumerate() {
        //println!("\nAnalyzing Sequence {}: {:?}", i, sequence);
        
        if sequence.len() < 2 {
            //println!("Sequence too short to analyze");
            continue;
        }

        let mut is_valid = true;
        let mut prev_value = sequence[0];
        let mut direction_set = false;
        let mut is_decreasing = false;

        // Check each pair of numbers in the sequence
        for window in sequence.windows(2) {
            let curr_value = window[1];
            let difference = curr_value - prev_value;

            // Check if difference is zero
            if difference == 0 {
                //println!("No change between values: {} to {}", prev_value, curr_value);
                is_valid = false;
                break;
            }

            // Check if values are within range (1-3)
            if difference.abs() >= 4 || difference.abs() < 1 {
                //println!("Invalid change between {} to {}: difference = {}", 
                //    prev_value, curr_value, difference);
                is_valid = false;
                break;
            }

            // Set or verify direction
            if !direction_set {
                is_decreasing = difference < 0;
                direction_set = true;
                //println!("Direction set to {}", if is_decreasing { "decreasing" } else { "increasing" });
            } else {
                let current_decreasing = difference < 0;
                if current_decreasing != is_decreasing {
                    //println!("Direction changed: {} to {}", prev_value, curr_value);
                    is_valid = false;
                    break;
                }
            }

            prev_value = curr_value;
        }

        if is_valid {
            //println!("Valid sequence: maintains direction and changes by 1-3 units");
            hits += 1;
        } else {
            //println!("Invalid sequence: violates direction or change constraints");
        }
    }

    println!("\nNumber of valid sequences: {}", hits);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}