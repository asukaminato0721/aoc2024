fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn generate_next_secret(mut secret: u64) -> u64 {
    // Step 1: Multiply by 64, mix, and prune
    let result = secret * 64;
    secret = mix(secret, result);
    secret = prune(secret);

    // Step 2: Divide by 32, mix, and prune
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Step 3: Multiply by 2048, mix, and prune
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    secret
}

fn generate_nth_secret(initial: u64, n: usize) -> u64 {
    let mut secret = initial;
    for _ in 0..n {
        secret = generate_next_secret(secret);
    }
    secret
}
#[test]
fn main() {
    // Example input
    let inputs = include_str!("d22.in")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Generate 2000th secret for each input and sum them
    let sum: u64 = inputs
        .iter()
        .map(|&initial| generate_nth_secret(initial, 2000))
        .sum();

    println!("Sum of 2000th secrets: {}", sum);

    // Print individual 2000th secrets for verification
    for &initial in &inputs {
        let secret_2000 = generate_nth_secret(initial, 2000);
        //   println!("{}: {}", initial, secret_2000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_sequence() {
        let mut secret = 123;
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for &expected_value in &expected {
            secret = generate_next_secret(secret);
            assert_eq!(secret, expected_value);
        }
    }

    #[test]
    fn test_example_2000th() {
        assert_eq!(generate_nth_secret(1, 2000), 8685429);
        assert_eq!(generate_nth_secret(10, 2000), 4700978);
        assert_eq!(generate_nth_secret(100, 2000), 15273692);
        assert_eq!(generate_nth_secret(2024, 2000), 8667524);
    }
}
