use std::fs;

fn main() {
    // Read input from a file
    let input = fs::read_to_string("../input/2015_001_input1.txt").expect("Failed to read file");

    // Count the number of '(' and ')' in input using iterators
    let up = input.chars().filter(|&c| c == '(').count();
    let down = input.chars().filter(|&c| c == ')').count();

    // Print the difference between up and down
    println!("{}", up - down);
}
