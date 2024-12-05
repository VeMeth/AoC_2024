use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use log::{debug, info, warn};

fn main() -> io::Result<()> {
    env_logger::init();
    
    let path = Path::new("data/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    
    debug!("Reading input file...");
    
    for line in lines {
        let line = line?;
        
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                if let (Ok(key), Ok(value)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                    debug!("Found rule: {} -> {}", key, value);
                    rules.entry(key).or_insert_with(Vec::new).push(value);
                }
            }
        } else if !line.trim().is_empty() {
            let numbers: Vec<i32> = line
                .split(',')
                .filter_map(|n| n.trim().parse().ok())
                .collect();
            if !numbers.is_empty() {
                debug!("Found sequence: {:?}", numbers);
                updates.push(numbers);
            }
        }
    }
    
    info!("Input summary - Rules: {}, Sequences: {}", rules.len(), updates.len());
    
    process_sequences(&rules, &updates);
    
    Ok(())
}

fn process_sequences(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut valid_sum = 0;
    let mut reordered_sum = 0;
    info!("Starting sequence processing");
    
    for (line_index, update) in updates.iter().enumerate() {
        debug!("Processing sequence {}: {:?}", line_index + 1, update);
        
        let (is_valid, violations) = check_sequence(rules, update);
        
        if is_valid {
            debug!("Sequence {} is valid", line_index + 1);
            if !update.is_empty() {
                let middle = update[update.len() / 2];
                debug!("Middle value for sequence {}: {}", line_index + 1, middle);
                valid_sum += middle;
            }
        } else {
            warn!("Invalid sequence {} detected", line_index + 1);
            for (from, to, from_pos, to_pos) in &violations {
                debug!("Violation in sequence {}: {} at pos {} must come before {} at pos {}", 
                    line_index + 1, from, from_pos, to, to_pos);
            }
            
            match attempt_reordering(rules, update) {
                Some(ordered) => {
                    debug!("Successfully reordered sequence {}: {:?}", line_index + 1, ordered);
                    if !ordered.is_empty() {
                        let middle = ordered[ordered.len() / 2];
                        debug!("Middle value after reordering sequence {}: {}", line_index + 1, middle);
                        reordered_sum += middle;
                    }
                }
                None => warn!("Could not find valid ordering for sequence {}", line_index + 1)
            }
        }
    }
    
    println!("Sum of middle values from valid sequences: {}", valid_sum);
    println!("Sum of middle values from reordered sequences: {}", reordered_sum);
}

fn check_sequence(rules: &HashMap<i32, Vec<i32>>, sequence: &Vec<i32>) -> (bool, Vec<(i32, i32, usize, usize)>) {
    debug!("Checking sequence validity: {:?}", sequence);
    let mut is_valid = true;
    let mut violations = Vec::new();
    
    for (&from, to_list) in rules {
        if sequence.contains(&from) {
            let from_pos = sequence.iter().position(|&x| x == from).unwrap();
            debug!("Checking rules for {} at position {}", from, from_pos);
            
            for &to in to_list {
                if sequence.contains(&to) {
                    let to_pos = sequence.iter().position(|&x| x == to).unwrap();
                    debug!("Found dependent value {} at position {}", to, to_pos);
                    
                    if from_pos > to_pos {
                        is_valid = false;
                        violations.push((from, to, from_pos, to_pos));
                        debug!("Found violation: {} at {} must come before {} at {}", 
                            from, from_pos, to, to_pos);
                    }
                }
            }
        }
    }
    
    if is_valid {
        debug!("Sequence passed validation");
    } else {
        debug!("Sequence failed validation with {} violations", violations.len());
    }
    
    (is_valid, violations)
}

fn attempt_reordering(rules: &HashMap<i32, Vec<i32>>, sequence: &Vec<i32>) -> Option<Vec<i32>> {
    debug!("Attempting to reorder sequence: {:?}", sequence);
    
    // Create graph
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (&from, to_list) in rules {
        for &to in to_list {
            graph.entry(from).or_default().insert(to);
        }
    }
    debug!("Built dependency graph with {} nodes", graph.len());
    
    // Calculate in-degree for each node
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    for &num in sequence {
        in_degree.insert(num, 0);
    }
    
    for &num in sequence {
        if let Some(deps) = graph.get(&num) {
            for &dep in deps {
                if sequence.contains(&dep) {
                    *in_degree.entry(dep).or_default() += 1;
                }
            }
        }
    }
    debug!("Initial in-degrees: {:?}", in_degree);
    
    // Perform topological sort
    let mut result = Vec::new();
    let mut queue: Vec<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&num, _)| num)
        .collect();
    
    debug!("Starting sort with initial queue: {:?}", queue);
    
    while !queue.is_empty() {
        let current = queue.remove(0);
        result.push(current);
        debug!("Processing node {}, result so far: {:?}", current, result);
        
        if let Some(deps) = graph.get(&current) {
            for &dep in deps {
                if sequence.contains(&dep) {
                    *in_degree.get_mut(&dep).unwrap() -= 1;
                    if in_degree[&dep] == 0 {
                        queue.push(dep);
                        debug!("Adding {} to queue", dep);
                    }
                }
            }
        }
    }
    
    // Add remaining numbers
    for &num in sequence {
        if !result.contains(&num) {
            debug!("Adding remaining number {} to result", num);
            result.push(num);
        }
    }
    
    // Verify the ordering is valid
    let (is_valid, violations) = check_sequence(rules, &result);
    if is_valid {
        debug!("Successfully reordered sequence");
        Some(result)
    } else {
        warn!("Reordering failed with {} violations", violations.len());
        None
    }
}