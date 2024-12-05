use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
use log::{info, warn};
use rayon::prelude::*;

fn main() -> io::Result<()> {
    env_logger::init();
    
    let path = Path::new("data/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::with_capacity(100);
    let mut updates: Vec<Vec<i32>> = Vec::with_capacity(1000);
    
    for line in reader.lines() {
        let line = line?;
        
        if let Some(pipe_idx) = line.find('|') {
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
            if let Ok(num) = line[num_start..].trim().parse() {
                numbers.push(num);
            }
            
            if !numbers.is_empty() {
                updates.push(numbers);
            }
        }
    }
    
    info!("Input summary - Rules: {}, Sequences: {}", rules.len(), updates.len());
    
    let graph = build_dependency_graph(&rules);
    process_sequences(&rules, &updates, &graph);
    Ok(())
}

fn process_sequences(
    rules: &HashMap<i32, Vec<i32>>,
    updates: &[Vec<i32>],
    graph: &HashMap<i32, HashSet<i32>>
) {
    let results: Vec<_> = updates.par_iter()
        .map(|update| {
            let (is_valid, violations) = check_sequence(rules, update);
            
            if is_valid {
                (true, update.get(update.len() / 2).copied())
            } else {
                warn!("Invalid sequence with {} violations", violations.len());
                let ordered = attempt_reordering(update, graph);
                (false, ordered.and_then(|ord| ord.get(ord.len() / 2).copied()))
            }
        })
        .collect();
    
    let (valid_sum, reordered_sum) = results.iter()
        .fold((0, 0), |(valid, reorder), &(is_valid, result)| {
            if let Some(val) = result {
                if is_valid {
                    (valid + val, reorder)
                } else {
                    (valid, reorder + val)
                }
            } else {
                (valid, reorder)
            }
        });
    
    println!("Valid sequences sum: {}", valid_sum);
    println!("Reordered sequences sum: {}", reordered_sum);
}

#[inline(always)]
fn check_sequence(rules: &HashMap<i32, Vec<i32>>, sequence: &[i32]) -> (bool, Vec<(i32, i32, usize, usize)>) {
    let mut violations = Vec::new();
    
    let mut positions = vec![usize::MAX; 10000];
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
    sequence: &[i32],
    graph: &HashMap<i32, HashSet<i32>>
) -> Option<Vec<i32>> {
    // Create in-degree count for each node
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut node_set: HashSet<i32> = HashSet::new();
    
    // Initialize node set with all sequence elements
    for &num in sequence {
        node_set.insert(num);
    }
    
    // Calculate in-degrees
    for &node in &node_set {
        if let Some(deps) = graph.get(&node) {
            for &dep in deps {
                if node_set.contains(&dep) {
                    *in_degree.entry(dep).or_insert(0) += 1;
                }
            }
        }
    }
    
    // Find nodes with no incoming edges
    let mut queue: VecDeque<i32> = node_set.iter()
        .filter(|&&node| !in_degree.contains_key(&node))
        .copied()
        .collect();
    
    let mut result = Vec::with_capacity(sequence.len());
    let mut remaining: HashSet<_> = sequence.iter().copied().collect();
    
    // Process nodes level by level
    while let Some(node) = queue.pop_front() {
        if remaining.remove(&node) {
            result.push(node);
            
            // Update in-degrees of dependent nodes
            if let Some(deps) = graph.get(&node) {
                for &dep in deps {
                    if let Some(in_deg) = in_degree.get_mut(&dep) {
                        *in_deg -= 1;
                        if *in_deg == 0 {
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }
    }
    
    // Add any remaining nodes that weren't part of the dependency graph
    for &num in sequence {
        if remaining.contains(&num) {
            result.push(num);
        }
    }
    
    if result.len() == sequence.len() {
        Some(result)
    } else {
        None
    }
}

fn build_dependency_graph(rules: &HashMap<i32, Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(rules.len());
    
    for (&from, to_list) in rules {
        let entry = graph.entry(from).or_insert_with(|| HashSet::with_capacity(to_list.len()));
        entry.extend(to_list.iter().copied());
        
        for &to in to_list {
            graph.entry(to).or_insert_with(HashSet::new);
        }
    }
    
    graph
}