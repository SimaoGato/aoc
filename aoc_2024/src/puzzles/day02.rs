use itertools::Itertools;

#[test]
fn test() {
    solve(String::from(
        "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9",
    ));
}

pub fn solve(data: String) {
    let mut safe_count = 0;

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in data.lines() {
        let mut row: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse::<i32>().unwrap());
        }
        matrix.push(row);
    }

    let mut _size = 0;
    let mut _valid = true;

    for row in matrix.iter() {
        if not_sorted(row) {
            continue;
        } else {
            _valid = true;
            _size = row.len() - 1;
            for i in 0.._size {
                let diff = (row[i] - row[i + 1]).abs();
                if diff < 1 || diff > 3 {
                    _valid = false;
                }
                if !_valid {
                    break;
                }
            }
            if _valid {
                safe_count += 1;
            }
        }
    }

    println!("Part 1: {}", safe_count);

    safe_count = 0;

    for row in matrix.iter() {
        if is_safe(row) {
            safe_count += 1;
        }
    }

    println!("Part 2: {}", safe_count);
}

fn not_sorted(row: &Vec<i32>) -> bool {
    let is_ascending = row.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = row.windows(2).all(|w| w[0] >= w[1]);

    return !is_ascending && !is_descending;
}

fn is_safe(row: &Vec<i32>) -> bool {
    let _size = row.len();
    let combinations = row.iter().combinations(_size - 1);
    let mut _valid = false;

    for comb in combinations {
        let owned_comb: Vec<i32> = comb.into_iter().copied().collect();
        if not_sorted(&owned_comb) {
            continue;
        } else {
            _valid = true;
            for i in 0.._size - 2 {
                let diff = (owned_comb[i] - owned_comb[i + 1]).abs();
                if diff < 1 || diff > 3 {
                    _valid = false;
                }
                if !_valid {
                    break;
                }
            }
            if _valid {
                return true;
            }
        }
    }

    return false;
}
