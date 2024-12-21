
fn solve() -> i32 {
    let input = include_str!("d15.in");
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = moves_str.chars().filter(|c| !c.is_whitespace()).collect();

    let mut robot_pos:(i32,i32) = (0, 0);
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '@' {
                robot_pos = (r as i32, c as i32);
                break;
            }
        }
    }

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    for move_cmd in moves {
        let (r_rob, c_rob) = robot_pos;
        let (dr, dc) = match move_cmd {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => continue,
        };

        let new_r_rob = r_rob + dr;
        let new_c_rob = c_rob + dc;

        if new_r_rob < 0 || new_r_rob >= rows || new_c_rob < 0 || new_c_rob >= cols || map[new_r_rob as usize][new_c_rob as usize] == '#' {
            continue;
        }

        if map[new_r_rob as usize][new_c_rob as usize] == 'O' {
            let mut push_r = new_r_rob;
            let mut push_c = new_c_rob;

            let mut can_push = true;
            let mut boxes_to_move = Vec::new();
            while push_r >= 0 && push_r < rows && push_c >= 0 && push_c < cols && map[push_r as usize][push_c as usize] == 'O' {
                boxes_to_move.push((push_r, push_c));
                push_r += dr;
                push_c += dc;
            }

            if push_r < 0 || push_r >= rows || push_c < 0 || push_c >= cols || map[push_r as usize][push_c as usize] == '#' {
                can_push = false;
            }

            if can_push {
                // Move the boxes
                for i in (1..boxes_to_move.len()).rev() {
                    map[boxes_to_move[i].0 as usize + dr as usize][boxes_to_move[i].1 as usize + dc as usize] = 'O';
                }
                map[boxes_to_move[0].0 as usize + dr as usize][boxes_to_move[0].1 as usize + dc as usize] = 'O';

                // Move the robot
                map[new_r_rob as usize][new_c_rob as usize] = '@';
                map[r_rob as usize][c_rob as usize] = '.';
                robot_pos = (new_r_rob, new_c_rob);
            }
        } else if map[new_r_rob as usize][new_c_rob as usize] == '.' {
            map[new_r_rob as usize][new_c_rob as usize] = '@';
            map[r_rob as usize][c_rob as usize] = '.';
            robot_pos = (new_r_rob, new_c_rob);
        }
    }

    let mut gps_sum = 0;
    for r in 0..rows {
        for c in 0..cols {
            if map[r as usize][c as usize] == 'O' {
                gps_sum += 100 * r + c;
            }
        }
    }

    gps_sum
}
#[test]
fn main() {
    println!("{}", solve());
}

#[test]
fn test_small_example() {
    let map_str = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#";
    let moves_str = "<^^>>>vv<v>>v<<";
    dbg!(solve_with_input(map_str, moves_str));
    assert_eq!(solve_with_input(map_str, moves_str), 2028);
}

fn solve_with_input(map_str: &str, moves_str: &str) -> i32 {
    let mut map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = moves_str.chars().collect();

    let mut robot_pos = (0, 0);
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '@' {
                robot_pos = (r, c);
                break;
            }
        }
    }

    let rows = map.len();
    let cols = map[0].len();

    for move_cmd in moves {
        let (r_rob, c_rob) = robot_pos;
        let (dr, dc):(i32,i32) = match move_cmd {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => continue,
        };

        let new_r_rob = r_rob + dr as usize;
        let new_c_rob = c_rob + dc as usize;

        if new_r_rob < 0 || new_r_rob >= rows || new_c_rob < 0 || new_c_rob >= cols || map[new_r_rob][new_c_rob] == '#' {
            continue;
        }

        if map[new_r_rob][new_c_rob] == 'O' {
            let mut push_r = new_r_rob as i32;
            let mut push_c = new_c_rob as i32;

            let mut can_push = true;
            let mut boxes_to_move = Vec::new();
            while push_r >= 0 && push_r < rows as i32 && push_c >= 0 && push_c < cols as i32 && map[push_r as usize][push_c as usize] == 'O' {
                boxes_to_move.push((push_r, push_c));
                push_r += dr;
                push_c += dc;
            }

            if push_r < 0 || push_r >= rows as i32 || push_c < 0 || push_c >= cols as i32 || map[push_r as usize][push_c as usize] == '#' {
                can_push = false;
            }

            if can_push {
                // Move the boxes
                for i in (0..boxes_to_move.len()).rev() {
                    let (box_r, box_c) = boxes_to_move[i];
                    let new_box_r = box_r + dr;
                    let new_box_c = box_c + dc;
                    if new_box_r >= 0 && new_box_r < rows as i32 && new_box_c >= 0 && new_box_c < cols as i32 {
                        map[new_box_r  as usize][new_box_c  as usize] = 'O';
                        if i == 0 {
                            map[box_r  as usize][box_c  as usize] = '@';
                            map[r_rob  as usize][c_rob  as usize] = '.';
                            robot_pos = (new_r_rob, new_c_rob);
                        } else {
                            map[box_r  as usize][box_c  as usize] = '.';
                        }
                    }
                }
            }
        } else if map[new_r_rob][new_c_rob] == '.' {
            map[new_r_rob][new_c_rob] = '@';
            map[r_rob][c_rob] = '.';
            robot_pos = (new_r_rob, new_c_rob);
        }
    }

    let mut gps_sum:i32 = 0;
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 'O' {
                gps_sum += 100 * r as i32 + c as i32;
            }
        }
    }

    gps_sum
}