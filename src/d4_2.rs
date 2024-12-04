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

    let count = count_xmas_patterns(&grid);
    println!("Found {} X-MAS patterns", count);
}

fn count_xmas_patterns(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows < 3 {
        return 0;
    }
    let cols = grid[0].len();
    if cols < 3 {
        return 0;
    }

    let mut count = 0;

    // Check each possible center point of the X
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if grid[row][col] == 'A' {
                let top_left = get_char(grid, row - 1, col - 1);
                let top_right = get_char(grid, row - 1, col + 1);
                let bottom_left = get_char(grid, row + 1, col - 1);
                let bottom_right = get_char(grid, row + 1, col + 1);

                // For each pair of diagonals, check if they form valid MAS patterns
                if let (Some(tl), Some(tr), Some(bl), Some(br)) = (top_left, top_right, bottom_left, bottom_right) {
                    if is_valid_xmas(tl, tr, bl, br) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn get_char(grid: &[Vec<char>], row: usize, col: usize) -> Option<char> {
    grid.get(row).and_then(|r| r.get(col)).copied()
}

fn is_valid_xmas(top_left: char, top_right: char, bottom_left: char, bottom_right: char) -> bool {
    // Check all possible combinations of MAS patterns
    is_valid_diagonal(top_left, bottom_right) && is_valid_diagonal(top_right, bottom_left)
}

fn is_valid_diagonal(c1: char, c2: char) -> bool {
    // A diagonal is valid if one end is 'M' and the other is 'S'
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let input = vec![
            ".M.S......".chars().collect(),
            "..A..MSMS.".chars().collect(),
            ".M.S.MAA..".chars().collect(),
            "..A.ASMSM.".chars().collect(),
            ".M.S.M....".chars().collect(),
            "..........".chars().collect(),
            "S.S.S.S.S.".chars().collect(),
            ".A.A.A.A..".chars().collect(),
            "M.M.M.M.M.".chars().collect(),
            "..........".chars().collect(),
        ];
        
        assert_eq!(count_xmas_patterns(&input), 9);
    }

    #[test]
    fn test_simple_xmas() {
        let input = vec![
            "M.S".chars().collect(),
            ".A.".chars().collect(),
            "M.S".chars().collect(),
        ];
        
        assert_eq!(count_xmas_patterns(&input), 1);
    }

    #[test]
    fn test_valid_diagonal() {
        assert!(is_valid_diagonal('M', 'S')); // Forward MAS
        assert!(is_valid_diagonal('S', 'M')); // Backward MAS
        assert!(!is_valid_diagonal('M', 'M')); // Invalid
        assert!(!is_valid_diagonal('S', 'S')); // Invalid
    }
}