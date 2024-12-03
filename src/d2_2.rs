use std::io::{self, BufRead};

fn is_safe_without_dampener(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    // Check first two numbers to determine if we should be increasing or decreasing
    let increasing = levels[1] > levels[0];
    let first_diff = (levels[1] - levels[0]).abs();

    // First difference must be between 1 and 3
    if first_diff < 1 || first_diff > 3 {
        return false;
    }

    // Check each adjacent pair
    for window in levels.windows(2) {
        let diff = window[1] - window[0];

        // Must maintain same direction (all increasing or all decreasing)
        if (increasing && diff <= 0) || (!increasing && diff >= 0) {
            return false;
        }

        // Difference must be between 1 and 3
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    // If it's already safe without the dampener, return true
    if is_safe_without_dampener(levels) {
        return true;
    }

    // Try removing each level one at a time and check if it becomes safe
    for i in 0..levels.len() {
        let mut modified_levels: Vec<i32> = levels.to_vec();
        modified_levels.remove(i);

        if is_safe_without_dampener(&modified_levels) {
            return true;
        }
    }

    false
}
#[test]
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut safe_count = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if is_safe_with_dampener(&levels) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        // Test cases without dampener
        assert!(is_safe_without_dampener(&[7, 6, 4, 2, 1])); // Safe normally
        assert!(!is_safe_without_dampener(&[1, 2, 7, 8, 9])); // Unsafe
        assert!(!is_safe_without_dampener(&[9, 7, 6, 2, 1])); // Unsafe
        assert!(!is_safe_without_dampener(&[1, 3, 2, 4, 5])); // Unsafe normally
        assert!(!is_safe_without_dampener(&[8, 6, 4, 4, 1])); // Unsafe normally
        assert!(is_safe_without_dampener(&[1, 3, 6, 7, 9])); // Safe normally

        // Test cases with dampener
        assert!(is_safe_with_dampener(&[7, 6, 4, 2, 1])); // Safe without removing
        assert!(!is_safe_with_dampener(&[1, 2, 7, 8, 9])); // Still unsafe
        assert!(!is_safe_with_dampener(&[9, 7, 6, 2, 1])); // Still unsafe
        assert!(is_safe_with_dampener(&[1, 3, 2, 4, 5])); // Safe by removing 3
        assert!(is_safe_with_dampener(&[8, 6, 4, 4, 1])); // Safe by removing middle 4
        assert!(is_safe_with_dampener(&[1, 3, 6, 7, 9])); // Safe without removing
    }
}
