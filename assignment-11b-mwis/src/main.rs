use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // 1. Load the input data
    // The file format: line 1 is number of vertices, subsequent lines are weights.
    let file = File::open("mwis.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let num_vertices: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Pad with 0 at index 0 to use 1-based indexing for vertices
    let mut weights: Vec<i64> = Vec::with_capacity(num_vertices + 1);
    weights.push(0); 

    for line in lines {
        let line_unwrap: String = line.unwrap();
        let weight: i64 = line_unwrap.parse().unwrap();
        weights.push(weight);
    }

    // 2. Phase 1: Dynamic Programming (Forward Pass)
    let mut a = vec![0i64; num_vertices + 1];
    a[0] = 0;
    a[1] = weights[1];

    for i in 2..=num_vertices {
        // Recurrence relation: max(exclude current, include current + optimal from i-2)
        a[i] = std::cmp::max(a[i - 1], a[i - 2] + weights[i]);
    }

    // 3. Phase 2: Reconstruction (Backward Pass)
    let mut mwis_vertices = HashSet::new();
    let mut i = num_vertices;

    while i >= 2 {
        if a[i - 1] >= a[i - 2] + weights[i] {
            // Case 1: Vertex i was not included
            i -= 1;
        } else {
            // Case 2: Vertex i was included
            mwis_vertices.insert(i);
            i -= 2; // Skip adjacent vertex i-1
        }
    }
    
    // Handle the remaining edge case base step
    if i == 1 {
        mwis_vertices.insert(1);
    }

    
    // Target vertices 
    let targets = [1, 2, 3, 4, 17, 117, 517, 997];
    let mut result_string = String::new();

    for target in &targets {
        if mwis_vertices.contains(target) {
            result_string.push('1');
        } else {
            result_string.push('0');
        }
    }

    println!("Max Weight: {}", a[num_vertices]);
    println!("Coursera Grader String: {}", result_string);

}
