use std::fs::read_to_string;
use std::error::Error;

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

        if !cells.iter().all(|row| row.len() == cols) {
            return Err("Inconsistent row lengths".into());
        }

        Ok(Grid { cells, rows, cols })
    }

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1), (1, 1), (1, 0), (1, -1),
        (0, -1), (-1, -1), (-1, 0), (-1, 1),
    ];

    fn count_word(&self, word: &str) -> usize {
        let word_chars: Vec<char> = word.chars().collect();
        let mut count = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                for &(dx, dy) in Self::DIRECTIONS.iter() {
                    if self.check_word_from_position(row, col, &word_chars, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn count_cross_pattern(&self) -> usize {
        let mut count = 0;

        // Pattern needs at least 3x3 space
        for row in 1..self.rows.saturating_sub(1) {
            for col in 1..self.cols.saturating_sub(1) {
                if self.check_cross_at_position(row, col, false) ||
                   self.check_cross_at_position(row, col, true) {
                    count += 1;
                }
            }
        }
        count
    }

    fn check_cross_at_position(&self, row: usize, col: usize, reversed: bool) -> bool {
        // First, check center A
        if self.cells[row][col] != 'A' {
            return false;
        }
    
        // Get the diagonal lines
        let forward_diagonal = [
            self.cells[row-1][col+1],  // top-right
            self.cells[row][col],      // center
            self.cells[row+1][col-1]   // bottom-left
        ];
    
        let back_diagonal = [
            self.cells[row-1][col-1],  // top-left
            self.cells[row][col],      // center
            self.cells[row+1][col+1]   // bottom-right
        ];
    
        // Define the patterns we're looking for
        let pattern1 = ['M', 'A', 'S'];
        let pattern2 = ['S', 'A', 'M'];
    
        // Check if either diagonal matches either pattern
        let forward_matches = forward_diagonal == pattern1 || forward_diagonal == pattern2;
        let back_matches = back_diagonal == pattern1 || back_diagonal == pattern2;
    
        // Both diagonals must match a pattern
        forward_matches && back_matches
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

fn main() -> Result<(), Box<dyn Error>> {
    let grid = Grid::from_file("data/input.txt")?;

    let word = "XMAS";
    let word_count = grid.count_word(word);
    println!("Found {} occurrences of '{}'", word_count, word);

    let cross_count = grid.count_cross_pattern();
    println!("Found {} cross patterns", cross_count);

    Ok(())
}