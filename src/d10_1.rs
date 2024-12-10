use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn find_trailheads(grid: &[Vec<u32>]) -> Vec<Point> {
    let mut trailheads = Vec::new();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row as usize][col as usize] == 0 {
                trailheads.push(Point { row, col });
            }
        }
    }
    trailheads
}

fn get_neighbors(point: Point, rows: i32, cols: i32) -> Vec<Point> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut neighbors = Vec::new();

    for (dr, dc) in directions.iter() {
        let new_row = point.row + dr;
        let new_col = point.col + dc;

        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            neighbors.push(Point {
                row: new_row,
                col: new_col,
            });
        }
    }
    neighbors
}

fn calculate_trailhead_score(grid: &[Vec<u32>], start: Point) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut reachable_nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Store point and current height
    queue.push_back((start, 0));
    visited.insert((start, 0));

    while let Some((current, height)) = queue.pop_front() {
        if grid[current.row as usize][current.col as usize] == 9 {
            reachable_nines.insert(current);
        }

        for neighbor in get_neighbors(current, rows, cols) {
            let neighbor_height = grid[neighbor.row as usize][neighbor.col as usize];
            
            // Only proceed if it's exactly one higher than current height
            if neighbor_height == height + 1 {
                let state = (neighbor, neighbor_height);
                if !visited.contains(&state) {
                    visited.insert(state);
                    queue.push_back(state);
                }
            }
        }
    }

    reachable_nines.len()
}

fn solve(input: &str) -> usize {
    let grid = parse_input(input);
    let trailheads = find_trailheads(&grid);
    
    trailheads
        .iter()
        .map(|&start| calculate_trailhead_score(&grid, start))
        .sum()
}
#[test]
fn main() {
    let input = include_str!("d10.in");

    let result = solve(input);
    println!("Sum of trailhead scores: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        
        assert_eq!(solve(input), 36);
    }

    #[test]
    fn test_simple_example() {
        let input = "0123
1234
8765
9876";
        
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn test_multiple_trailheads() {
        let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";

        assert_eq!(solve(input), 3);
    }
}