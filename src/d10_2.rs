use std::collections::{HashMap, HashSet};

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

fn count_paths_to_nine(
    grid: &[Vec<u32>],
    current: Point,
    target_height: u32,
    memo: &mut HashMap<(Point, u32), usize>,
) -> usize {
    // If we've reached height 9, we've found one valid path
    if grid[current.row as usize][current.col as usize] == 9 {
        return 1;
    }

    // Check memoization cache
    let state = (current, target_height);
    if let Some(&count) = memo.get(&state) {
        return count;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut total_paths = 0;

    // Try all possible next steps
    for next in get_neighbors(current, rows, cols) {
        let next_height = grid[next.row as usize][next.col as usize];

        // Only proceed if the next position has exactly the height we're looking for
        if next_height == target_height {
            total_paths += count_paths_to_nine(grid, next, target_height + 1, memo);
        }
    }

    // Cache the result
    memo.insert(state, total_paths);
    total_paths
}

fn calculate_trailhead_rating(grid: &[Vec<u32>], start: Point) -> usize {
    let mut memo = HashMap::new();
    count_paths_to_nine(grid, start, 1, &mut memo)
}

fn solve(input: &str) -> usize {
    let grid = parse_input(input);
    let trailheads = find_trailheads(&grid);

    trailheads
        .iter()
        .map(|&start| calculate_trailhead_rating(&grid, start))
        .sum()
}
#[test]
fn main() {
    let input = include_str!("d10.in");

    let result = solve(input);
    println!("Sum of trailhead ratings: {}", result);
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

        assert_eq!(solve(input), 81);
    }

    #[test]
    fn test_single_trailhead_three_paths() {
        let input = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_single_trailhead_thirteen_paths() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

        assert_eq!(solve(input), 13);
    }

    #[test]
    fn test_single_trailhead_227_paths() {
        let input = "012345
123456
234567
345678
4.6789
56789.";

        assert_eq!(solve(input), 227);
    }
}
