use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
#[test]
fn main() {
    let input ="########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = moves_str.chars().filter(|&c| c != '\n').collect();

    let mut boxes: HashSet<Point> = HashSet::new();
    let mut robot_pos = Point { x: 0, y: 0 };

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                'O' => {
                    boxes.insert(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '@' => {
                    robot_pos = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                _ => {}
            }
        }
    }

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    for &move_dir in &moves {
        let (dx, dy) = match move_dir {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid move direction"),
        };

        let next_robot_pos = Point {
            x: robot_pos.x + dx,
            y: robot_pos.y + dy,
        };

        if next_robot_pos.x < 0
            || next_robot_pos.x >= width
            || next_robot_pos.y < 0
            || next_robot_pos.y >= height
            || map[next_robot_pos.y as usize][next_robot_pos.x as usize] == '#'
        {
            continue;
        }

        if boxes.contains(&next_robot_pos) {
            let next_box_pos = Point {
                x: next_robot_pos.x + dx,
                y: next_robot_pos.y + dy,
            };

            if next_box_pos.x < 0
                || next_box_pos.x >= width
                || next_box_pos.y < 0
                || next_box_pos.y >= height
                || map[next_box_pos.y as usize][next_box_pos.x as usize] == '#'
                || boxes.contains(&next_box_pos)
            {
                continue;
            }

            boxes.remove(&next_robot_pos);
            boxes.insert(next_box_pos);
        }

        robot_pos = next_robot_pos;
    }

    let mut gps_sum = 0;
    for box_pos in boxes {
        gps_sum += box_pos.y * 100 + box_pos.x;
    }

    println!("Sum of GPS coordinates: {}", gps_sum);
}