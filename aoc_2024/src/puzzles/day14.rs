#[test]
fn test() {
    solve(String::from(
        "p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3",
    ));
}

pub fn solve(data: String) {
    let robots: Vec<((i32, i32), (i32, i32))> = data
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pos = parts[0][2..].split_once(',').unwrap();
            let vel = parts[1][2..].split_once(',').unwrap();
            (
                (pos.0.parse().unwrap(), pos.1.parse().unwrap()),
                (vel.0.parse().unwrap(), vel.1.parse().unwrap()),
            )
        })
        .collect();

    let width = 101;
    let height = 103;
    let seconds = 100;

    let mut final_positions = Vec::new();
    for ((px, py), (vx, vy)) in &robots {
        let final_x = ((px + vx * seconds) % width + width) % width;
        let final_y = ((py + vy * seconds) % height + height) % height;

        final_positions.push((final_x, final_y));
    }

    let mut quadrant_counts = [0; 4];
    let mid_x = width / 2;
    let mid_y = height / 2;

    for (x, y) in final_positions {
        if x == mid_x || y == mid_y {
            continue; // Skip robots in the middle
        }
        if x < mid_x && y < mid_y {
            quadrant_counts[0] += 1;
        } else if x >= mid_x && y < mid_y {
            quadrant_counts[1] += 1;
        } else if x < mid_x && y >= mid_y {
            quadrant_counts[2] += 1;
        } else {
            quadrant_counts[3] += 1;
        }
    }

    // Compute the safety factor
    let safety_factor: i32 = quadrant_counts.iter().product();

    println!("Part 1: {}", safety_factor);

    // The approach for part 2 was inspired by Neil Thistlethwaite's explanation in his video (https://www.youtube.com/watch?v=U3SoVMGpF-E). The idea involves finding the first second where there are no overlaps between robots. This implementation translates his approach to Rust.
    let mut seconds = 0;
    loop {
        let mut position_set = std::collections::HashSet::new();
        let mut overlap = false;

        for ((px, py), (vx, vy)) in &robots {
            let x = ((px + vx * seconds) % width + width) % width;
            let y = ((py + vy * seconds) % height + height) % height;

            if !position_set.insert((x, y)) {
                overlap = true;
                break;
            }
        }

        if !overlap {
            break;
        }

        seconds += 1;
    }

    println!("Part 2: {}", seconds);
}
