use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn move_forward(&self, pos: Position) -> Position {
        match self {
            Direction::North => Position { x: pos.x, y: pos.y - 1 },
            Direction::East => Position { x: pos.x + 1, y: pos.y },
            Direction::South => Position { x: pos.x, y: pos.y + 1 },
            Direction::West => Position { x: pos.x - 1, y: pos.y },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: Position,
    direction: Direction,
    path: Vec<Position>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    grid: Vec<Vec<char>>,
    start: Position,
    end: Position,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        
        let mut start = Position { x: 0, y: 0 };
        let mut end = Position { x: 0, y: 0 };
        
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                match cell {
                    'S' => start = Position { x: x as i32, y: y as i32 },
                    'E' => end = Position { x: x as i32, y: y as i32 },
                    _ => {}
                }
            }
        }
        
        Maze { grid, start, end }
    }

    fn is_wall(&self, pos: Position) -> bool {
        if pos.x < 0 || pos.y < 0 {
            return true;
        }
        
        let y = pos.y as usize;
        let x = pos.x as usize;
        
        if y >= self.grid.len() || x >= self.grid[0].len() {
            return true;
        }
        
        self.grid[y][x] == '#'
    }

    fn find_optimal_paths(&self) -> (Option<i32>, HashSet<Position>) {
        let mut heap = BinaryHeap::new();
        let mut seen = HashMap::new();
        let mut optimal_paths = Vec::new();
        let mut min_cost = None;
        
        // Start facing East
        let initial = State {
            cost: 0,
            position: self.start,
            direction: Direction::East,
            path: vec![self.start],
        };
        
        heap.push(initial);
        seen.insert((self.start, Direction::East), 0);
        
        while let Some(State { cost, position, direction, path }) = heap.pop() {
            if position == self.end {
                match min_cost {
                    None => {
                        min_cost = Some(cost);
                        optimal_paths.push(path);
                    }
                    Some(mc) if cost == mc => {
                        optimal_paths.push(path);
                    }
                    Some(mc) if cost > mc => continue,
                    _ => {}
                }
                continue;
            }
            
            if let Some(mc) = min_cost {
                if cost > mc {
                    continue;
                }
            }
            
            // Try moving forward
            let next_pos = direction.move_forward(position);
            if !self.is_wall(next_pos) {
                let next_cost = cost + 1;
                if let Some(&prev_cost) = seen.get(&(next_pos, direction)) {
                    if next_cost <= prev_cost {
                        let mut next_path = path.clone();
                        next_path.push(next_pos);
                        seen.insert((next_pos, direction), next_cost);
                        heap.push(State {
                            cost: next_cost,
                            position: next_pos,
                            direction,
                            path: next_path,
                        });
                    }
                } else {
                    let mut next_path = path.clone();
                    next_path.push(next_pos);
                    seen.insert((next_pos, direction), next_cost);
                    heap.push(State {
                        cost: next_cost,
                        position: next_pos,
                        direction,
                        path: next_path,
                    });
                }
            }
            
            // Try turning left and right
            for &new_dir in &[direction.turn_left(), direction.turn_right()] {
                let next_cost = cost + 1000;
                if let Some(&prev_cost) = seen.get(&(position, new_dir)) {
                    if next_cost <= prev_cost {
                        seen.insert((position, new_dir), next_cost);
                        heap.push(State {
                            cost: next_cost,
                            position,
                            direction: new_dir,
                            path: path.clone(),
                        });
                    }
                } else {
                    seen.insert((position, new_dir), next_cost);
                    heap.push(State {
                        cost: next_cost,
                        position,
                        direction: new_dir,
                        path: path.clone(),
                    });
                }
            }
        }
        
        // Collect all positions that appear in any optimal path
        let optimal_tiles: HashSet<_> = optimal_paths.into_iter()
            .flat_map(|path| path.into_iter())
            .collect();
        
        (min_cost, optimal_tiles)
    }

    fn count_optimal_tiles(&self) -> Option<usize> {
        let (_, optimal_tiles) = self.find_optimal_paths();
        Some(optimal_tiles.len())
    }

    #[cfg(test)]
    fn print_optimal_path(&self) -> Option<String> {
        let (_, optimal_tiles) = self.find_optimal_paths();
        
        let mut result = String::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '#' {
                    result.push('#');
                } else if optimal_tiles.contains(&Position { x: x as i32, y: y as i32 }) {
                    result.push('O');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        
        Some(result)
    }
}
#[test]
fn main() {
    let input = include_str!("d16.in");

    let maze = Maze::parse(input);
    if let Some(count) = maze.count_optimal_tiles() {
        println!("Number of tiles in optimal paths: {}", count);
        if let Some(visualization) = maze.print_optimal_path() {
            println!("\nVisualization of optimal paths:");
            println!("{}", visualization);
        }
    } else {
        println!("No solution found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let maze = Maze::parse(input);
        assert_eq!(maze.count_optimal_tiles(), Some(45));
    }

    #[test]
    fn test_example2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let maze = Maze::parse(input);
        assert_eq!(maze.count_optimal_tiles(), Some(64));
    }
}