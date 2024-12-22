use std::collections::HashMap;

fn calculate_next_secret(secret: u64) -> u64 {
    let mut current_secret = secret;

    // Step 1
    let step1_result = current_secret * 64;
    current_secret ^= step1_result;
    current_secret %= 16777216;

    // Step 2
    let step2_result = current_secret / 32;
    current_secret ^= step2_result;
    current_secret %= 16777216;

    // Step 3
    let step3_result = current_secret * 2048;
    current_secret ^= step3_result;
    current_secret %= 16777216;

    current_secret
}

fn generate_secrets(initial_secret: u64, num_secrets: usize) -> Vec<u64> {
    let mut secrets = Vec::with_capacity(num_secrets + 1);
    secrets.push(initial_secret);
    let mut current_secret = initial_secret;
    for _ in 0..num_secrets {
        current_secret = calculate_next_secret(current_secret);
        secrets.push(current_secret);
    }
    secrets
}

fn calculate_prices(secrets: &[u64]) -> Vec<u8> {
    secrets.iter().map(|&secret| (secret % 10) as u8).collect()
}

fn calculate_price_changes(prices: &[u8]) -> Vec<i8> {
    prices
        .windows(2)
        .map(|window| (window[1] as i8) - (window[0] as i8))
        .collect()
}

fn find_best_sequence(initial_secrets: &[u64]) -> i64 {
    let num_secrets = 2000;
    let mut best_total_bananas = 0;
    let mut best_sequence = Vec::new();

    // Generate all price sequences for all buyers
    let all_prices: Vec<Vec<u8>> = initial_secrets
        .iter()
        .map(|&secret| generate_secrets(secret, num_secrets))
        .map(|secrets| calculate_prices(&secrets))
        .collect();
    let all_changes: Vec<Vec<i8>> = all_prices
        .iter()
        .map(|prices| calculate_price_changes(prices))
        .collect();

    // Loop through all possible sequences of 4 changes and find the highest banana total
    for i in -5..=5 {
        for j in -5..=5 {
            for k in -5..=5 {
                for l in -5..=5 {
                    let sequence = vec![i, j, k, l];
                    let mut total_bananas = 0;

                    for (prices, changes) in all_prices.iter().zip(all_changes.iter()) {
                        if let Some(index) =
                            changes.windows(4).position(|window| window == sequence)
                        {
                            total_bananas += prices[index + 4] as i64;
                        }
                    }
                    if total_bananas > best_total_bananas {
                        best_total_bananas = total_bananas;
                        best_sequence = sequence;
                    }
                }
            }
        }
    }

    println!("Best sequence: {:?}", best_sequence);
    best_total_bananas
}
#[test]

fn main() {
    let initial_secrets_part1: Vec<u64> = vec![1, 10, 100, 2024];
    let initial_secrets_part2: Vec<u64> = vec![1, 2, 3, 2024];

    // Part 1
    let mut sum_of_2000th_secrets: u64 = 0;
    for &secret in &initial_secrets_part1 {
        let secrets = generate_secrets(secret, 2000);
        sum_of_2000th_secrets += secrets[2000];
    }
    println!("Part 1: Sum of 2000th secrets: {}", sum_of_2000th_secrets);

    // Part 2
    let max_bananas = find_best_sequence(&initial_secrets_part2);
    println!("Part 2: Max bananas: {}", max_bananas);

    let initial_secrets_part2_final: Vec<u64> = include_str!("d22.in")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let max_bananas_final = find_best_sequence(&initial_secrets_part2_final);
    println!("Part 2: Max bananas final: {}", max_bananas_final);
}
