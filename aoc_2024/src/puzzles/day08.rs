use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        "............
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
        ............",
    ));
}

pub fn solve(data: String) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        grid.push(line.trim().chars().collect());
    }

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut antennas: Vec<(i32, i32, char)> = Vec::new();

    // Find all antenna positions and their frequencies
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c.is_alphanumeric() {
                antennas.push((x as i32, y as i32, c));
            }
        }
    }

    let mut antinode_positions: HashSet<(i32, i32)> = HashSet::new();

    // Calculate antinodes for each pair of antennas with the same frequency
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (x1, y1, f1) = antennas[i];
            let (x2, y2, f2) = antennas[j];

            if f1 == f2 {
                // Calculate the differences in coordinates
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate the antinodes
                let antinode_1 = (x1 - dx, y1 - dy);
                let antinode_2 = (x2 + dx, y2 + dy);

                // Add the antinodes only if they are within bounds
                if antinode_1.0 >= 0
                    && antinode_1.1 >= 0
                    && antinode_1.0 < width
                    && antinode_1.1 < height
                {
                    antinode_positions.insert(antinode_1);
                }
                if antinode_2.0 >= 0
                    && antinode_2.1 >= 0
                    && antinode_2.0 < width
                    && antinode_2.1 < height
                {
                    antinode_positions.insert(antinode_2);
                }
            }
        }
    }

    // Print the total number of unique valid antinodes
    println!("Part 1: {}", antinode_positions.len());

    // Group antennas by frequency
    let mut freq_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (x, y, f) in antennas {
        freq_map.entry(f).or_insert(Vec::new()).push((x, y));
    }

    let mut antinode_positions: HashSet<(i32, i32)> = HashSet::new();

    // Helper function to compute gcd
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a.abs()
        } else {
            gcd(b, a % b)
        }
    }

    // Process each frequency group
    for (_freq, coords) in freq_map.iter() {
        // Only if we have at least two antennas
        if coords.len() < 2 {
            continue;
        }

        // For each pair of antennas
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];

                let mut dx = x2 - x1;
                let mut dy = y2 - y1;

                // Normalize direction vector (dx, dy)
                let g = gcd(dx, dy);
                dx /= g;
                dy /= g;

                // Ensure a consistent direction for uniqueness (e.g., dx > 0 or if dx=0 then dy > 0)
                if dx < 0 || (dx == 0 && dy < 0) {
                    dx = -dx;
                    dy = -dy;
                }

                // From one antenna, move along the line in both directions
                // Forward direction
                let mut step = 0;
                loop {
                    let nx = x1 + step * dx;
                    let ny = y1 + step * dy;
                    if nx < 0 || nx >= width || ny < 0 || ny >= height {
                        break;
                    }
                    antinode_positions.insert((nx, ny));
                    step += 1;
                }

                // Backward direction
                let mut step = 1;
                loop {
                    let nx = x1 - step * dx;
                    let ny = y1 - step * dy;
                    if nx < 0 || nx >= width || ny < 0 || ny >= height {
                        break;
                    }
                    antinode_positions.insert((nx, ny));
                    step += 1;
                }
            }
        }
    }

    println!("Part 2: {}", antinode_positions.len());
}
