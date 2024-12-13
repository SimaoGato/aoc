use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        "RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE",
    ));
}

pub fn solve(data: String) {
    let complex: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut gardens: HashMap<char, Vec<Vec<char>>> = HashMap::new();

    for r in 0..complex.len() {
        for c in 0..complex[0].len() {
            let garden_id = complex[r][c];
            if !gardens.contains_key(&garden_id) {
                let mut garden = vec![vec!['.'; complex[0].len()]; complex.len()];
                garden[r][c] = garden_id;
                gardens.insert(garden_id, garden);
            } else {
                let garden = gardens.get_mut(&garden_id).unwrap();
                garden[r][c] = garden_id;
            }
        }
    }

    let mut total_price = 0;
    let mut total_price_sides = 0;
    for garden in gardens.values() {
        let (price, price_sides) = calculate_price(&garden);
        total_price += price;
        total_price_sides += price_sides;
    }

    println!("Part 1: {}", total_price);
    println!("Part 2: {}", total_price_sides);
}

fn calculate_price(garden: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = garden.len();
    let cols = garden[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut total_price = 0;
    let mut total_price_sides = 0;

    fn dfs(
        garden: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        r: usize,
        c: usize,
        directions: &Vec<(i32, i32)>,
    ) -> (i32, i32, i32) {
        let mut stack = vec![(r, c)];
        let mut area = 0;
        let mut perimeter = 0;
        let mut zone = HashSet::new();
        let garden_id = garden[r][c];

        while let Some((cr, cc)) = stack.pop() {
            if visited[cr][cc] {
                continue;
            }

            visited[cr][cc] = true;
            zone.insert((cr, cc));
            area += 1;

            for &(dr, dc) in directions {
                let nr = cr as i32 + dr;
                let nc = cc as i32 + dc;

                if nr >= 0 && nr < garden.len() as i32 && nc >= 0 && nc < garden[0].len() as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if garden[nr][nc] == garden[cr][cc] && !visited[nr][nc] {
                        stack.push((nr, nc));
                    } else if garden[nr][nc] != garden[cr][cc] {
                        perimeter += 1;
                    }
                } else {
                    // Edge of the garden
                    perimeter += 1;
                }
            }
        }

        // rebuild zone
        let mut zone_vec = vec![vec!['.'; garden[0].len()]; garden.len()];
        for (r, c) in &zone {
            zone_vec[*r][*c] = garden[*r][*c];
        }

        // eliminate all the rows with only dots
        zone_vec.retain(|row| row.iter().any(|&c| c != '.'));

        // eliminate all columns with only dots
        let mut cols_to_remove = Vec::new();
        let cols = zone_vec[0].len();

        // identify the columns to remove
        for c in 0..cols {
            if zone_vec.iter().all(|row| row[c] == '.') {
                cols_to_remove.push(c);
            }
        }

        // Remove columns in reverse order to prevent shifting issues
        for &col in cols_to_remove.iter().rev() {
            for row in &mut zone_vec {
                row.remove(col);
            }
        }

        let sides = count_external_sides(zone_vec.clone(), garden_id);

        return (area, perimeter, sides);
    }

    for r in 0..rows {
        for c in 0..cols {
            if garden[r][c] != '.' && !visited[r][c] {
                let (area, perimeter, sides) = dfs(garden, &mut visited, r, c, &directions);
                total_price += area * perimeter;
                total_price_sides += area * sides;
            }
        }
    }

    return (total_price, total_price_sides);
}

fn count_external_sides(grid: Vec<Vec<char>>, garden_id: char) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    if cols == 0 {
        return 0;
    }

    let mut horiz_edges: HashMap<(usize, char), Vec<usize>> = HashMap::new();
    let mut vert_edges: HashMap<(usize, char), Vec<usize>> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == garden_id {
                // Up
                if r == 0 || grid[r - 1][c] == '.' {
                    horiz_edges.entry((r, 'T')).or_default().push(c);
                }
                // Down
                if r == rows - 1 || grid[r + 1][c] == '.' {
                    horiz_edges.entry((r + 1, 'B')).or_default().push(c);
                }
                // Left
                if c == 0 || grid[r][c - 1] == '.' {
                    vert_edges.entry((c, 'L')).or_default().push(r);
                }
                // Right
                if c == cols - 1 || grid[r][c + 1] == '.' {
                    vert_edges.entry((c + 1, 'R')).or_default().push(r);
                }
            }
        }
    }

    fn count_merged_sides<K: Ord>(edges_map: HashMap<(K, char), Vec<usize>>) -> usize {
        let mut count = 0;
        let mut entries: Vec<_> = edges_map.into_iter().collect();

        entries.sort_by(|a, b| a.0.cmp(&b.0));

        for ((_key, _orientation), mut indices) in entries {
            indices.sort_unstable();
            let mut _start = indices[0];
            let mut prev = _start;
            for &x in &indices[1..] {
                if x == prev + 1 {
                    prev = x;
                } else {
                    count += 1;
                    _start = x;
                    prev = x;
                }
            }
            count += 1;
        }

        return count;
    }

    let horizontal_sides = count_merged_sides(horiz_edges);
    let vertical_sides = count_merged_sides(vert_edges);

    return (horizontal_sides + vertical_sides) as i32;
}
