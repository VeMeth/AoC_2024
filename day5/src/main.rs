use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use log::{info, warn};
use petgraph::Graph;
use petgraph::algo::toposort;


fn main() -> io::Result<()> {
    env_logger::init();
    
    let path = Path::new("data/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut rules: HashSet<(i32, i32)> = HashSet::with_capacity(1200);
    let mut updates: Vec<Vec<i32>> = Vec::with_capacity(1000);
    
    for line in reader.lines() {
        let line = line?;
        
        if let Some((key_str, value_str)) = line.split_once('|') {
            if let (Ok(key), Ok(value)) = (key_str.trim().parse(), value_str.trim().parse()) {
                rules.insert((key, value));
            }
        } else if !line.is_empty() {
            let numbers: Vec<i32> = line
                .split(',')
                .filter_map(|n| n.trim().parse().ok())
                .collect();
            if !numbers.is_empty() {
                updates.push(numbers);
            }
        }
    }
    
    info!("Input summary - Rules: {}, Sequences: {}", rules.len(), updates.len());
    process_sequences(&rules, &updates);
    Ok(())
}

fn process_sequences(rules: &HashSet<(i32, i32)>, updates: &[Vec<i32>]) {
    let mut valid_sum = 0;
    let mut reordered_sum = 0;
    
    for (line_index, update) in updates.iter().enumerate() {
        let is_valid  = check_sequence(rules, update);
        
        if is_valid {
            if let Some(&middle) = update.get(update.len() / 2) {
                valid_sum += middle;
            }
        } else {
            //warn!("Invalid sequence {} with {} violations", line_index + 1, violations.len());
            
            if let Some(ordered) = attempt_reordering(rules, update) {
                if let Some(&middle) = ordered.get(ordered.len() / 2) {
                    reordered_sum += middle;
                }
            }
        }
    }
    
    println!("Valid sequences sum: {}", valid_sum);
    println!("Reordered sequences sum: {}", reordered_sum);
}

#[inline]
fn check_sequence(rules: &HashSet<(i32,i32)>, sequence: &[i32]) -> bool {
    for i in 0..sequence.len() - 1 {
        for j in i + 1..sequence.len() {
            if rules.contains(&(sequence[j], sequence[i])) {
                return false;
            }
        }
    }
    return true;
}

fn attempt_reordering(rules: &HashSet<(i32, i32)>, sequence: &[i32]) -> Option<Vec<i32>> {
    let mut graph = Graph::<i32, ()>::new();
    let mut node_indices = HashMap::new();
    
    // Create nodes
    for &num in sequence {
        let idx = graph.add_node(num);
        node_indices.insert(num, idx);
    }

    for &from in sequence {
        for &to in sequence {
            if rules.contains(&(from, to)) {
                graph.add_edge(*node_indices.get(&from)?, *node_indices.get(&to)?, ());
            }
        }
    }
    
    // Perform topological sort
    match toposort(&graph, None) {
        Ok(indices) => Some(indices.into_iter()
            .map(|idx| graph[idx])
            .collect()),
        Err(_) => None
    }
}