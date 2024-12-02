use std::fs;

fn main() {
    // read the input file
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // split input into reports
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // check each report and count the safe ones
    let safe_count = reports.iter().filter(|report| is_safe_with_dampener(report)).count();

    // print result
    println!("Number of safe reports with dampener: {}", safe_count);
}

// helper function to check if a report is safe
fn is_safe(report: &Vec<i32>) -> bool {
    // check if the report is strictly increasing or decreasing
    let is_increasing = report.windows(2).all(|pair| pair[1] > pair[0] && (pair[1] - pair[0]).abs() <= 3);
    let is_decreasing = report.windows(2).all(|pair| pair[1] < pair[0] && (pair[1] - pair[0]).abs() <= 3);

    // return true if either condition is satisfied
    is_increasing || is_decreasing
}

// Helper function to check if a report is safe with the Problem Dampener
fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    // If the report is already safe, return true
    if is_safe(report) {
        return true;
    }

    // Try removing each level one by one
    for i in 0..report.len() {
        // Create a new report with the current level removed
        let mut modified_report = report.clone();
        modified_report.remove(i);

        // Check if the modified report is safe
        if is_safe(&modified_report) {
            return true;
        }
    }

    // If no single-level removal makes the report safe, return false
    false
}
