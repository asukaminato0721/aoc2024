use std::collections::{HashMap, BinaryHeap};
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: i32,
    position: Position,
    direction: Direction,
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

#[derive(Debug)]
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

    fn solve(&self) -> Option<i32> {
        let mut heap = BinaryHeap::new();
        let mut seen = HashMap::new();
        
        // Start facing East as specified
        let initial = State {
            cost: 0,
            position: self.start,
            direction: Direction::East,
        };
        
        heap.push(initial);
        seen.insert((initial.position, initial.direction), 0);
        
        while let Some(State { cost, position, direction }) = heap.pop() {
            if position == self.end {
                return Some(cost);
            }
            
            // Try moving forward (cost: 1)
            let next_pos = direction.move_forward(position);
            if !self.is_wall(next_pos) {
                let next = State {
                    cost: cost + 1,
                    position: next_pos,
                    direction,
                };
                
                if let Some(&prev_cost) = seen.get(&(next_pos, direction)) {
                    if next.cost < prev_cost {
                        heap.push(next);
                        seen.insert((next_pos, direction), next.cost);
                    }
                } else {
                    heap.push(next);
                    seen.insert((next_pos, direction), next.cost);
                }
            }
            
            // Try turning left (cost: 1000)
            let left = direction.turn_left();
            let next = State {
                cost: cost + 1000,
                position,
                direction: left,
            };
            
            if let Some(&prev_cost) = seen.get(&(position, left)) {
                if next.cost < prev_cost {
                    heap.push(next);
                    seen.insert((position, left), next.cost);
                }
            } else {
                heap.push(next);
                seen.insert((position, left), next.cost);
            }
            
            // Try turning right (cost: 1000)
            let right = direction.turn_right();
            let next = State {
                cost: cost + 1000,
                position,
                direction: right,
            };
            
            if let Some(&prev_cost) = seen.get(&(position, right)) {
                if next.cost < prev_cost {
                    heap.push(next);
                    seen.insert((position, right), next.cost);
                }
            } else {
                heap.push(next);
                seen.insert((position, right), next.cost);
            }
        }
        
        None
    }
}
#[test]
fn main() {
    let input = include_str!("d16.in");

    let maze = Maze::parse(input);
    if let Some(result) = maze.solve() {
        println!("Lowest possible score: {}", result);
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
        assert_eq!(maze.solve(), Some(7036));
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
        assert_eq!(maze.solve(), Some(11048));
    }
}