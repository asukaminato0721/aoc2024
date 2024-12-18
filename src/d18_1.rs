use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (i32, i32),
}

// Custom ordering for our priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.trim().split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn find_shortest_path(corrupted: &HashSet<(i32, i32)>, grid_size: i32) -> Option<usize> {
    let start = (0, 0);
    let goal = (grid_size - 1, grid_size - 1);
    
    let mut distances = HashSet::new();
    let mut heap = BinaryHeap::new();

    // Start from the beginning with cost 0
    heap.push(State { cost: 0, position: start });
    distances.insert((0, start));

    // Possible moves: up, right, down, left
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    while let Some(State { cost, position }) = heap.pop() {
        // If we've reached the goal, return the cost
        if position == goal {
            return Some(cost);
        }

        // Look at all possible moves
        for &(dx, dy) in &directions {
            let new_pos = (position.0 + dx, position.1 + dy);
            
            // Check if the new position is valid
            if new_pos.0 >= 0 && new_pos.0 < grid_size &&
               new_pos.1 >= 0 && new_pos.1 < grid_size &&
               !corrupted.contains(&new_pos) {
                
                let new_state = State {
                    cost: cost + 1,
                    position: new_pos,
                };

                // If we haven't seen this position before or found a better path
                if distances.insert((new_state.cost, new_state.position)) {
                    heap.push(new_state);
                }
            }
        }
    }

    None // No path found
}

#[test]
fn main() {
    let input = include_str!("d18.in");

    let byte_positions = parse_input(input);
    
    // For the first part, we only need to consider the first 1024 bytes
    let corrupted: HashSet<(i32, i32)> = byte_positions.iter()
        .take(1024)
        .copied()
        .collect();

    // Use 71 for the actual puzzle, 7 for the example
    let grid_size = 71;
    
    if let Some(steps) = find_shortest_path(&corrupted, grid_size) {
        println!("Minimum steps needed: {}", steps);
    } else {
        println!("No path found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1";
        
        let byte_positions = parse_input(input);
        let corrupted: HashSet<(i32, i32)> = byte_positions.iter().copied().collect();
        
        assert_eq!(find_shortest_path(&corrupted, 7), Some(22));
    }
}