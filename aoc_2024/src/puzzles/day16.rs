use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[test]
fn test() {
    let data = String::from(
        "###############
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
    ###############",
    );
    solve(data);
}

pub fn solve(data: String) {
    let maze: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let rows = maze.len();
    let cols = maze[0].len();

    let mut start_x = 0;
    let mut start_y = 0;
    for i in 0..rows {
        for j in 0..cols {
            if maze[i][j] == 'S' {
                start_x = i;
                start_y = j;
                break;
            }
        }
    }

    let (min_cost, cost) = find_lowest_score(&maze, start_x, start_y);

    println!("Part 1: {}", min_cost);

    let on_best_path = find_tiles_on_best_paths(&maze, &cost);

    let mut count_on_best_path = 0;

    for row in &on_best_path {
        for &tile in row {
            if tile {
                count_on_best_path += 1;
            }
        }
    }

    println!("Part 2: {}", count_on_best_path);
}

fn find_lowest_score(
    maze: &[Vec<char>],
    start_x: usize,
    start_y: usize,
) -> (usize, Vec<Vec<Vec<usize>>>) {
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let rows = maze.len();
    let cols = maze[0].len();

    let mut cost = vec![vec![vec![99999999; 4]; cols]; rows];

    cost[start_x][start_y][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start_x, start_y, 0));

    let mut visited = vec![vec![vec![false; 4]; cols]; rows];
    let mut min_cost_to_end = 99999999;

    while let Some((Reverse(cur_cost), cur_x, cur_y, cur_dir)) = heap.pop() {
        if visited[cur_x][cur_y][cur_dir] {
            continue;
        }
        visited[cur_x][cur_y][cur_dir] = true;

        if maze[cur_x][cur_y] == 'E' {
            if cur_cost < min_cost_to_end {
                min_cost_to_end = cur_cost;
            }
        }

        // Move forward
        if let Some((nx, ny)) = next_cell(cur_x, cur_y, dx[cur_dir], dy[cur_dir], rows, cols) {
            if maze[nx][ny] != '#' {
                let forward_cost = cur_cost + 1;
                if forward_cost < cost[nx][ny][cur_dir] {
                    cost[nx][ny][cur_dir] = forward_cost;
                    heap.push((Reverse(forward_cost), nx, ny, cur_dir));
                }
            }
        }

        // Turn left
        let left_dir = (cur_dir + 3) % 4;
        let left_cost = cur_cost + 1000;
        if left_cost < cost[cur_x][cur_y][left_dir] {
            cost[cur_x][cur_y][left_dir] = left_cost;
            heap.push((Reverse(left_cost), cur_x, cur_y, left_dir));
        }

        // Turn right
        let right_dir = (cur_dir + 1) % 4;
        let right_cost = cur_cost + 1000;
        if right_cost < cost[cur_x][cur_y][right_dir] {
            cost[cur_x][cur_y][right_dir] = right_cost;
            heap.push((Reverse(right_cost), cur_x, cur_y, right_dir));
        }
    }

    return (min_cost_to_end, cost);
}

fn find_tiles_on_best_paths(maze: &[Vec<char>], cost: &[Vec<Vec<usize>>]) -> Vec<Vec<bool>> {
    let rows = maze.len();
    let cols = maze[0].len();

    let mut min_end_cost = 99999999;
    let mut end_x = 0;
    let mut end_y = 0;

    for x in 0..rows {
        for y in 0..cols {
            if maze[x][y] == 'E' {
                for d in 0..4 {
                    if cost[x][y][d] < min_end_cost {
                        min_end_cost = cost[x][y][d];
                        end_x = x;
                        end_y = y;
                    }
                }
            }
        }
    }

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let mut on_path_state = vec![vec![vec![false; 4]; cols]; rows];
    let mut queue = VecDeque::new();

    for d in 0..4 {
        if cost[end_x][end_y][d] == min_end_cost {
            on_path_state[end_x][end_y][d] = true;
            queue.push_back((end_x, end_y, d));
        }
    }

    while let Some((x, y, dir)) = queue.pop_front() {
        let cur_cost = cost[x][y][dir];

        if cur_cost > 0 {
            if let Some((px, py)) = prev_cell(x, y, dx[dir], dy[dir], rows, cols) {
                if maze[px][py] != '#' {
                    let prev_cost = cur_cost - 1;
                    if cost[px][py][dir] == prev_cost && !on_path_state[px][py][dir] {
                        on_path_state[px][py][dir] = true;
                        queue.push_back((px, py, dir));
                    }
                }
            }
        }

        if cur_cost >= 1000 {
            let left_dir = (dir + 1) % 4;
            let left_prev_cost = cur_cost - 1000;
            if cost[x][y][left_dir] == left_prev_cost && !on_path_state[x][y][left_dir] {
                on_path_state[x][y][left_dir] = true;
                queue.push_back((x, y, left_dir));
            }
        }

        if cur_cost >= 1000 {
            let right_dir = (dir + 3) % 4;
            let right_prev_cost = cur_cost - 1000;
            if cost[x][y][right_dir] == right_prev_cost && !on_path_state[x][y][right_dir] {
                on_path_state[x][y][right_dir] = true;
                queue.push_back((x, y, right_dir));
            }
        }
    }

    let mut on_path_tile = vec![vec![false; cols]; rows];
    for x in 0..rows {
        for y in 0..cols {
            if maze[x][y] != '#' {
                for d in 0..4 {
                    if on_path_state[x][y][d] {
                        on_path_tile[x][y] = true;
                        break;
                    }
                }
            }
        }
    }

    return on_path_tile;
}

fn next_cell(
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    rows: usize,
    cols: usize,
) -> Option<(usize, usize)> {
    let nx_i = x as i32 + dx;
    let ny_i = y as i32 + dy;
    if nx_i < 0 || ny_i < 0 {
        return None;
    }
    let nx = nx_i as usize;
    let ny = ny_i as usize;
    if nx < rows && ny < cols {
        Some((nx, ny))
    } else {
        None
    }
}

fn prev_cell(
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    rows: usize,
    cols: usize,
) -> Option<(usize, usize)> {
    let px_i = x as i32 - dx;
    let py_i = y as i32 - dy;
    if px_i < 0 || py_i < 0 {
        return None;
    }
    let px = px_i as usize;
    let py = py_i as usize;
    if px < rows && py < cols {
        Some((px, py))
    } else {
        None
    }
}
