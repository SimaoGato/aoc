use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        "r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb",
    ));
}

pub fn solve(data: String) {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let patterns: HashSet<String> = parts[0].split(',').map(|s| s.trim().to_string()).collect();
    let designs: Vec<String> = parts[1]
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let mut possible_count = 0;
    let mut total_ways = 0;

    for design in &designs {
        if can_make_design(design, &patterns) {
            possible_count += 1;
            total_ways += count_ways_to_make_design(design, &patterns);
        }
    }

    println!("Part 1: {}", possible_count);
    println!("Part 2: {}", total_ways);
}

fn can_make_design(design: &str, patterns: &HashSet<String>) -> bool {
    let mut dp = vec![false; design.len() + 1];
    dp[0] = true;

    for i in 1..=design.len() {
        for j in 0..i {
            if dp[j] && patterns.contains(&design[j..i].to_string()) {
                dp[i] = true;
                break;
            }
        }
    }

    return dp[design.len()];
}

fn count_ways_to_make_design(design: &str, patterns: &HashSet<String>) -> usize {
    let mut dp = vec![0; design.len() + 1];
    dp[0] = 1;

    for i in 1..=design.len() {
        for j in 0..i {
            if patterns.contains(&design[j..i].to_string()) {
                dp[i] += dp[j];
            }
        }
    }

    return dp[design.len()];
}
