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
}

// Calculate Greatest Common Divisor using Euclidean algorithm
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Check if three points are collinear using cross product
fn is_collinear(p1: &Point, p2: &Point, test_point: &Point) -> bool {
    let dx1 = test_point.x - p1.x;
    let dy1 = test_point.y - p1.y;
    let dx2 = test_point.x - p2.x;
    let dy2 = test_point.y - p2.y;
    
    // Cross product should be 0 for collinear points
    dx1 * dy2 == dx2 * dy1
}

fn find_antinodes(antennas: &[Point], bounds: (i32, i32)) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    
    // Add all antenna positions if there are multiple antennas of this frequency
    if antennas.len() > 1 {
        antinodes.extend(antennas.iter().cloned());
    }
    
    // Check all points for being collinear with any pair of antennas
    for x in 0..=bounds.0 {
        for y in 0..=bounds.1 {
            let point = Point::new(x, y);
            
            // For each pair of antennas
            for i in 0..antennas.len() {
                for j in (i + 1)..antennas.len() {
                    if is_collinear(&antennas[i], &antennas[j], &point) {
                        antinodes.insert(point);
                    }
                }
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
            if ch != '.' && ch != '#' {  // Ignore existing antinode markers
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

    // Process each frequency separately
    for (_, freq_antennas) in antennas {
        if freq_antennas.len() >= 2 {
            let freq_antinodes = find_antinodes(&freq_antennas, bounds);
            all_antinodes.extend(freq_antinodes);
        }
    }

    all_antinodes.len()
}

#[test]
fn main(){
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
        assert_eq!(count_antinodes(input), 34);
    }

    #[test]
    fn test_three_t() {
        let input = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";
        assert_eq!(count_antinodes(input), 9);
    }

    #[test]
    fn test_simple_line() {
        let input = ".....
..a..
.....
..a..
.....";
        assert_eq!(count_antinodes(input), 5);  // All points in the middle column
    }
}