use std::collections::HashSet;
use std::fs::read_to_string;

fn can_make_pattern(
    target: &str,
    patterns: &HashSet<String>,
    memo: &mut HashSet<String>,
) -> bool {
    // Base case: empty string is always possible
    if target.is_empty() {
        return true;
    }

    // Return memoized result if available
    if memo.contains(target) {
        return false;
    }

    // Try each pattern as a prefix
    for pattern in patterns {
        if target.starts_with(pattern) {
            // Recursively try to make the remaining string
            let remaining = &target[pattern.len()..];
            if can_make_pattern(remaining, patterns, memo) {
                return true;
            }
        }
    }

    // If we get here, this target is impossible
    memo.insert(target.to_string());
    false
}
#[test]
fn main() {
    // Read input file
    let input = read_to_string("src/d19.in").expect("Failed to read input file");
    let mut lines = input.lines();

    // Parse patterns from first line
    let patterns: HashSet<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    // Skip empty line
    lines.next();

    // Process each design
    let mut possible_count = 0;
    let mut memo = HashSet::new();

    for design in lines {
        if can_make_pattern(design, &patterns, &mut memo) {
            possible_count += 1;
        }
    }

    println!("Number of possible designs: {}", possible_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        let patterns: HashSet<String> = vec![
            "r", "wr", "b", "g", "bwu", "rb", "gb", "br",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let test_cases = vec![
            ("brwrr", true),   // can be made with br + wr + r
            ("bggr", true),    // can be made with b + g + g + r
            ("gbbr", true),    // can be made with gb + br
            ("rrbgbr", true),  // can be made with r + rb + g + br
            ("ubwu", false),   // impossible
            ("bwurrg", true),  // can be made with bwu + r + r + g
            ("brgr", true),    // can be made with br + g + r
            ("bbrgwb", false), // impossible
        ];

        for (design, expected) in test_cases {
            let mut memo = HashSet::new();
            assert_eq!(
                can_make_pattern(design, &patterns, &mut memo),
                expected,
                "Failed for design: {}",
                design
            );
        }
    }
}