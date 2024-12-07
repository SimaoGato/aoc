#[test]
fn test() {
    solve(String::from(
        "190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20",
    ));
}

fn initialize_combinations_part1() -> Vec<Vec<String>> {
    let mut all_combinations: Vec<Vec<String>> = vec![];

    for i in 1..12 {
        let mut combs = vec![];
        for j in 0..2_i64.pow(i as u32) {
            let mut comb = String::new();
            for k in 0..i {
                if j & (1 << k) != 0 {
                    comb.push('*');
                } else {
                    comb.push('+');
                }
            }
            combs.push(comb);
        }
        all_combinations.push(combs);
    }

    all_combinations
}

fn initialize_combinations_part2() -> Vec<Vec<String>> {
    let mut all_combinations: Vec<Vec<String>> = vec![];

    for i in 1..12 {
        let mut combs = vec![];
        for j in 0..3_i64.pow(i as u32) {
            let mut comb = String::new();
            for k in 0..i {
                match j / 3_i64.pow(k as u32) % 3 {
                    0 => comb.push('+'),
                    1 => comb.push('*'),
                    // use '#' instead of '||' to maintain the same length
                    2 => comb.push('#'),
                    _ => panic!("Invalid operator"),
                }
            }
            combs.push(comb);
        }
        all_combinations.push(combs);
    }

    all_combinations
}

pub fn solve(data: String) {
    let all_combinations = initialize_combinations_part1();
    let all_combinations_part2 = initialize_combinations_part2();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in data.lines() {
        // Get the key and the values
        let mut parts = line.trim().split(": ");
        let test_value = parts.next().unwrap().parse::<i64>().unwrap();
        let values = parts.next().unwrap();
        let numbers = values
            .split(" ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let combinations = &all_combinations[numbers.len() - 2];
        let combinations_part2 = &all_combinations_part2[numbers.len() - 2];

        if is_equation_possible(test_value, &numbers, combinations) {
            sum1 += test_value;
            sum2 += test_value;
        } else if is_equation_possible_part2(test_value, &numbers, combinations_part2) {
            sum2 += test_value;
        }
    }

    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}

fn is_equation_possible(_test_value: i64, _numbers: &[i64], _operators: &[String]) -> bool {
    for comb in _operators {
        let mut numbers = _numbers.to_vec();
        let mut sum = numbers.remove(0);
        for op in comb.chars() {
            let n = numbers.remove(0);
            match op {
                '+' => sum += n,
                '*' => sum *= n,
                _ => panic!("Invalid operator"),
            }
        }
        if sum == _test_value {
            return true;
        }
    }

    return false;
}

fn is_equation_possible_part2(_test_value: i64, _numbers: &[i64], _operators: &[String]) -> bool {
    for comb in _operators {
        let mut numbers = _numbers.to_vec();
        let mut sum = numbers.remove(0);
        for op in comb.chars() {
            let n = numbers.remove(0);
            match op {
                '+' => sum += n,
                '*' => sum *= n,
                '#' => sum = format!("{}{}", sum, n).parse::<i64>().unwrap(),
                _ => panic!("Invalid operator"),
            }
        }
        if sum == _test_value {
            return true;
        }
    }

    return false;
}
