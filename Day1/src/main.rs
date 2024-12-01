use std::collections::HashMap;
use std::fs;

fn main() {
    // Read and parse input
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .unzip();

    // Part 1: Calculate total distance
    let part1_result = calc_total_dist(&mut left, &mut right);
    println!("Part 1 - Total Distance: {}", part1_result);

    // Part 2: Calculate similarity score
    let part2_result = calc_sim_score(&left, &right);
    println!("Part 2 - Similarity Score: {}", part2_result);
}

// Part 1: Calculate total distance between the lists
fn calc_total_dist(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    // Sort both lists
    left.sort();
    right.sort();

    // Calculate total distance
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

// Part 2: Calculate the similarity score
fn calc_sim_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // Build frequency map for the right list
    let mut right_map = HashMap::new();
    for &num in right {
        *right_map.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score
    left.iter()
        .map(|&num| num * right_map.get(&num).unwrap_or(&0))
        .sum()
}
