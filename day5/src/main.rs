use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use log::{info, warn};
use rayon::prelude::*;

fn main() -> io::Result<()> {
    env_logger::init();
    
    let path = Path::new("data/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    // Pre-allocate with capacity hints
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::with_capacity(100);
    let mut updates: Vec<Vec<i32>> = Vec::with_capacity(1000);
    
    // Single-pass file reading with more efficient parsing
    for line in reader.lines() {
        let line = line?;
        
        if let Some(pipe_idx) = line.find('|') {
            // Avoid allocation of temporary strings
            let (key_str, value_str) = line.split_at(pipe_idx);
            if let (Ok(key), Ok(value)) = (
                key_str.trim().parse(),
                value_str[1..].trim().parse()
            ) {
                rules.entry(key)
                    .or_insert_with(|| Vec::with_capacity(5))
                    .push(value);
            }
        } else if !line.is_empty() {
            // More efficient number parsing
            let mut numbers = Vec::with_capacity(10);
            let mut num_start = 0;
            
            for (i, c) in line.chars().enumerate() {
                if c == ',' {
                    if let Ok(num) = line[num_start..i].trim().parse() {
                        numbers.push(num);
                    }
                    num_start = i + 1;
                }
            }
            // Handle last number
            if let Ok(num) = line[num_start..].trim().parse() {
                numbers.push(num);
            }
            
            if !numbers.is_empty() {
                updates.push(numbers);
            }
        }
    }
    
    info!("Input summary - Rules: {}, Sequences: {}", rules.len(), updates.len());
    
    // Build graph once
    let graph = build_dependency_graph(&rules);
    process_sequences(&rules, &updates, &graph);
    Ok(())
}

fn process_sequences(
    rules: &HashMap<i32, Vec<i32>>,
    updates: &[Vec<i32>],
    graph: &HashMap<i32, HashSet<i32>>
) {
    // Use parallel iterator for processing sequences
    let results: Vec<_> = updates.par_iter()
        .map(|update| {
            let (is_valid, violations) = check_sequence(rules, update);
            
            if is_valid {
                update.get(update.len() / 2).copied()
            } else {
                warn!("Invalid sequence with {} violations", violations.len());
                attempt_reordering(rules, update, graph)
                    .and_then(|ordered| ordered.get(ordered.len() / 2).copied())
            }
        })
        .collect();
    
    // Calculate sums
    let (valid_sum, reordered_sum) = results.iter()
        .fold((0, 0), |(valid, reorder), &result| {
            match result {
                Some(val) => (valid + val, reorder),
                None => (valid, reorder)
            }
        });
    
    println!("Valid sequences sum: {}", valid_sum);
    println!("Reordered sequences sum: {}", reordered_sum);
}

#[inline(always)]
fn check_sequence(rules: &HashMap<i32, Vec<i32>>, sequence: &[i32]) -> (bool, Vec<(i32, i32, usize, usize)>) {
    let mut violations = Vec::new();
    
    // Use a fixed-size array for position lookup if possible
    let mut positions = [usize::MAX; 10000];
    for (i, &val) in sequence.iter().enumerate() {
        positions[val as usize] = i;
    }
    
    'outer: for (&from, to_list) in rules {
        let from_pos = positions[from as usize];
        if from_pos != usize::MAX {
            for &to in to_list {
                let to_pos = positions[to as usize];
                if to_pos != usize::MAX && from_pos > to_pos {
                    violations.push((from, to, from_pos, to_pos));
                    // Early exit if we only need to know if it's valid
                    if violations.len() == 1 {
                        break 'outer;
                    }
                }
            }
        }
    }
    
    (violations.is_empty(), violations)
}

fn attempt_reordering(
    rules: &HashMap<i32, Vec<i32>>,
    sequence: &[i32],
    graph: &HashMap<i32, HashSet<i32>>
) -> Option<Vec<i32>> {
    let mut ordered = Vec::with_capacity(sequence.len());
    let mut visited = HashSet::with_capacity(sequence.len());
    let mut temp_visited = HashSet::new();
    
    // Custom topological sort implementation
    fn visit(
        node: i32,
        graph: &HashMap<i32, HashSet<i32>>,
        visited: &mut HashSet<i32>,
        temp_visited: &mut HashSet<i32>,
        ordered: &mut Vec<i32>
    ) -> bool {
        if temp_visited.contains(&node) {
            return false;
        }
        if visited.contains(&node) {
            return true;
        }
        
        temp_visited.insert(node);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visit(neighbor, graph, visited, temp_visited, ordered) {
                    return false;
                }
            }
        }
        
        temp_visited.remove(&node);
        visited.insert(node);
        ordered.push(node);
        true
    }
    
    for &node in sequence {
        if !visited.contains(&node) {
            if !visit(node, graph, &mut visited, &mut temp_visited, &mut ordered) {
                return None;
            }
        }
    }
    
    Some(ordered)
}

fn build_dependency_graph(rules: &HashMap<i32, Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(rules.len());
    
    for (&from, to_list) in rules {
        let entry = graph.entry(from).or_insert_with(|| HashSet::with_capacity(to_list.len()));
        entry.extend(to_list.iter().copied());
        
        // Ensure all destination nodes have an entry in the graph
        for &to in to_list {
            graph.entry(to).or_insert_with(HashSet::new);
        }
    }
    
    graph
}