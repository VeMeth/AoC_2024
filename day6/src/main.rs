use std::fs::read_to_string;
use std::error::Error;

struct Maze {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    steps: usize,  // New field to track steps taken
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

impl Maze {
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = read_to_string(path)?;
        let grid: Vec<Vec<char>> = content
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };

        Ok(Maze { 
            grid, 
            rows, 
            cols,
            steps: 0  // Initialize step counter
        })
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for (row, row_chars) in self.grid.iter().enumerate() {
            for (col, &ch) in row_chars.iter().enumerate() {
                if ch == '^' {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn is_valid_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32
    }

    fn solve(&mut self) -> Result<usize, Box<dyn Error>> {  // Changed return type to include step count
        let start = self.find_start().ok_or("No start position (^) found")?;
        let mut current_pos = (start.0 as i32, start.1 as i32);
        let mut direction = Direction::Up;

        // Mark the starting position
        self.grid[start.0][start.1] = 'X';
        self.steps = 1;  // Count the starting position as first step

        loop {
            let (dx, dy) = direction.get_delta();
            let next_pos = (current_pos.0 + dx, current_pos.1 + dy);

            // Check if we would leave the grid
            if !self.is_valid_position(next_pos.0, next_pos.1) {
                break;
            }

            // Check if we hit a wall
            if self.grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                // Rotate right and continue
                direction = direction.rotate_right();
                continue;
            }

            // Move to the next position and mark it
            if self.grid[next_pos.0 as usize][next_pos.1 as usize] != 'X' {
                self.steps += 1;  // Only increment the counter if the current position isn't already an X
            }
            current_pos = next_pos;
            self.grid[current_pos.0 as usize][current_pos.1 as usize] = 'X';
        }

        Ok(self.steps)  // Return the total number of steps
    }

    fn display(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut maze = Maze::from_file("data/input.txt")?;  // Updated file path
    let steps = maze.solve()?;
    //println!("Solved maze:\n{}", maze.display());
    println!("\nTotal steps taken: {}", steps);  // Display the step count
    Ok(())
}