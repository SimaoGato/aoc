use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("../input/2015_003_input.txt").expect("Failed to read file");

    let initial_position = (0, 0);
    let mut houses: HashMap<(i32, i32), i32> = HashMap::new();
    houses.insert(initial_position, 0);

    let mut santa_position = initial_position;

    for c in input.chars() {
        match c {
            '^' => santa_position.1 += 1,
            'v' => santa_position.1 -= 1,
            '>' => santa_position.0 += 1,
            '<' => santa_position.0 -= 1,
            _ => (),
        }
        houses.entry(santa_position).or_insert(0);
    }

    println!("Houses visited: {}", houses.len());
}