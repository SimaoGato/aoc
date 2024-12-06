#[test]
fn test() {
    solve(String::from(
        "....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...",
    ));
}

pub fn solve(data: String) {
    let map: Vec<Vec<char>> = data.lines().map(|l| l.trim().chars().collect()).collect();

    // clone map
    let mut map_clone = map.clone();

    let mut initial_pos = (0, 0);
    let mut state = (0, 0, 0);

    for (y, row) in map_clone.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                initial_pos = (y as i32, x as i32);
                state = (y as i32, x as i32, 0);
                break;
            }
        }
    }

    let mut stop = false;
    let mut going_up = true;
    let mut going_right = false;
    let mut going_down = false;
    let mut going_left = false;

    let row_len = map_clone[0].len();

    map_clone[initial_pos.0 as usize][initial_pos.1 as usize] = '.';

    let mut positions_visited = vec![];

    while !stop {
        if going_up {
            if map_clone[initial_pos.0 as usize][initial_pos.1 as usize] == '.' {
                positions_visited.push((initial_pos.0, initial_pos.1));
                map_clone[initial_pos.0 as usize][initial_pos.1 as usize] = 'X';
            }
            if initial_pos.0 == 0 {
                stop = true;
            } else if map_clone[initial_pos.0 as usize - 1][initial_pos.1 as usize] == '#' {
                going_up = false;
                going_right = true;
                initial_pos.1 += 1;
            } else {
                initial_pos.0 -= 1;
            }
        } else if going_right {
            if map_clone[initial_pos.0 as usize][initial_pos.1 as usize] == '.' {
                positions_visited.push((initial_pos.0, initial_pos.1));
                map_clone[initial_pos.0 as usize][initial_pos.1 as usize] = 'X';
            }
            if initial_pos.1 == row_len as i32 - 1 {
                stop = true;
            } else if map_clone[initial_pos.0 as usize][initial_pos.1 as usize + 1] == '#' {
                going_right = false;
                going_down = true;
                initial_pos.0 += 1;
            } else {
                initial_pos.1 += 1;
            }
        } else if going_down {
            if map_clone[initial_pos.0 as usize][initial_pos.1 as usize] == '.' {
                positions_visited.push((initial_pos.0, initial_pos.1));
                map_clone[initial_pos.0 as usize][initial_pos.1 as usize] = 'X';
            }
            if initial_pos.0 == map.len() as i32 - 1 {
                stop = true;
            } else if map_clone[initial_pos.0 as usize + 1][initial_pos.1 as usize] == '#' {
                going_down = false;
                going_left = true;
                initial_pos.1 -= 1;
            } else {
                initial_pos.0 += 1;
            }
        } else if going_left {
            if map_clone[initial_pos.0 as usize][initial_pos.1 as usize] == '.' {
                positions_visited.push((initial_pos.0, initial_pos.1));
                map_clone[initial_pos.0 as usize][initial_pos.1 as usize] = 'X';
            }
            if initial_pos.1 == 0 {
                stop = true;
            } else if map_clone[initial_pos.0 as usize][initial_pos.1 as usize - 1] == '#' {
                going_left = false;
                going_up = true;
                initial_pos.0 -= 1;
            } else {
                initial_pos.1 -= 1;
            }
        }
    }

    println!("Part 1: {}", positions_visited.len());

    let mut cycle_count = 0;

    for pos in positions_visited.iter() {
        map_clone = map.clone();
        map_clone[pos.0 as usize][pos.1 as usize] = '#';
        if has_cycle(map_clone, state) {
            cycle_count += 1;
        }
    }

    println!("Part 2: {}", cycle_count);
}

pub fn has_cycle(map: Vec<Vec<char>>, mut state: (i32, i32, i32)) -> bool {
    let mut visited_states = std::collections::HashSet::new();

    loop {
        // Record the current state
        if visited_states.contains(&state) {
            return true; // Cycle detected
        }
        visited_states.insert(state);

        // Simulate movement
        let (y, x, dir) = state;
        let (dy, dx) = match dir {
            0 => (-1, 0), // Up
            1 => (0, 1),  // Right
            2 => (1, 0),  // Down
            3 => (0, -1), // Left
            _ => panic!("Invalid direction"),
        };

        let new_y = y + dy;
        let new_x = x + dx;

        // Check bounds
        if new_y < 0 || new_y >= map.len() as i32 || new_x < 0 || new_x >= map[0].len() as i32 {
            break; // Guard leaves the map
        }

        // Update state
        if map[new_y as usize][new_x as usize] == '#' {
            state.2 = (state.2 + 1) % 4;
        } else {
            state = (new_y, new_x, state.2);
        }
    }

    return false;
}
