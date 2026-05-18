use std::f64;
use std::fs::File;
use std::io::{BufReader, BufRead};

// Structure to hold coordinates as specified in the assignment text
#[derive(Debug, Clone, Copy)]
struct City {
    x: f64,
    y: f64,
}

/// Helper function to compute the standard Euclidean distance between two cities
fn euclidean_distance(c1: &City, c2: &City) -> f64 {
    let dx = c1.x - c2.x;
    let dy = c1.y - c2.y;
    (dx * dx + dy * dy).sqrt()
}

/// Computes the minimum cost to complete a TSP tour using a flattened DP table
fn tsp_dynamic_programming(cities: &[City]) -> f64 {
    let n = cities.len();
    if n <= 1 {
        return 0.0;
    }

    // 1. Calculate the Euclidean distance matrix
    let mut dist = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            dist[i][j] = euclidean_distance(&cities[i], &cities[j]);
        }
    }

    // 2. Setup the Dynamic Programming state table
    let num_states = 1 << n; // 2^n
    let mut dp = vec![f64::INFINITY; num_states * n];

    // Base Case: Start at city 0 (represented by bitmask 1)
    dp[(1 << 0) * n + 0] = 0.0;

    // 3. Tabulation
    for mask in 1..num_states {
        // Optimization: Skip subsets that don't include the starting city (city 0)
        if (mask & 1) == 0 {
            continue;
        }

        for u in 0..n {
            if (mask & (1 << u)) == 0 {
                continue;
            }

            let current_dist = dp[mask * n + u];
            if current_dist == f64::INFINITY {
                continue;
            }

            // Transition to an unvisited city `v`
            for v in 0..n {
                if (mask & (1 << v)) == 0 {
                    let next_mask = mask | (1 << v);
                    let next_dist = current_dist + dist[u][v];
                    
                    let target_idx = next_mask * n + v;
                    if next_dist < dp[target_idx] {
                        dp[target_idx] = next_dist;
                    }
                }
            }
        }
    }

    // 4. Return to the start city (0) from the completed tour mask
    let full_mask = num_states - 1;
    let mut min_tour_cost = f64::INFINITY;

    for u in 1..n {
        let tour_cost = dp[full_mask * n + u] + dist[u][0];
        if tour_cost < min_tour_cost {
            min_tour_cost = tour_cost;
        }
    }

    min_tour_cost
}

fn main() {
    // Mimicking the structure of `tsp.txt`
    // First line of the file indicates the number of cities (e.g., 4)
    // Followed by lines containing raw x and y float coordinates
    let file = File::open("TSP.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let num_cities: usize = lines.next().unwrap().unwrap().parse().unwrap();

    
    let mut cities = Vec::with_capacity(num_cities);

    for line in lines {
        let line_unwrap = line.unwrap();
        let mut line_coords = line_unwrap.split_whitespace();
        let x = line_coords.next().unwrap().parse().unwrap();
        let y = line_coords.next().unwrap().parse().unwrap();
        
        cities.push(City { x: x, y: y});
        }
    

    // Run the algorithm
    let exact_cost = tsp_dynamic_programming(&cities);
    
    //  rounding DOWN to the nearest integer
    let final_answer = exact_cost.floor() as i32;

    println!("Exact calculated cost: {}", exact_cost);
    println!("Expected calculated_cost (rounded down): {}", final_answer);
}



