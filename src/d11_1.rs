#[test]
fn main() {
    // Example input (can be modified)
    let initial_stones = include_str!("d11.in")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut stones = initial_stones;
    let blinks = 25;

    // println!("Initial arrangement:");
    // println!("{:?}", stones);

    for i in 1..=blinks {
        stones = transform_stones(&stones);
        // println!("\nAfter {} blinks:", i);
        // println!("{:?}", stones);
    }

    println!("\nFinal number of stones: {}", stones.len());
}

fn transform_stones(stones: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();

    for &stone in stones {
        let transformed = transform_single_stone(stone);
        result.extend(transformed);
    }

    result
}

fn transform_single_stone(stone: u64) -> Vec<u64> {
    // Rule 1: If stone is 0, replace with 1
    if stone == 0 {
        return vec![1];
    }

    // Rule 2: If stone has even number of digits, split in half
    let digit_count = count_digits(stone);
    if digit_count % 2 == 0 {
        let power = 10_u64.pow((digit_count / 2) as u32);
        let left = stone / power;
        let right = stone % power;
        return vec![left, right];
    }

    // Rule 3: Multiply by 2024
    vec![stone * 2024]
}

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    (n as f64).log10().floor() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_sequence() {
        let stones = vec![0, 1, 10, 99, 999];
        let transformed = transform_stones(&stones);
        assert_eq!(transformed, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_transform_zero() {
        assert_eq!(transform_single_stone(0), vec![1]);
    }

    #[test]
    fn test_transform_even_digits() {
        assert_eq!(transform_single_stone(1234), vec![12, 34]);
        assert_eq!(transform_single_stone(10), vec![1, 0]);
    }

    #[test]
    fn test_transform_multiply() {
        assert_eq!(transform_single_stone(1), vec![2024]);
        assert_eq!(transform_single_stone(999), vec![2021976]);
    }
}
