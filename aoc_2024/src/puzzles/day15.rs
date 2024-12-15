use std::collections::VecDeque;

#[test]
fn test() {
    solve(String::from(
        "##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ));
}

pub fn solve(data: String) {
    let mut sections = data.split("\n\n");
    let map_raw = sections.next().unwrap();
    let moves_raw = sections.next().unwrap();

    let map: Vec<Vec<char>> = map_raw
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let moves_str: String = moves_raw.lines().map(|line| line.trim()).collect();
    let moves: Vec<char> = moves_str.chars().collect();

    let (start_x, start_y) = find_start(&map);

    let final_grid = get_final_grid(&map, &moves, start_x, start_y);
    let result = calculate_sum(&final_grid);
    println!("Part 1: {}", result);

    let mut new_grid = vec![];
    for i in 0..map.len() {
        let mut row = vec![];
        for j in 0..map[i].len() {
            if map[i][j] == '.' {
                row.push('.');
                row.push('.');
            } else if map[i][j] == 'O' {
                row.push('[');
                row.push(']');
            } else if map[i][j] == '#' {
                row.push('#');
                row.push('#');
            } else if map[i][j] == '@' {
                row.push('@');
                row.push('.');
            }
        }
        new_grid.push(row);
    }

    let (start_x2, start_y2) = find_start(&new_grid);

    let final_grid2 = get_final_grid(&new_grid, &moves, start_x2, start_y2);
    let result2 = calculate_sum(&final_grid2);
    println!("Part 2: {}", result2);
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                return (i, j);
            }
        }
    }
    panic!("Start position '@' not found in the map");
}

fn get_final_grid(
    map: &Vec<Vec<char>>,
    moves: &Vec<char>,
    start_x: usize,
    start_y: usize,
) -> Vec<Vec<char>> {
    let rows = map.len();
    let cols = map[0].len();
    let mut map = map.clone();
    let mut cx = start_x;
    let mut cy = start_y;

    let mut new_grid = vec![vec!['.'; cols]; rows];

    for &c in moves {
        let (dx, dy) = match c {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid move"),
        };

        let mut visited = vec![vec![false; cols]; rows];
        let mut queue = VecDeque::new();

        visited[cx][cy] = true;
        queue.push_back((cx, cy));

        let mut wall_detected = false;
        let mut to_move = vec![(cx, cy)];

        while let Some((x, y)) = queue.pop_front() {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;

            let target = map[new_x][new_y];
            if target == '#' {
                wall_detected = true;
                break;
            }

            if target == 'O' || target == '[' || target == ']' {
                if !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;
                    queue.push_back((new_x, new_y));
                    to_move.push((new_x, new_y));

                    if target == '[' {
                        let pair_x = new_x;
                        let pair_y = new_y + 1;
                        if !visited[pair_x][pair_y] {
                            visited[pair_x][pair_y] = true;
                            queue.push_back((pair_x, pair_y));
                            to_move.push((pair_x, pair_y));
                        }
                    } else if target == ']' {
                        let pair_x = new_x;
                        let pair_y = new_y - 1;
                        if !visited[pair_x][pair_y] {
                            visited[pair_x][pair_y] = true;
                            queue.push_back((pair_x, pair_y));
                            to_move.push((pair_x, pair_y));
                        }
                    }
                }
            }
        }

        if wall_detected {
            continue;
        }

        for i in 0..rows {
            for j in 0..cols {
                new_grid[i][j] = map[i][j];
            }
        }

        for &(x, y) in &to_move {
            new_grid[x][y] = '.';
        }

        for &(x, y) in &to_move {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            new_grid[nx][ny] = map[x][y];
        }

        map.clone_from(&new_grid);

        cx = (cx as i32 + dx) as usize;
        cy = (cy as i32 + dy) as usize;
    }

    return map;
}

fn calculate_sum(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            let c = grid[i][j];
            if c == 'O' || c == '[' {
                sum += 100 * (i as i32) + (j as i32);
            }
        }
    }

    sum
}
