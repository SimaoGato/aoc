use std::collections::VecDeque;

#[test]
fn test() {
    solve(
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_string(),
    );
}

pub fn solve(data: String) {
    const GRID_SIZE: usize = 71;
    const BYTE_LIMIT: usize = 1024;

    let corrupted_positions: Vec<(usize, usize)> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let coords: Vec<&str> = line.split(',').collect();
            let x = coords[0].trim().parse::<usize>().unwrap();
            let y = coords[1].trim().parse::<usize>().unwrap();
            (x, y)
        })
        .collect();

    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

    for i in 0..BYTE_LIMIT {
        let (x, y) = corrupted_positions[i];
        if x < GRID_SIZE && y < GRID_SIZE {
            grid[x][y] = '#';
        }
    }

    fn bfs(grid: &Vec<Vec<char>>, width: usize, height: usize) -> usize {
        let start = (0, 0);
        let goal = (width - 1, height - 1);

        let mut visited = vec![vec![false; width]; height];
        visited[start.0][start.1] = true;
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while !queue.is_empty() {
            let ((cx, cy), steps) = queue.pop_front().unwrap();
            if (cx, cy) == goal {
                return steps;
            }
            for &(dx, dy) in &directions {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[nx][ny] && grid[nx][ny] == '.' {
                        visited[nx][ny] = true;
                        queue.push_back(((nx, ny), steps + 1));
                    }
                }
            }
        }

        return 0;
    }

    let part1_result = bfs(&grid, GRID_SIZE, GRID_SIZE);
    println!("Part 1: {}", part1_result);

    for i in BYTE_LIMIT..corrupted_positions.len() {
        let (x, y) = corrupted_positions[i];
        if x < GRID_SIZE && y < GRID_SIZE {
            grid[x][y] = '#';
        }

        let result = bfs(&grid, GRID_SIZE, GRID_SIZE);
        if result == 0 {
            println!("Part 2: {},{}", x, y);
            break;
        }
    }
}
