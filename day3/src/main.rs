use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Multiplication {
    num1: i32,
    num2: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("data/input.txt")?;
    let (multiplications, precise_multiplications) = parse_multiplications(&content);
    
    let sum: i32 = multiplications.iter().map(|m| m.num1 * m.num2).sum();
    let precise_sum: i32 = precise_multiplications.iter().map(|m| m.num1 * m.num2).sum();
    
    println!("Total Sum: {}", sum);
    println!("Precise Sum: {}", precise_sum);
    Ok(())
}

fn parse_multiplications(input: &str) -> (Vec<Multiplication>, Vec<Multiplication>) {
    let mut result = Vec::new();
    let mut precise_result = Vec::new();
    let mut do_flag = true;
    
    let controls: Vec<(usize, bool)> = input.match_indices("don't()")
        .map(|(pos, _)| (pos, false))
        .chain(input.match_indices("do()").map(|(pos, _)| (pos, true)))
        .collect();
    
    for (pos, _) in input.match_indices("mul(") {
        if let Some(end) = input[pos..].find(')') {
            let nums: Vec<&str> = input[pos+4..pos+end].split(',').collect();
            if nums.len() != 2 { continue; }
            
            do_flag = controls.iter()
                .filter(|(control_pos, _)| control_pos < &pos)
                .max_by_key(|&(pos, _)| pos)
                .map_or(true, |&(_, flag)| flag);
            
            if let (Ok(num1), Ok(num2)) = (nums[0].trim().parse(), nums[1].trim().parse()) {
                let mult = Multiplication { num1, num2 };
                result.push(mult.clone());
                if do_flag {
                    precise_result.push(mult);
                }
            }
        }
    }
    
    (result, precise_result)
}