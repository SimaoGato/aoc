use std::fs;

fn main() {

    // Read input from a file
    let input = fs::read_to_string("../input/2015_001_input1.txt").expect("Failed to read file");

    // Count the number of characters till ) becames +1 than (
    let mut floor = 0;
    let mut position = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            break;
        }
        position += 1;
    }

    // Print current floor
    println!("{}", position + 1);

}