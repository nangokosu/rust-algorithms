use std::fs::File;

use std::io::{BufReader, BufRead};

use std::collections::HashSet;


fn main() {

    let file = File::open("2Sum.txt").expect("Failed to open file");
    let buf_read = BufReader::new(file);
    let mut numbers: Vec<i64> = buf_read.lines().map(|line: Result<String, std::io::Error>| line.expect("Failed to read line")
    .trim().parse::<i64>().expect("Failed to parse integer")).collect();


    numbers.sort_unstable(); // sorting to make implementation faster

    numbers.dedup();

    let mut valid_targets: HashSet<i64> = HashSet::new();

    for &x in &numbers {
        let low_bound = -10000 - x;
        let high_bound = 10000 - x;

        
        let start_idx = match numbers.binary_search(&low_bound) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        // Scan from the start index forward
        for &y in &numbers[start_idx..] {
            if y > high_bound {
                break; // Stop immediately when elements get too large
            }

            // Explicitly enforce the prompt's condition: x and y must be distinct
            if x != y {
                valid_targets.insert(x + y);
            }
        }
    }

    println!("Total target value: {}", valid_targets.len());
    

    
}

 

