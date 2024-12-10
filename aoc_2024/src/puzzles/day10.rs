use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732",
    ));
}

pub fn solve(data: String) {
    let grid: Vec<Vec<u32>> = data
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut total_score = 0;
    let mut total_rating = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                total_score += calculate_score(&grid, r, c);
                total_rating += calculate_rating(&grid, r, c);
            }
        }
    }

    println!("Part 1: {}", total_score);
    println!("Part 2: {}", total_rating);
}

fn calculate_score(grid: &Vec<Vec<u32>>, start_row: usize, start_col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut stack = Vec::new();
    stack.push((start_row, start_col, 0));
    let mut score = 0;

    while stack.len() > 0 {
        let current = stack.pop().unwrap();
        let row = current.0;
        let col = current.1;
        let height = current.2;

        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;

        if grid[row][col] == 9 {
            score += 1;
        }

        // Explore neighbors
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for i in 0..directions.len() {
            let new_row = row as isize + directions[i].0;
            let new_col = col as isize + directions[i].1;

            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == height + 1 && !visited[new_row][new_col] {
                    stack.push((new_row, new_col, height + 1));
                }
            }
        }
    }

    return score;
}

fn calculate_rating(grid: &Vec<Vec<u32>>, start_row: usize, start_col: usize) -> usize {
    let mut visited = HashSet::new();
    return count_paths(grid, &mut visited, start_row, start_col, 0);
}

fn count_paths(
    grid: &Vec<Vec<u32>>,
    visited: &mut HashSet<(usize, usize)>,
    row: usize,
    col: usize,
    height: u32,
) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    if grid[row][col] != height {
        return 0;
    }

    if grid[row][col] == 9 {
        return 1;
    }

    let mut path_count = 0;
    visited.insert((row, col));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for i in 0..directions.len() {
        let new_row = row as isize + directions[i].0;
        let new_col = col as isize + directions[i].1;

        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if !visited.contains(&(new_row, new_col)) {
                path_count += count_paths(grid, visited, new_row, new_col, height + 1);
            }
        }
    }

    visited.remove(&(row, col));
    return path_count;
}
