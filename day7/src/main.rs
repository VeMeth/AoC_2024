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
                // Optimize concatenation by using arithmetic instead of string operations
                let mut num2 = numbers[i + 1];
                let mut multiplier = 1;
                while num2 > 0 {
                    multiplier *= 10;
                    num2 /= 10;
                }
                result = result * multiplier + numbers[i + 1];
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
    
    // Early exit: For single operator cases, check direct solutions first
    if n_slots == 1 {
        // Check concatenation
        if include_concat {
            let mut multiplier = 1;
            let mut temp = eq.numbers[1];
            while temp > 0 {
                multiplier *= 10;
                temp /= 10;
            }
            if eq.numbers[0] * multiplier + eq.numbers[1] == eq.test_value {
                return true;
            }
        }
        
        // Check addition
        if eq.numbers[0] + eq.numbers[1] == eq.test_value {
            return true;
        }
        
        // Check multiplication
        if eq.numbers[0] * eq.numbers[1] == eq.test_value {
            return true;
        }
        
        // If none of the direct operations work for two numbers, return false
        if n_slots == 1 {
            return false;
        }
    }
    
    let total_combinations = ops.len().pow(n_slots as u32);
    
    // Pre-calculate concatenation multipliers for each number
    let mut concat_multipliers = if include_concat {
        let mut multipliers = Vec::with_capacity(eq.numbers.len());
        for &num in &eq.numbers {
            let mut multiplier = 1;
            let mut temp = num;
            while temp > 0 {
                multiplier *= 10;
                temp /= 10;
            }
            multipliers.push(multiplier);
        }
        Some(multipliers)
    } else {
        None
    };

    // Try all possible combinations of operators
    for combo in 0..total_combinations {
        let mut operators = Vec::with_capacity(n_slots);
        let mut temp = combo;
        for _ in 0..n_slots {
            operators.push(ops[temp % ops.len()]);
            temp /= ops.len();
        }
        
        let mut should_skip = false;
        let mut current_value = eq.test_value;
        
        for i in (0..n_slots).rev() {
            match operators[i] {
                '*' => {
                    if current_value % eq.numbers[i + 1] != 0 {
                        should_skip = true;
                        break;
                    }
                    current_value /= eq.numbers[i + 1];
                },
                '|' => {
                    let divisor = concat_multipliers.as_ref().unwrap()[i + 1];
                    if current_value % divisor != eq.numbers[i + 1] {
                        should_skip = true;
                        break;
                    }
                    current_value /= divisor;
                },
                '+' => {
                    current_value -= eq.numbers[i + 1];
                    if current_value < 0 {
                        should_skip = true;
                        break;
                    }
                },
                _ => panic!("Invalid operator"),
            }
        }
        
        if !should_skip && current_value == eq.numbers[0] {
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