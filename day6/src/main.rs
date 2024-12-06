use std::fs::read_to_string;
use std::error::Error;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct WallCollision {
    position: (i32, i32),
    direction: Direction,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Maze {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    steps: usize,
    wall_collisions: HashSet<WallCollision>,
    has_loop: bool,
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
            steps: 0,
            wall_collisions: HashSet::new(),
            has_loop: false,
        })
    }

    fn try_all_wall_positions(&mut self) -> (Vec<(usize, usize, usize, bool)>, usize) {
        let mut results = Vec::new();
        let original_grid = self.grid.clone();
        let mut total_loops = 0;
        
        for row in 0..self.rows {
            for col in 0..self.cols {
                if original_grid[row][col] != '#' && original_grid[row][col] != '^' {
                    self.grid[row][col] = '#';
                    if let Ok((steps, has_loop)) = self.solve() {
                        if has_loop {
                            total_loops += 1;
                            self.grid = original_grid.clone();
                            continue;
                        }
                        results.push((row, col, steps, has_loop));
                    }
                    self.grid = original_grid.clone();
                }
            }
        }
        
        (results, total_loops)
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

    fn solve(&mut self) -> Result<(usize, bool), Box<dyn Error>> {
        let start = self.find_start().ok_or("No start position (^) found")?;
        let mut current_pos = (start.0 as i32, start.1 as i32);
        let mut direction = Direction::Up;

        self.grid[start.0][start.1] = 'X';
        self.steps = 1;
        self.wall_collisions.clear();
        self.has_loop = false;

        loop {
            let (dx, dy) = direction.get_delta();
            let next_pos = (current_pos.0 + dx, current_pos.1 + dy);

            if !self.is_valid_position(next_pos.0, next_pos.1) {
                break;
            }

            if self.grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                let collision = WallCollision {
                    position: next_pos,
                    direction,
                };
                
                if !self.wall_collisions.insert(collision) {
                    self.has_loop = true;
                    break;
                }
                
                direction = direction.rotate_right();
                continue;
            }

            if self.grid[next_pos.0 as usize][next_pos.1 as usize] != 'X' {
                self.steps += 1;
            }
            current_pos = next_pos;
            self.grid[current_pos.0 as usize][current_pos.1 as usize] = 'X';
        }

        Ok((self.steps, self.has_loop))
    }

    
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut original_maze = Maze::from_file("data/input.txt")?;
    let (original_steps, _) = original_maze.solve()?;
    println!("Original maze steps: {}", original_steps);

    let mut maze = Maze::from_file("data/input.txt")?;
    let (_results, total_loops) = maze.try_all_wall_positions();
    
    println!("Total configurations with loops: {}", total_loops);
    
    Ok(())
}