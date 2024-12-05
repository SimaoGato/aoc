use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from(
        "47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47",
    ));
}

pub fn solve(data: String) {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let rules = parts[0].lines();
    let updates = parts[1].lines();

    // HashMaps to store higher and lower constraints for each page
    let mut higher: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lower: HashMap<i32, Vec<i32>> = HashMap::new();

    // Parse rules
    for rule in rules {
        let parts: Vec<&str> = rule.split('|').collect();
        let x: i32 = parts[0].trim().parse().unwrap();
        let y: i32 = parts[1].trim().parse().unwrap();

        higher.entry(x).or_insert_with(Vec::new).push(y);
        lower.entry(y).or_insert_with(Vec::new).push(x);
    }

    // Parse updates
    let mut updates_vec: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        let nums: Vec<i32> = update
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        updates_vec.push(nums);
    }

    // Check each update
    let mut middle_sum = 0;
    let mut corrrected_middle_sum = 0;

    for update in updates_vec {
        if is_valid_update(&update, &higher, &lower) {
            let middle_idx = update.len() / 2;
            middle_sum += update[middle_idx];
        } else {
            let corrected_update = reorder_update(&update, &higher, &lower);
            let middle_idx = corrected_update.len() / 2;
            corrrected_middle_sum += corrected_update[middle_idx];
        }
    }

    println!("Part 1: {}", middle_sum);
    println!("Part 2: {}", corrrected_middle_sum);
}

fn is_valid_update(
    update: &Vec<i32>,
    higher: &HashMap<i32, Vec<i32>>,
    lower: &HashMap<i32, Vec<i32>>,
) -> bool {
    for i in 0..update.len() {
        let page = update[i];

        // Check higher constraints
        if higher.contains_key(&page) {
            let higher_pages = &higher[&page];
            for j in 0..higher_pages.len() {
                let higher_page = higher_pages[j];
                for k in 0..update.len() {
                    if update[k] == higher_page && k < i {
                        return false; // Rule violated
                    }
                }
            }
        }

        // Check lower constraints
        if lower.contains_key(&page) {
            let lower_pages = &lower[&page];
            for j in 0..lower_pages.len() {
                let lower_page = lower_pages[j];
                for k in 0..update.len() {
                    if update[k] == lower_page && k > i {
                        return false; // Rule violated
                    }
                }
            }
        }
    }

    return true;
}

fn reorder_update(
    update: &Vec<i32>,
    higher: &HashMap<i32, Vec<i32>>,
    lower: &HashMap<i32, Vec<i32>>,
) -> Vec<i32> {
    // Build a dependency graph for the update
    let mut dependencies: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..update.len() {
        let page = update[i];
        if !dependencies.contains_key(&page) {
            dependencies.insert(page, Vec::new());
        }

        // Add higher dependencies
        if higher.contains_key(&page) {
            for j in 0..higher[&page].len() {
                let higher_page = higher[&page][j];
                if update.contains(&higher_page) {
                    dependencies.get_mut(&page).unwrap().push(higher_page);
                }
            }
        }

        // Add lower dependencies
        if lower.contains_key(&page) {
            for j in 0..lower[&page].len() {
                let lower_page = lower[&page][j];
                if update.contains(&lower_page) {
                    if !dependencies.contains_key(&lower_page) {
                        dependencies.insert(lower_page, Vec::new());
                    }
                    dependencies.get_mut(&lower_page).unwrap().push(page);
                }
            }
        }
    }

    // Perform topological sort
    let mut sorted_update = Vec::new();
    let mut no_dependencies = Vec::new();

    // Find pages with no dependencies
    for i in 0..update.len() {
        let page = update[i];
        if dependencies[&page].is_empty() {
            no_dependencies.push(page);
        }
    }

    while !no_dependencies.is_empty() {
        let current = no_dependencies.pop().unwrap();
        sorted_update.push(current);

        // Update dependencies for all remaining pages
        for i in 0..update.len() {
            let page = update[i];
            if dependencies.contains_key(&page) {
                let mut idx = 0;
                while idx < dependencies[&page].len() {
                    if dependencies[&page][idx] == current {
                        dependencies.get_mut(&page).unwrap().remove(idx);
                    } else {
                        idx += 1;
                    }
                }

                // If this page now has no dependencies, add it to the list
                if dependencies[&page].is_empty() && !sorted_update.contains(&page) {
                    no_dependencies.push(page);
                }
            }
        }
    }

    return sorted_update;
}
