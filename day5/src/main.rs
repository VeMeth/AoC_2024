use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use log::{info, warn};
use petgraph::graph::{NodeIndex, DiGraph};

struct SequenceProcessor {
    rules: HashMap<i32, Vec<i32>>,
    graph: DiGraph<i32, ()>,
    node_indices: HashMap<i32, NodeIndex>,
}

impl SequenceProcessor {
    fn new(rules: HashMap<i32, Vec<i32>>) -> Self {
        let mut graph = DiGraph::with_capacity(rules.len() * 2, rules.len() * 3);
        let mut node_indices = HashMap::with_capacity(rules.len() * 2);
        
        let unique_nodes: HashSet<_> = rules.keys()
            .copied()  // Change here: get owned values
            .chain(rules.values().flatten().copied())  // And here
            .collect();
            
        for num in unique_nodes {  // Remove reference
            let idx = graph.add_node(num);
            node_indices.insert(num, idx);
        }
        
        for (&from, to_list) in &rules {
            let from_idx = node_indices[&from];
            for &to in to_list {
                let to_idx = node_indices[&to];
                graph.add_edge(from_idx, to_idx, ());
            }
        }

        Self { rules, graph, node_indices }
    }

    fn check_sequence(&self, sequence: &[i32]) -> (bool, Vec<(i32, i32, usize, usize)>) {
        let mut violations = Vec::new();
        let len = sequence.len();
        
        // Create position lookup table
        let mut positions = HashMap::with_capacity(len);
        for (i, &val) in sequence.iter().enumerate() {
            positions.insert(val, i);
        }
        
        // Check violations using position lookup
        for (&from, to_list) in &self.rules {
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

    fn attempt_reordering(&self, sequence: &[i32]) -> Option<Vec<i32>> {
        let sequence_set: HashSet<_> = sequence.iter().copied().collect();
        let mut in_degree: HashMap<i32, usize> = HashMap::new();
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        
        // Build adjacency list and calculate in-degrees
        for &num in sequence {
            if let Some(deps) = self.rules.get(&num) {
                for &dep in deps {
                    if sequence_set.contains(&dep) {
                        adj_list.entry(num).or_default().push(dep);
                        *in_degree.entry(dep).or_default() += 1;
                    }
                }
            }
        }
        
        // Find nodes with no incoming edges
        let mut queue: Vec<i32> = sequence.iter()
            .copied()
            .filter(|&num| !in_degree.contains_key(&num))
            .collect();
        
        let mut result = Vec::with_capacity(sequence.len());
        
        while let Some(node) = queue.pop() {
            result.push(node);
            
            if let Some(neighbors) = adj_list.get(&node) {
                for &next in neighbors {
                    if let Some(degree) = in_degree.get_mut(&next) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(next);
                        }
                    }
                }
            }
        }
        
        if result.len() == sequence.len() {
            Some(result)
        } else {
            None
        }
    }

  
}

fn main() -> io::Result<()> {
    env_logger::init();
    
    let path = Path::new("data/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut rules = HashMap::with_capacity(100);
    let mut updates = Vec::with_capacity(1000);
    
    // Optimized file parsing
    for line in reader.lines() {
        let line = line?;
        
        if let Some((key_str, value_str)) = line.split_once('|') {
            if let (Ok(key), Ok(value)) = (key_str.trim().parse(), value_str.trim().parse()) {
                rules.entry(key)
                    .or_insert_with(|| Vec::with_capacity(5))
                    .push(value);
            }
        } else if !line.is_empty() {
            let mut numbers = Vec::with_capacity(10);
            for num_str in line.split(',') {
                if let Ok(num) = num_str.trim().parse() {
                    numbers.push(num);
                }
            }
            if !numbers.is_empty() {
                updates.push(numbers);
            }
        }
    }
    
    info!("Input summary - Rules: {}, Sequences: {}", rules.len(), updates.len());
    
    let processor = SequenceProcessor::new(rules);
    process_sequences(&processor, &updates);
    Ok(())
}

fn process_sequences(processor: &SequenceProcessor, updates: &[Vec<i32>]) {
    let mut valid_sum = 0;
    let mut reordered_sum = 0;
    
    for (line_index, update) in updates.iter().enumerate() {
        let (is_valid, violations) = processor.check_sequence(update);
        
        if is_valid {
            if let Some(&middle) = update.get(update.len() / 2) {
                valid_sum += middle;
            }
        } else {
            warn!("Invalid sequence {} with {} violations", line_index + 1, violations.len());
            
            if let Some(ordered) = processor.attempt_reordering(update) {
                if let Some(&middle) = ordered.get(ordered.len() / 2) {
                    reordered_sum += middle;
                }
            }
        }
    }
    
    println!("Valid sequences sum: {}", valid_sum);
    println!("Reordered sequences sum: {}", reordered_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_validation() {
        let mut rules = HashMap::new();
        rules.insert(1, vec![2, 3]);
        rules.insert(2, vec![3]);
        
        let processor = SequenceProcessor::new(rules);
        
        let (is_valid, violations) = processor.check_sequence(&[2, 3, 1]);
        assert!(is_valid);
        assert!(violations.is_empty());
        
        let (is_valid, violations) = processor.check_sequence(&[1, 3, 2]);
        assert!(!is_valid);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_reordering() {
        let mut rules = HashMap::new();
        rules.insert(1, vec![2]);
        rules.insert(2, vec![3]);
        
        let processor = SequenceProcessor::new(rules);
        
        let result = processor.attempt_reordering(&[1, 3, 2]);
        assert!(result.is_some());
        if let Some(ordered) = result {
            assert!(ordered.windows(2).all(|w| {
                !processor.rules.get(&w[1])
                    .map_or(false, |deps| deps.contains(&w[0]))
            }));
        }
    }
}