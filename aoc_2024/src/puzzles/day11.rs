use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from("125 17"));
}

const ITERATIONS: i32 = 75;

pub fn solve(data: String) {
    let mut stone_counts: HashMap<i64, i64> = HashMap::new();

    let initial_stones: Vec<&str> = data.split_whitespace().collect();

    for stone_str in initial_stones {
        let stone: i64 = stone_str.parse().expect("Invalid input");

        if stone_counts.contains_key(&stone) {
            *stone_counts.get_mut(&stone).unwrap() += 1;
        } else {
            stone_counts.insert(stone, 1);
        }
    }

    for j in 0..ITERATIONS {
        let mut next_stone_counts: HashMap<i64, i64> = HashMap::new();

        for (&stone, &count) in &stone_counts {
            match stone {
                0 => {
                    // Rule 1: 0 becomes 1
                    increment_count(&mut next_stone_counts, 1, count);
                }
                x if x.to_string().len() % 2 == 0 => {
                    // Rule 2: Even digit stones split
                    let digits = x.to_string().len();
                    let half = digits / 2;
                    let divisor = 10_i64.pow(half as u32);

                    let left_stone = x / divisor;
                    let right_stone = x % divisor;

                    increment_count(&mut next_stone_counts, left_stone, count);
                    increment_count(&mut next_stone_counts, right_stone, count);
                }
                x => {
                    // Rule 3: Multiply by 2024
                    let new_stone = x * 2024;
                    increment_count(&mut next_stone_counts, new_stone, count);
                }
            }
        }

        stone_counts = next_stone_counts;

        if j == 24 {
            let total_stones: i64 = stone_counts.values().sum();
            println!("Part 1: {}", total_stones);
        }
    }

    let total_stones: i64 = stone_counts.values().sum();
    println!("Part 2: {}", total_stones);
}

fn increment_count(counts: &mut HashMap<i64, i64>, stone: i64, additional_count: i64) {
    if counts.contains_key(&stone) {
        *counts.get_mut(&stone).unwrap() += additional_count;
    } else {
        counts.insert(stone, additional_count);
    }
}
