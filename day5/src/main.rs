use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use log::{info, warn};
use petgraph::graph::{NodeIndex, DiGraph};
use rayon::prelude::*;

struct SequenceProcessor {
    rules: HashMap<i32, Vec<i32>>,
    graph: DiGraph<i32, ()>,
    node_indices: HashMap<i32, NodeIndex>,
}

impl SequenceProcessor {
    fn new(rules: HashMap<i32, Vec<i32>>) -> Self {
        let mut graph = DiGraph::with_capacity(rules.len() * 2, rules.len() * 3);
        let mut node_indices = HashMap::with_capacity(rules.len() * 2);

        // First loop: add all nodes
        for (&from, to_list) in &rules {
            node_indices.entry(from).or_insert_with(|| graph.add_node(from));
            for &to in to_list {
                node_indices.entry(to).or_insert_with(|| graph.add_node(to));
            }
        }

        // Second loop: add edges
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

        let positions: HashMap<_, _> = sequence.iter().enumerate().map(|(i, &val)| (val, i)).collect();

        for (&from, to_list) in &self.rules {
            if let Some(&from_pos) = positions.get(&from) {
                violations.extend(to_list.iter().filter_map(|&to| {
                    positions.get(&to).filter(|&&to_pos| from_pos > to_pos).map(|&to_pos| (from, to, from_pos, to_pos))
                }));
            }
        }

        (violations.is_empty(), violations)
    }

    fn attempt_reordering(&self, sequence: &[i32]) -> Option<Vec<i32>> {
        let sequence_set: HashSet<_> = sequence.iter().copied().collect();
        let mut in_degree: HashMap<i32, usize> = HashMap::new();
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new(); // Explicit type annotation here

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

        let mut queue: Vec<_> = sequence.iter().copied().filter(|&num| !in_degree.contains_key(&num)).collect();
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

    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key_str, value_str)) = line.split_once('|') {
            if let (Ok(key), Ok(value)) = (key_str.trim().parse(), value_str.trim().parse()) {
                rules.entry(key).or_insert_with(Vec::new).push(value);
            }
        } else if !line.is_empty() {
            let numbers = line
                .split(',')
                .filter_map(|num_str| num_str.trim().parse().ok())
                .collect::<Vec<i32>>();
            if !numbers.is_empty() {
                updates.push(numbers);
            }
        }
    }

    info!("Input summary - Rules: {}, Sequences: {}", rules.len(), updates.len());

    let processor = SequenceProcessor::new(rules);
    process_sequences_parallel(&processor, &updates);

    Ok(())
}

fn process_sequences_parallel(processor: &SequenceProcessor, updates: &[Vec<i32>]) {
    let (valid_sum, reordered_sum): (i32, i32) = updates
        .par_iter()
        .map(|update| {
            let (is_valid, violations) = processor.check_sequence(update);

            let valid_sum = if is_valid {
                update.get(update.len() / 2).copied().unwrap_or(0)
            } else {
                0
            };

            let reordered_sum = if !is_valid {
                if let Some(ordered) = processor.attempt_reordering(update) {
                    ordered.get(ordered.len() / 2).copied().unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            };

            (valid_sum, reordered_sum)
        })
        .reduce(|| (0, 0), |(vs1, rs1), (vs2, rs2)| (vs1 + vs2, rs1 + rs2));

    println!("Valid sequences sum: {}", valid_sum);
    println!("Reordered sequences sum: {}", reordered_sum);
}
