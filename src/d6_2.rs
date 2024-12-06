use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_position(&self, pos: (i32, i32)) -> (i32, i32) {
        let (x, y) = pos;
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn new(input: &str) -> (Self, (i32, i32), Direction) {
        let mut cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let rows = cells.len() as i32;
        let cols = cells[0].len() as i32;
        let mut start_pos = (0, 0);
        let mut start_dir = Direction::Up;

        // Find starting position and direction
        for (y, row) in cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if *cell == '^' {
                    start_pos = (x as i32, y as i32);
                    start_dir = Direction::Up;
                    *cell = '.';
                    break;
                }
            }
        }

        (Grid { cells, rows, cols }, start_pos, start_dir)
    }

    fn is_valid_position(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.cols && y >= 0 && y < self.rows
    }

    fn is_obstacle(&self, (x, y): (i32, i32)) -> bool {
        self.cells[y as usize][x as usize] == '#'
    }

    fn add_obstruction(&mut self, (x, y): (i32, i32)) {
        self.cells[y as usize][x as usize] = '#';
    }

    fn remove_obstruction(&mut self, (x, y): (i32, i32)) {
        self.cells[y as usize][x as usize] = '.';
    }
}

fn detect_loop(grid: &Grid, start_pos: (i32, i32), start_dir: Direction) -> bool {
    let mut pos = start_pos;
    let mut direction = start_dir;
    let mut state_history: HashMap<((i32, i32), Direction), usize> = HashMap::new();
    let mut steps = 0;

    loop {
        // Record current state
        let state = (pos, direction);

        // If we've seen this state before, we're in a loop
        if let Some(previous_step) = state_history.get(&state) {
            return true;
        }

        state_history.insert(state, steps);
        steps += 1;

        // Get next position
        let next_pos = direction.next_position(pos);

        // If out of bounds, not a loop
        if !grid.is_valid_position(next_pos) {
            return false;
        }

        // Update position/direction based on obstacles
        if grid.is_obstacle(next_pos) {
            direction = direction.turn_right();
        } else {
            pos = next_pos;
        }
    }
}

fn find_loop_positions(input: &str) -> usize {
    let (mut grid, start_pos, start_dir) = Grid::new(input);
    let mut loop_positions = HashSet::new();

    // Try each empty position
    for y in 0..grid.rows {
        for x in 0..grid.cols {
            let pos = (x, y);

            // Skip if not empty or if it's the start position
            if pos == start_pos || grid.is_obstacle(pos) {
                continue;
            }

            // Add obstruction and check for loop
            grid.add_obstruction(pos);
            if detect_loop(&grid, start_pos, start_dir) {
                loop_positions.insert(pos);
            }
            grid.remove_obstruction(pos);
        }
    }

    loop_positions.len()
}
#[test]
fn sample() {
    let example_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let result = find_loop_positions(example_input);
    println!("Number of possible loop-causing positions: {}", result);
    assert_eq!(result, 6, "Expected 6 positions, but got {}", result);
}
#[test]
fn main() {
    let input = include_str!("d6.in");
    let result = find_loop_positions(input);
    println!("Number of possible loop-causing positions: {}", result);
}
