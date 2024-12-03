#[test]
fn test() {
    solve(String::from(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ));
}

pub fn solve(data: String) {
    part1(data.clone());
    part2(data);
}

pub fn part2(data: String) {
    let mut has_m = false;
    let mut has_u = false;
    let mut has_l = false;
    let mut has_open_bracket = false;
    let mut digit1 = 0;
    let mut digit2 = 0;
    let mut reading_digit1 = true;
    let mut sum = 0;
    let mut do_mul = true;
    let mut has_d = false;
    let mut has_o = false;
    let mut has_n = false;
    let mut has_ap = false;
    let mut has_t = false;
    let mut has_open_bracket_do = false;
    let mut has_open_bracket_dont = false;

    for line in data.lines() {
        for c in line.chars() {
            match c {
                'd' => {
                    if !has_d {
                        has_d = true;
                    }
                }
                'o' => {
                    if has_d && !has_o {
                        has_o = true;
                    }
                }
                'n' => {
                    if has_d && has_o && !has_n {
                        has_n = true;
                    }
                }
                '\'' => {
                    if has_d && has_o && has_n && !has_ap {
                        has_ap = true;
                    }
                }
                't' => {
                    if has_d && has_o && has_n && has_ap && !has_t {
                        has_t = true;
                    }
                }
                'm' => {
                    if !has_m {
                        has_m = true;
                    }
                }
                'u' => {
                    if has_m && !has_u {
                        has_u = true;
                    }
                }
                'l' => {
                    if has_m && has_u && !has_l {
                        has_l = true;
                    }
                }
                '(' => {
                    if has_m && has_u && has_l && !has_open_bracket {
                        has_open_bracket = true;
                        reading_digit1 = true;
                        digit1 = 0;
                    }

                    if has_d && has_o && has_n && has_ap && has_t && !has_open_bracket_dont {
                        has_open_bracket_dont = true;
                    }

                    if has_d && has_o && !has_n && !has_ap && !has_t && !has_open_bracket_do {
                        has_open_bracket_do = true;
                    }
                }
                ',' => {
                    if has_open_bracket {
                        reading_digit1 = false;
                        digit2 = 0;
                    }
                }
                ')' => {
                    if has_open_bracket_dont {
                        if has_d && has_o && has_n && has_ap && has_t {
                            do_mul = false;
                        }
                        has_open_bracket_dont = false;
                        has_d = false;
                        has_o = false;
                        has_n = false;
                        has_ap = false;
                        has_t = false;
                    }

                    if has_open_bracket_do {
                        if has_d && has_o {
                            do_mul = true;
                        }
                        has_open_bracket_do = false;
                        has_d = false;
                        has_o = false;
                    }

                    if has_open_bracket {
                        if do_mul {
                            sum += digit1 * digit2;
                        }
                        has_m = false;
                        has_u = false;
                        has_l = false;
                        has_open_bracket = false;
                        digit1 = 0;
                        digit2 = 0;
                    }
                }
                c if c.is_digit(10) => {
                    if has_d || has_o || has_n || has_ap || has_t {
                        has_d = false;
                        has_o = false;
                        has_n = false;
                        has_ap = false;
                        has_t = false;
                    } else {
                        let value = c.to_digit(10).unwrap() as i32;
                        if reading_digit1 {
                            digit1 = digit1 * 10 + value;
                        } else {
                            digit2 = digit2 * 10 + value;
                        }
                    }
                }
                _ => {
                    has_m = false;
                    has_u = false;
                    has_l = false;
                    has_open_bracket = false;
                    digit1 = 0;
                    digit2 = 0;
                    has_d = false;
                    has_o = false;
                    has_n = false;
                    has_ap = false;
                    has_t = false;
                    has_open_bracket_do = false;
                    has_open_bracket_dont = false;
                }
            }
        }
    }

    println!("Day 3 Part 2: {}", sum);
}

pub fn part1(data: String) {
    let mut has_m = false;
    let mut has_u = false;
    let mut has_l = false;
    let mut has_open_bracket = false;
    let mut digit1 = 0;
    let mut digit2 = 0;
    let mut reading_digit1 = true;
    let mut sum = 0;

    for line in data.lines() {
        for c in line.chars() {
            match c {
                'm' if !has_m => has_m = true,
                'u' if has_m && !has_u => has_u = true,
                'l' if has_m && has_u && !has_l => has_l = true,
                '(' if has_m && has_u && has_l && !has_open_bracket => {
                    has_open_bracket = true;
                    reading_digit1 = true;
                    digit1 = 0;
                }
                ',' if has_open_bracket => {
                    reading_digit1 = false;
                    digit2 = 0;
                }
                ')' if has_open_bracket => {
                    sum += digit1 * digit2;
                    has_m = false;
                    has_u = false;
                    has_l = false;
                    has_open_bracket = false;
                    digit1 = 0;
                    digit2 = 0;
                }
                _ if c.is_digit(10) => {
                    let value = c.to_digit(10).unwrap() as i32;
                    if reading_digit1 {
                        digit1 = digit1 * 10 + value;
                    } else {
                        digit2 = digit2 * 10 + value;
                    }
                }
                _ => {
                    has_m = false;
                    has_u = false;
                    has_l = false;
                    has_open_bracket = false;
                    digit1 = 0;
                    digit2 = 0;
                }
            }
        }
    }

    println!("Part 1: {}", sum);
}
