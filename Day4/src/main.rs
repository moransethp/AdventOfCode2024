use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // open the input file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // read the grid into a Vec<Vec<char>>
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let height = grid.len();
    if height == 0 {
        println!("The grid is empty.");
        return Ok(());
    }
    let width = grid[0].len();

    let mut count = 0;

    // loop over the grid, avoiding the borders
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if grid[y][x] == 'A' {
                // check the first diagonal (top-left to bottom-right)
                let diag1 = format!(
                    "{}{}{}",
                    grid[y - 1][x - 1],
                    grid[y][x],
                    grid[y + 1][x + 1]
                );
                // check the second diagonal (top-right to bottom-left)
                let diag2 = format!(
                    "{}{}{}",
                    grid[y - 1][x + 1],
                    grid[y][x],
                    grid[y + 1][x - 1]
                );

                let valid_diag1 = diag1 == "MAS" || diag1 == "SAM";
                let valid_diag2 = diag2 == "MAS" || diag2 == "SAM";

                if valid_diag1 && valid_diag2 {
                    count += 1;
                }
            }
        }
    }

    // output the total count of X-MAS patterns
    println!("{}", count);

    Ok(())
}
