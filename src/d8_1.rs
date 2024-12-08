use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Calculate squared distance between two points
    fn distance_squared(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

// Check if point is collinear with two antennas
fn is_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    // Use cross product to determine collinearity
    let area = (p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y);
    area == 0
}

// Check if p3 is between p1 and p2
fn is_between(p1: &Point, p2: &Point, p3: &Point) -> bool {
    if !is_collinear(p1, p2, p3) {
        return false;
    }
    
    if p1.x != p2.x {
        // Use x-coordinates for comparison
        (p1.x <= p3.x && p3.x <= p2.x) || (p2.x <= p3.x && p3.x <= p1.x)
    } else {
        // Use y-coordinates for vertical lines
        (p1.y <= p3.y && p3.y <= p2.y) || (p2.y <= p3.y && p3.y <= p1.y)
    }
}

fn find_antinodes(a1: &Point, a2: &Point, bounds: (i32, i32)) -> Vec<Point> {
    let mut antinodes = Vec::new();
    
    // Ensure we're not checking the same point
    if a1 == a2 {
        return antinodes;
    }

    // For each point in bounds
    for x in 0..=bounds.0 {
        for y in 0..=bounds.1 {
            let p = Point::new(x, y);
            
            // Skip if point is an antenna location
            if p == *a1 || p == *a2 {
                continue;
            }

            // Check if point is collinear with antennas
            if !is_collinear(a1, a2, &p) {
                continue;
            }

            // Calculate distances
            let d1 = p.distance_squared(a1);
            let d2 = p.distance_squared(a2);

            // Check for exact 1:2 ratio and that point isn't between antennas
            if (d1 == 4 * d2 || d2 == 4 * d1) && !is_between(a1, a2, &p) {
                antinodes.push(p);
            }
        }
    }

    antinodes
}

fn parse_map(input: &str) -> (HashMap<char, Vec<Point>>, (i32, i32)) {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Point::new(x as i32, y as i32));
            }
            max_x = max_x.max(x as i32);
        }
        max_y = max_y.max(y as i32);
    }

    (antennas, (max_x, max_y))
}

fn count_antinodes(input: &str) -> usize {
    let (antennas, bounds) = parse_map(input);
    let mut all_antinodes = HashSet::new();

    // Process each frequency
    for (freq, positions) in antennas.iter() {
        // Need at least 2 antennas of same frequency
        if positions.len() < 2 {
            continue;
        }

        // Check each pair of antennas
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let nodes = find_antinodes(&positions[i], &positions[j], bounds);
                all_antinodes.extend(nodes);
            }
        }
    }

    all_antinodes.len()
}
#[test]
fn main() {
    let input = include_str!("d8.in");

    let result = count_antinodes(input);
    println!("Number of unique antinode locations: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(count_antinodes(input), 14);
    }

    #[test]
    fn test_simple() {
        let input = "..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........";
        assert_eq!(count_antinodes(input), 2);
    }
}