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
    
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::with_capacity(100);
    let mut updates: Vec<Vec<i32>> = Vec::with_capacity(1000);
    
    for line in reader.lines() {
        let line = line?;
        
        if let Some((key_str, value_str)) = line.split_once('|') {
            if let (Ok(key), Ok(value)) = (key_str.trim().parse(), value_str.trim().parse()) {
                rules.entry(key).or_insert_with(|| Vec::with_capacity(5)).push(value);
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

fn process_sequences(rules: &HashMap<i32, Vec<i32>>, updates: &[Vec<i32>]) {
    let mut valid_sum = 0;
    let mut reordered_sum = 0;
    
    let graph = build_dependency_graph(rules);
    
    for (line_index, update) in updates.iter().enumerate() {
        let (is_valid, violations) = check_sequence(rules, update);
        
        if is_valid {
            if let Some(&middle) = update.get(update.len() / 2) {
                valid_sum += middle;
            }
        } else {
            warn!("Invalid sequence {} with {} violations", line_index + 1, violations.len());
            
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
fn check_sequence(rules: &HashMap<i32, Vec<i32>>, sequence: &[i32]) -> (bool, Vec<(i32, i32, usize, usize)>) {
    let mut violations = Vec::new();
    
    let positions: HashMap<i32, usize> = sequence.iter()
        .enumerate()
        .map(|(i, &val)| (val, i))
        .collect();
    
    for (&from, to_list) in rules {
        if let Some(&from_pos) = positions.get(&from) {
            for &to in to_list {
                if let Some(&to_pos) = positions.get(&to) {
                    if from_pos > to_pos {
                        violations.push((from, to, from_pos, to_pos));
                    }
                }
            }
        }
    }
    
    (violations.is_empty(), violations)
}

fn build_dependency_graph(rules: &HashMap<i32, Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (&from, to_list) in rules {
        graph.entry(from)
            .or_insert_with(HashSet::new)
            .extend(to_list.iter().copied());
    }
    graph
}

fn attempt_reordering(rules: &HashMap<i32, Vec<i32>>, sequence: &[i32]) -> Option<Vec<i32>> {
    let mut graph = Graph::<i32, ()>::new();
    let mut node_indices = HashMap::new();
    
    // Create nodes
    for &num in sequence {
        let idx = graph.add_node(num);
        node_indices.insert(num, idx);
    }
    
    // Add edges
    for (&from, to_list) in rules {
        if let Some(&from_idx) = node_indices.get(&from) {
            for &to in to_list {
                if let Some(&to_idx) = node_indices.get(&to) {
                    graph.add_edge(from_idx, to_idx, ());
                }
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