#[test]
fn test() {
    solve(String::from(
        "Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279",
    ));
}

pub fn solve(data: String) {
    let mut machines = Vec::new();

    for block in data.split("\n\n") {
        let mut dx_a = 0;
        let mut dy_a = 0;
        let mut dx_b = 0;
        let mut dy_b = 0;
        let mut prize_x = 0;
        let mut prize_y = 0;

        for line in block.lines() {
            if line.trim().starts_with("Button A:") {
                let parts: Vec<&str> = line.split(':').nth(1).unwrap().split(',').collect();
                dx_a = parts[0]
                    .trim()
                    .strip_prefix("X+")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                dy_a = parts[1]
                    .trim()
                    .strip_prefix("Y+")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
            } else if line.trim().starts_with("Button B:") {
                let parts: Vec<&str> = line.split(':').nth(1).unwrap().split(',').collect();
                dx_b = parts[0]
                    .trim()
                    .strip_prefix("X+")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                dy_b = parts[1]
                    .trim()
                    .strip_prefix("Y+")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
            } else if line.trim().starts_with("Prize:") {
                let parts: Vec<&str> = line.split(':').nth(1).unwrap().split(',').collect();
                prize_x = parts[0]
                    .trim()
                    .strip_prefix("X=")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                prize_y = parts[1]
                    .trim()
                    .strip_prefix("Y=")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
            }
        }

        machines.push((dx_a, dy_a, dx_b, dy_b, prize_x, prize_y));
    }

    let (total_tokens, _) = min_tokens_to_win(machines.clone(), false);
    println!("Part 1: {}", total_tokens);

    let (total_tokens, _) = min_tokens_to_win(machines, true);
    println!("Part 2: {}", total_tokens);
}

pub fn min_tokens_to_win(
    machines: Vec<(i32, i32, i32, i32, i32, i32)>,
    is_part_two: bool,
) -> (i64, usize) {
    let mut total_cost: i64 = 0;
    let mut prizes_won: usize = 0;

    for (ndx_a, ndy_a, ndx_b, ndy_b, nprize_x, nprize_y) in machines {
        let dx_a = ndx_a as i64;
        let dy_a = ndy_a as i64;
        let dx_b = ndx_b as i64;
        let dy_b = ndy_b as i64;
        let prize_x: i64;
        let prize_y: i64;

        if is_part_two {
            prize_x = nprize_x as i64 + 10_000_000_000_000;
            prize_y = nprize_y as i64 + 10_000_000_000_000;
        } else {
            prize_x = nprize_x as i64;
            prize_y = nprize_y as i64;
        }

        // Determinant
        let d = dx_a * dy_b - dx_b * dy_a;

        if d == 0 {
            continue; // no solution
        }

        // Cramer's rule
        // A = (prize_x*dy_b - prize_y*dx_b) / d
        // B = (-prize_x*dy_a + prize_y*dx_a) / d
        let a_num = prize_x * dy_b - prize_y * dx_b;
        let b_num = -prize_x * dy_a + prize_y * dx_a;

        // Check if divides evenly
        if a_num % d != 0 || b_num % d != 0 {
            continue; // not an integer solution
        }

        let a = a_num / d;
        let b = b_num / d;

        if a < 0 || b < 0 {
            continue; // can't press negative times
        }

        // Compute cost: 3 tokens per A press, 1 token per B press
        let cost = 3 * a + b;
        total_cost += cost as i64;
        prizes_won += 1;
    }

    (total_cost, prizes_won)
}
