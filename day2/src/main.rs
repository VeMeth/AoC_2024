use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read sequences from file
    let mut all_sequences: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("data/input.txt") {
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
    let mut truehits = 0;
    // Bruteforce
    for (i, sequence) in all_sequences.iter().enumerate() {
        //println!("\nAnalyzing Sequence {}: {:?}", i, sequence);
        
        if sequence.len() < 2 {
            //println!("Sequence too short to analyze");
            continue;
        }

        // Try the original sequence first
        if is_valid_sequence(sequence) {
            //println!("Original sequence is valid");
            hits += 1;
            truehits += 1;
            continue;
        }

        // Try removing each value one at a time
        let mut found_valid = false;
        for skip_index in 0..sequence.len() {
            let modified_sequence: Vec<i32> = [&sequence[..skip_index], &sequence[skip_index + 1..]].concat();

            if modified_sequence.len() >= 2 && is_valid_sequence(&modified_sequence) {
                //println!("Found valid sequence by removing index {}: {:?}", skip_index, modified_sequence);
                hits += 1;
                found_valid = true;
                break;
            }
        }

        if !found_valid {
            //println!("Could not find a valid sequence by removing one number");
        }
    }
    println!("\nNumber of valid sequences: {}", truehits);
    println!("\nNumber of valid sequences (including fixed ones): {}", hits);
    

}



fn is_valid_sequence(sequence: &[i32]) -> bool {
    if sequence.len() < 2 {
        return false;
    }

    let mut prev_value = sequence[0];
    let mut direction_set = false;
    let mut is_decreasing = false;

    for window in sequence.windows(2) {
        let curr_value = window[1];
        let difference = curr_value - prev_value;

        // min +1
        if difference == 0 {
            return false;
        }

        // range
        if difference.abs() >= 4 || difference.abs() < 1 {
            return false;
        }

        // direction
        if !direction_set {
            is_decreasing = difference < 0;
            direction_set = true;
        } else {
            let current_decreasing = difference < 0;
            if current_decreasing != is_decreasing {
                return false;
            }
        }

        prev_value = curr_value;
    }

    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}