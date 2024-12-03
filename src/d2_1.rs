use std::io::{self, BufRead};

fn is_safe(levels: &[i32]) -> bool {
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

        if is_safe(&levels) {
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
        assert!(is_safe(&[7, 6, 4, 2, 1])); // Safe: decreasing by 1 or 2
        assert!(!is_safe(&[1, 2, 7, 8, 9])); // Unsafe: 2->7 increases by 5
        assert!(!is_safe(&[9, 7, 6, 2, 1])); // Unsafe: 6->2 decreases by 4
        assert!(!is_safe(&[1, 3, 2, 4, 5])); // Unsafe: mixed increase/decrease
        assert!(!is_safe(&[8, 6, 4, 4, 1])); // Unsafe: no change between 4->4
        assert!(is_safe(&[1, 3, 6, 7, 9])); // Safe: increasing by 1, 2, or 3
    }
}
