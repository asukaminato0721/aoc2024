use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return None;
        }

        let pos_parts: Vec<&str> = parts[0].trim_start_matches("p=").split(',').collect();
        let vel_parts: Vec<&str> = parts[1].trim_start_matches("v=").split(',').collect();

        Some(Robot {
            position: Position {
                x: pos_parts[0].parse().ok()?,
                y: pos_parts[1].parse().ok()?,
            },
            velocity: Velocity {
                x: vel_parts[0].parse().ok()?,
                y: vel_parts[1].parse().ok()?,
            },
        })
    }

    fn update_position(&mut self, width: i32, height: i32) {
        // Update position
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(width);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(height);
    }
}

fn get_quadrant(pos: Position, width: i32, height: i32) -> Option<usize> {
    let mid_x = width / 2;
    let mid_y = height / 2;

    // Skip robots on the middle lines
    if pos.x == mid_x || pos.y == mid_y {
        return None;
    }

    // Determine quadrant (0-based indexing)
    Some(match (pos.x < mid_x, pos.y < mid_y) {
        (true, true) => 0,   // Top-left
        (false, true) => 1,  // Top-right
        (true, false) => 2,  // Bottom-left
        (false, false) => 3, // Bottom-right
    })
}

fn simulate_robots(input: &str, width: i32, height: i32, steps: i32) -> i64 {
    let mut robots: Vec<Robot> = input
        .lines()
        .filter_map(|line| Robot::from_str(line.trim()))
        .collect();

    // Simulate movement
    for _ in 0..steps {
        for robot in robots.iter_mut() {
            robot.update_position(width, height);
        }
    }

    // Count robots in each quadrant
    let mut quadrant_counts = vec![0; 4];
    for robot in robots {
        if let Some(quadrant) = get_quadrant(robot.position, width, height) {
            quadrant_counts[quadrant] += 1;
        }
    }

    // Calculate safety factor (multiply all quadrant counts)
    quadrant_counts.iter().fold(1, |acc, &count| acc * count as i64)
}
#[test]
fn main() {
    let input = include_str!("d14.in");

    let width = 101;
    let height = 103;
    let steps = 100;

    let safety_factor = simulate_robots(input, width, height, steps);
    println!("Safety factor after {} seconds: {}", steps, safety_factor);
}