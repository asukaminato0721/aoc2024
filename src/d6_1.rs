use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
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

struct Grid {
    cells: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn new(input: &str) -> (Self, (i32, i32), Direction) {
        let mut cells: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        
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

        (
            Grid { cells, rows, cols },
            start_pos,
            start_dir,
        )
    }

    fn is_valid_position(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        x >= 0 && x < self.cols && y >= 0 && y < self.rows
    }

    fn is_obstacle(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        self.cells[y as usize][x as usize] == '#'
    }
}

fn simulate_guard_path(input: &str) -> usize {
    let (grid, mut pos, mut direction) = Grid::new(input);
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        let next_pos = direction.next_position(pos);

        if !grid.is_valid_position(next_pos) {
            break;
        }

        if grid.is_obstacle(next_pos) {
            direction = direction.turn_right();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }

    visited.len()
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

    let result = simulate_guard_path(example_input);
    println!("Number of distinct positions visited: {}", result);
    assert_eq!(result, 41, "Expected 41 positions, but got {}", result);
}

#[test]
fn main(){
    let input = include_str!("d6.in");
    let result = simulate_guard_path(input);
    println!("Number of distinct positions visited: {}", result);
}