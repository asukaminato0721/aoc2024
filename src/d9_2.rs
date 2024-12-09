#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
    start_pos: usize,
}

fn create_initial_state(input: &str) -> (Vec<Option<usize>>, Vec<File>) {
    let mut blocks = Vec::new();
    let mut files = Vec::new();
    let mut current_pos = 0;
    let mut file_id = 0;

    for (i, c) in input.chars().enumerate() {
        if !c.is_numeric() {
            continue;
        }
        let length = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // File block
            files.push(File {
                id: file_id,
                size: length,
                start_pos: current_pos,
            });
            for _ in 0..length {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // Free space
            for _ in 0..length {
                blocks.push(None);
            }
        }
        current_pos += length;
    }
    (blocks, files)
}

fn find_leftmost_fit(blocks: &[Option<usize>], file_size: usize) -> Option<usize> {
    let mut current_free = 0;
    let mut start_pos = None;

    for (i, block) in blocks.iter().enumerate() {
        if block.is_none() {
            if start_pos.is_none() {
                start_pos = Some(i);
            }
            current_free += 1;
            if current_free >= file_size {
                return start_pos;
            }
        } else {
            current_free = 0;
            start_pos = None;
        }
    }
    None
}

fn move_file(blocks: &mut Vec<Option<usize>>, file: &File, new_pos: usize) {
    // Clear old position
    for i in file.start_pos..file.start_pos + file.size {
        blocks[i] = None;
    }
    // Set new position
    for i in new_pos..new_pos + file.size {
        blocks[i] = Some(file.id);
    }
}

fn compact_disk(mut blocks: Vec<Option<usize>>, mut files: Vec<File>) -> Vec<Option<usize>> {
    // Sort files by ID in descending order
    files.sort_by_key(|f| std::cmp::Reverse(f.id));

    for file in files {
        // Find the leftmost span of free space that can fit this file
        if let Some(new_pos) = find_leftmost_fit(&blocks, file.size) {
            // Only move if the new position is to the left of the current position
            if new_pos < file.start_pos {
                move_file(&mut blocks, &file, new_pos);
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
    let (blocks, files) = create_initial_state(input);
    let compacted = compact_disk(blocks, files);
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
        assert_eq!(solve(input), 2858);
    }
}
