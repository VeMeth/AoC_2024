use std::fs;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn evaluate(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => {
                // Convert both numbers to strings, concatenate, then parse back to i64
                let concat = format!("{}{}", result, numbers[i + 1]);
                result = concat.parse().unwrap();
            }
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn can_make_value(eq: &Equation, include_concat: bool) -> bool {
    let ops = if include_concat {
        vec!['+', '*', '|']
    } else {
        vec!['+', '*']
    };
    
    let n_slots = eq.numbers.len() - 1;
    let total_combinations = ops.len().pow(n_slots as u32);
    
    // Try all possible combinations of operators
    for combo in 0..total_combinations {
        let mut operators = Vec::new();
        let mut temp = combo;
        for _ in 0..n_slots {
            operators.push(ops[temp % ops.len()]);
            temp /= ops.len();
        }
        
        if evaluate(&eq.numbers, &operators) == eq.test_value {
            return true;
        }
    }
    false
}

fn parse_line(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(": ").collect();
    let test_value = parts[0].parse().unwrap();
    let numbers: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    Equation { test_value, numbers }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input file");
    let equations: Vec<Equation> = input.lines().map(parse_line).collect();
    
    // Part 1: Only + and *
    let result1: i64 = equations
        .iter()
        .filter(|eq| can_make_value(eq, false))
        .map(|eq| eq.test_value)
        .sum();
        
    println!("Part 1 - Total calibration result: {}", result1);
    
    // Part 2: Including concatenation
    let result2: i64 = equations
        .iter()
        .filter(|eq| can_make_value(eq, true))
        .map(|eq| eq.test_value)
        .sum();
        
    println!("Part 2 - Total calibration result: {}", result2);

    // Write results to file
    fs::write(
        "data/answer.txt",
        format!("{}\n{}", result1, result2)
    ).expect("Failed to write answer file");
}
