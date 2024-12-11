use std::collections::HashMap;

fn solve(initial_stones: Vec<u64>, blinks: usize) -> usize {
    let mut counts: HashMap<u64, usize> = HashMap::new();
    for stone in initial_stones {
        *counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_counts: HashMap<u64, usize> = HashMap::new();
        for (stone, count) in counts {
            if stone == 0 {
                *new_counts.entry(1).or_insert(0) += count;
            } else {
                let s = stone.to_string();
                if s.len() % 2 == 0 {
                    let mid = s.len() / 2;
                    let left = s[..mid].parse::<u64>().unwrap_or(0);
                    let right = s[mid..].parse::<u64>().unwrap_or(0);
                    *new_counts.entry(left).or_insert(0) += count;
                    *new_counts.entry(right).or_insert(0) += count;
                } else {
                    *new_counts.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }
        counts = new_counts;
    }

    counts.values().sum()
}
#[test]
fn main() {
    let initial_stones = vec![125, 17];
    println!("Part 1: {}", solve(initial_stones.clone(), 25));
    println!("Part 2: {}", solve(initial_stones, 75));

    // For the actual puzzle input:
    let input: Vec<u64> = include_str!("d11.in")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Puzzle Part 1: {}", solve(input.clone(), 25));
    println!("Puzzle Part 2: {}", solve(input, 75));
}
