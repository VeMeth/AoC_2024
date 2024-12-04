use std::fmt;
use std::fs::read_to_string;
use std::error::Error;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = read_to_string(path)?;
        let cells: Vec<Vec<char>> = content
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let rows = cells.len();
        let cols = if rows > 0 { cells[0].len() } else { 0 };

        Ok(Grid { cells, rows, cols })
    }

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),   // right
        (1, 1),   // down-right
        (1, 0),   // down
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, -1), // up-left
        (-1, 0),  // up
        (-1, 1),  // up-right
    ];

    fn find_word(&self, word: &str) -> Vec<(usize, usize, &'static str)> {
        let mut results = Vec::new();
        let word_chars: Vec<char> = word.chars().collect();

        for row in 0..self.rows {
            for col in 0..self.cols {
                for (direction_idx, &(dx, dy)) in Self::DIRECTIONS.iter().enumerate() {
                    if self.check_word_from_position(row, col, &word_chars, dx, dy) {
                        let direction_name = match direction_idx {
                            0 => "right",
                            1 => "down-right",
                            2 => "down",
                            3 => "down-left",
                            4 => "left",
                            5 => "up-left",
                            6 => "up",
                            7 => "up-right",
                            _ => unreachable!(),
                        };
                        results.push((row, col, direction_name));
                    }
                }
            }
        }
        results
    }

    fn check_word_from_position(&self, row: usize, col: usize, word: &[char], dx: i32, dy: i32) -> bool {
        let word_len = word.len();
        
        let end_row = row as i32 + dx * (word_len as i32 - 1);
        let end_col = col as i32 + dy * (word_len as i32 - 1);
        
        if end_row < 0 || end_row >= self.rows as i32 || end_col < 0 || end_col >= self.cols as i32 {
            return false;
        }

        for i in 0..word_len {
            let curr_row = (row as i32 + dx * i as i32) as usize;
            let curr_col = (col as i32 + dy * i as i32) as usize;
            
            if self.cells[curr_row][curr_col] != word[i] {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            for &cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = Grid::from_file("data/input.txt")?;

    println!("Grid:");
    println!("{}", grid);

    let word = "XMAS";
    let results = grid.find_word(word);
    let count = results.len();
    
    if results.is_empty() {
        println!("\nWord '{}' not found in the grid.", word);
    } else {
        println!("\nFound {} occurrences of '{}':", count, word);
        for (row, col, direction) in results {
            println!("({}, {}) going {}", row, col, direction);
        }
        println!("\nTotal combinations found: {}", count);
    }

    Ok(())
}