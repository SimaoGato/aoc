use std::fs;

fn main() {
    // Read file into input
    let input = fs::read_to_string("../input/2015_002_input.txt").expect("Something went wrong reading the file");

    let mut total_wrap = 0;
    let mut total_ribbon = 0;

    // Split input into lines
    let lines: Vec<&str> = input.split("\n").collect();
    for l in lines {
        // Split line into dimensions
        let dims: Vec<&str> = l.split("x").collect();
        // Convert dimensions to integers
        let mut d: Vec<i32> = Vec::new();
        for dim in dims {
            d.push(dim.parse::<i32>().unwrap());
        }
        // Calculate surface area
        let mut surface_area = 2 * d[0] * d[1] + 2 * d[1] * d[2] + 2 * d[2] * d[0];
        // Find smallest side
        let mut smallest_side = d[0] * d[1];
        let mut smallest_perimiter = 2 * d[0] + 2 * d[1];
        if d[1] * d[2] < smallest_side {
            smallest_side = d[1] * d[2];
            smallest_perimiter = 2 * d[1] + 2 * d[2];
        }
        if d[2] * d[0] < smallest_side {
            smallest_side = d[2] * d[0];
            smallest_perimiter = 2 * d[2] + 2 * d[0];
        }
        // Add smallest side to surface area
        surface_area += smallest_side;
        // Print result
        total_wrap += surface_area;
        total_ribbon += smallest_perimiter + d[0] * d[1] * d[2];
    }

    println!("Total wrapping paper needed: {}\nTotal ribbon needed: {}", total_wrap, total_ribbon);
}
