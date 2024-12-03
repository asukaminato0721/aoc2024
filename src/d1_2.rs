use std::collections::HashMap;
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

        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() >= 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    // Count occurrences in right list
    let mut right_counts: HashMap<i64, i64> = HashMap::new();
    for &num in &right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score
    let similarity_score: i64 = left_list
        .iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        let mut right_counts: HashMap<i64, i64> = HashMap::new();
        for &num in &right {
            *right_counts.entry(num).or_insert(0) += 1;
        }

        let score: i64 = left
            .iter()
            .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
            .sum();

        assert_eq!(score, 31);
    }
}
