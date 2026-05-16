use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Job {
    pub weight: i64,
    pub length: i64,
}

fn main() {
    let file = File::open("scheduling.txt").expect("Could not open file");
    let mut lines = BufReader::new(file).lines();

    // Skip or parse the first line containing the total count
    let _number_of_jobs: usize = lines
        .next()
        .expect("First line not found")
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut jobs: Vec<Job> = Vec::new();

    for line in lines {
        let line_str = line.unwrap();
        let parts: Vec<i64> = line_str
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        
        if parts.len() >= 2 {
            jobs.push(Job {
                weight: parts[0],
                length: parts[1],
            });
        }
    }

    // Difference Metric (w - l) ---
    let mut jobs_1 = jobs.clone();
    // Sort descending by (weight - length), then by weight.
    jobs_1.sort_unstable_by(|a, b| {
        let diff_a = a.weight - a.length;
        let diff_b = b.weight - b.length;
        
        diff_b.cmp(&diff_a).then_with(|| b.weight.cmp(&a.weight))
    });

    let mut total_weighted_time_1: i64 = 0;
    let mut current_time_1: i64 = 0;
    for job in &jobs_1 {
        current_time_1 += job.length;
        total_weighted_time_1 += current_time_1 * job.weight;
    }

    // Ratio Metric (w / l) ---
    let mut jobs_2 = jobs.clone();
    // Cross-multiply to avoid floating point precision issues
    
    jobs_2.sort_unstable_by(|a, b| {
        let ratio_compare_a = a.weight * b.length;
        let ratio_compare_b = b.weight * a.length;
        
        ratio_compare_b.cmp(&ratio_compare_a)
    });

    let mut total_weighted_time_2: i64 = 0;
    let mut current_time_2: i64 = 0;
    for job in &jobs_2 {
        current_time_2 += job.length;
        total_weighted_time_2 += current_time_2 * job.weight;
    }

    println!("Weighted completion time in method 1 is {}", total_weighted_time_1);
    println!("Weighted completion time in method 2 is {}", total_weighted_time_2);
}
