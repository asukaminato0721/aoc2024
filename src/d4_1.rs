use std::io::{self, BufRead};
#[test]
fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let count = count_xmas_occurrences(&grid);
    println!("Found {} occurrences of XMAS", count);
}

fn count_xmas_occurrences(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    
    // Define all possible directions: right, down-right, down, down-left, 
    // left, up-left, up, up-right
    let directions = [
        (0, 1),   // right
        (1, 1),   // down-right
        (1, 0),   // down
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, -1), // up-left
        (-1, 0),  // up
        (-1, 1),  // up-right
    ];
    
    let target = "XMAS";
    let mut count = 0;
    
    // Check each starting position
    for row in 0..rows {
        for col in 0..cols {
            // Try each direction from this position
            for &(dy, dx) in &directions {
                if check_word(grid, row, col, dy, dx, target) {
                    count += 1;
                }
            }
        }
    }
    
    count
}

fn check_word(grid: &[Vec<char>], start_row: usize, start_col: usize, dy: i32, dx: i32, target: &str) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let target_chars: Vec<char> = target.chars().collect();
    
    for (i, &target_char) in target_chars.iter().enumerate() {
        let curr_row = start_row as i32 + dy * i as i32;
        let curr_col = start_col as i32 + dx * i as i32;
        
        // Check bounds
        if curr_row < 0 || curr_row >= rows || curr_col < 0 || curr_col >= cols {
            return false;
        }
        
        // Check character match
        if grid[curr_row as usize][curr_col as usize] != target_char {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let input = vec![
            "MMMSXXMASM".chars().collect(),
            "MSAMXMSMSA".chars().collect(),
            "AMXSXMAAMM".chars().collect(),
            "MSAMASMSMX".chars().collect(),
            "XMASAMXAMM".chars().collect(),
            "XXAMMXXAMA".chars().collect(),
            "SMSMSASXSS".chars().collect(),
            "SAXAMASAAA".chars().collect(),
            "MAMMMXMMMM".chars().collect(),
            "MXMXAXMASX".chars().collect(),
        ];
        
        assert_eq!(count_xmas_occurrences(&input), 18);
    }
}