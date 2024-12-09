fn create_block_array(input: &str) -> Vec<Option<usize>> {
    let mut result = Vec::new();
    let mut file_id = 0;

    for (i, c) in input.chars().enumerate() {
        if !c.is_numeric() {
            continue;
        }
        let length = c.to_digit(10).expect("fail") as usize;
        for _ in 0..length {
            result.push(if i % 2 == 0 { Some(file_id) } else { None });
        }
        if i % 2 == 0 {
            file_id += 1;
        }
    }
    result
}

fn compact_disk(mut blocks: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut changed = true;
    while changed {
        changed = false;
        if let Some(right_idx) = blocks.iter().rposition(|b| b.is_some()) {
            if let Some(left_idx) = blocks[..right_idx].iter().position(|b| b.is_none()) {
                blocks[left_idx] = blocks[right_idx];
                blocks[right_idx] = None;
                changed = true;
            }
        }
    }
    blocks
}

fn calculate_checksum(blocks: &[Option<usize>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| (pos as u64) * (id as u64)))
        .sum()
}

fn solve(input: &str) -> u64 {
    let blocks = create_block_array(input);
    let compacted = compact_disk(blocks);
    calculate_checksum(&compacted)
}
#[test]
fn main() {
    let input = include_str!("d9.in");
    let result = solve(input);
    println!("Filesystem checksum: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 1928);
    }
}
