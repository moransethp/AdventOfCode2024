use std::fs;
use regex::Regex;

fn main() {
    // read the input from file "input.txt"
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // regex to match cases
    let re_instr = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    // init vars
    let mut enabled = true; // For Part Two
    let mut sum_part1 = 0;  // Sum for Part One
    let mut sum_part2 = 0;  // Sum for Part Two

    // iterate over all matches of the instructions
    for m in re_instr.find_iter(&input) {
        let s = m.as_str();

        if let Some(caps) = re_mul.captures(s) {
            // valid mul(X,Y)
            let x: u32 = caps[1].parse().unwrap();
            let y: u32 = caps[2].parse().unwrap();
            let product = x * y;
            sum_part1 += product; // part 1 sum
            if enabled {
                sum_part2 += product; // part 2 sum
            }
        } else if re_do.is_match(s) {
            // do() instruction found
            enabled = true;
        } else if re_dont.is_match(s) {
            // don't() instruction found
            enabled = false;
        }
        // ignore any other matches
    }

    // output the results
    println!("Part 1: Sum of all mul instructions: {}", sum_part1);
    println!("Part 2: Sum of enabled mul instructions: {}", sum_part2);
}
