use std::fs::read_to_string;

#[derive(Debug)]
struct Multiplication {
    num1: i32,
    num2: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("data/input.txt")?;
    let multiplications = parse_multiplications(&content);
    
    //println!("Found {} valid multiplications", multiplications.len());

    let mut sum = 0;
    for mult in multiplications.iter() {
        //println!("{}x{}", mult.num1, mult.num2);
        sum = sum + mult.num1 * mult.num2;
    }
    
    println!("Total Sum of Multiplications: {}", sum);
    Ok(())
}

fn parse_multiplications(input: &str) -> Vec<Multiplication> {
    let mut result = Vec::new();
    
    for part in input.split("mul(").skip(1) {
        if let Some(end) = part.find(')') {
            let numbers: Vec<&str> = part[..end].split(',').collect();
            
            if numbers.len() == 2 {
                if let (Ok(num1), Ok(num2)) = (
                    numbers[0].trim().parse::<i32>(),
                    numbers[1].trim().parse::<i32>()
                ) {
                    result.push(Multiplication { num1, num2 });
                }
            }
        }
    }
    
    result
}