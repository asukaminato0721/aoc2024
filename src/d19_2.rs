use std::collections::HashMap;
use std::fs::read_to_string;

fn count_arrangements(
    target: &str,
    patterns: &Vec<String>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    // Base case: empty string has one way to make it
    if target.is_empty() {
        return 1;
    }

    // Return memoized result if available
    if let Some(&count) = memo.get(target) {
        return count;
    }

    let mut total = 0;
    // Try each pattern as a prefix
    for pattern in patterns {
        if target.starts_with(pattern) {
            // Add the number of ways to make the remaining string
            let remaining = &target[pattern.len()..];
            total += count_arrangements(remaining, patterns, memo);
        }
    }

    // Memoize and return result
    memo.insert(target.to_string(), total);
    total
}
#[test]
fn main() {
    // Read input file
    let input = read_to_string("src/d19.in").expect("Failed to read input file");
    let mut lines = input.lines();

    // Parse patterns from first line
    let patterns: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    // Skip empty line
    lines.next();

    // Process each design
    let mut total_arrangements = 0;
    let mut memo = HashMap::new();

    for design in lines {
        let arrangements = count_arrangements(design, &patterns, &mut memo);
        if arrangements > 0 {
            println!("{}: {} ways", design, arrangements);
            total_arrangements += arrangements;
        } else {
            println!("{}: impossible", design);
        }
    }

    println!("\nTotal number of possible arrangements: {}", total_arrangements);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        let patterns: Vec<String> = vec![
            "r", "wr", "b", "g", "bwu", "rb", "gb", "br",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let test_cases = vec![
            ("brwrr", 2),    // 2 ways
            ("bggr", 1),     // 1 way
            ("gbbr", 4),     // 4 ways
            ("rrbgbr", 6),   // 6 ways
            ("ubwu", 0),     // impossible
            ("bwurrg", 1),   // 1 way
            ("brgr", 2),     // 2 ways
            ("bbrgwb", 0),   // impossible
        ];

        for (design, expected) in test_cases {
            let mut memo = HashMap::new();
            assert_eq!(
                count_arrangements(design, &patterns, &mut memo),
                expected,
                "Failed for design: {}",
                design
            );
        }
    }
}