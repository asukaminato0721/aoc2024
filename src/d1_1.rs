use std::io::{self, BufRead};
#[test]
fn main() -> io::Result<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Parse input
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() >= 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    // Sort both lists independently
    left_list.sort_unstable();
    right_list.sort_unstable();

    // Calculate total distance
    let total_distance: i64 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (*a as i64 - *b as i64).abs())
        .sum();

    println!("Total distance: {}", total_distance);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];

        left.sort_unstable();
        right.sort_unstable();

        let total: i64 = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| (*a as i64 - *b as i64).abs())
            .sum();

        assert_eq!(total, 11);
    }
}
