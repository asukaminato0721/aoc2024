use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.perimeter
    }
}

fn find_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    // Directions for checking adjacent cells (up, right, down, left)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for i in 0..rows {
        for j in 0..cols {
            if visited.contains(&(i, j)) {
                continue;
            }

            let plant_type = grid[i][j];
            let mut region = Region {
                area: 0,
                perimeter: 0,
            };

            // BFS to find all connected plots of the same type
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            visited.insert((i, j));

            while let Some((r, c)) = queue.pop_front() {
                region.area += 1;

                // Check all four directions
                for (dr, dc) in directions.iter() {
                    let new_r = (r as i32 + dr) as usize;
                    let new_c = (c as i32 + dc) as usize;

                    // Check if the new position is within bounds
                    if new_r < rows && new_c < cols {
                        if grid[new_r][new_c] == plant_type {
                            // If it's the same plant type and not visited, add to queue
                            if !visited.contains(&(new_r, new_c)) {
                                queue.push_back((new_r, new_c));
                                visited.insert((new_r, new_c));
                            }
                        } else {
                            // If it's a different plant type, add to perimeter
                            region.perimeter += 1;
                        }
                    } else {
                        // If it's out of bounds, add to perimeter
                        region.perimeter += 1;
                    }
                }
            }

            regions.push(region);
        }
    }

    regions
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_total_price(input: &str) -> usize {
    let grid = parse_input(input);
    let regions = find_regions(&grid);
    regions.iter().map(|region| region.price()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(calculate_total_price(input), 140);
    }

    #[test]
    fn test_o_x_example() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(calculate_total_price(input), 772);
    }

    #[test]
    fn test_larger_example() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(calculate_total_price(input), 1930);
    }
}
#[test]
fn main() {
    // You can add your input here as a string
    let input = include_str!("d12.in"); // Add your puzzle input here
    let total_price = calculate_total_price(input);
    println!("Total price of fencing: {}", total_price);
}