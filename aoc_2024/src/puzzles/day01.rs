#[test]
fn test() {
    solve(String::from(
        "3   4
    4   3
    2   5
    1   3
    3   9
    3   3",
    ));
}

pub fn solve(data: String) {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in data.lines() {
        let mut nums = line.split_whitespace();
        let a = nums.next().unwrap().parse::<i32>().unwrap();
        let b = nums.next().unwrap().parse::<i32>().unwrap();
        v1.push(a);
        v2.push(b);
    }

    v1.sort();
    v2.sort();

    let size = v1.len();

    let mut sum = 0;

    for i in 0..size {
        if v2[i] > v1[i] {
            sum += v2[i] - v1[i];
        } else {
            sum += v1[i] - v2[i];
        }
    }

    println!("Part 1: {}", sum);

    sum = 0;
    let mut _count = 0;

    for i in 0..size {
        _count = v2.iter().filter(|&n| *n == v1[i]).count();
        sum += v1[i] * (_count as i32);
    }

    println!("Part 2: {}", sum);
}
