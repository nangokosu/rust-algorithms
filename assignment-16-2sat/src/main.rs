use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

/// Map a Stanford literal integer (e.g. -16808 or 75250) to a unique usize index.
/// 1 -> 0, -1 -> 1, 2 -> 2, -2 -> 3, etc.
#[inline(always)]
fn literal_to_index(lit: i32) -> usize {
    if lit > 0 {
        ((lit - 1) * 2) as usize
    } else {
        (((-lit - 1) * 2) + 1) as usize
    }
}

/// Invert the index to represent its negation.
/// Uses bitwise XOR to quickly swap between 0 <-> 1, 2 <-> 3, etc.
#[inline(always)]
fn negate_index(idx: usize) -> usize {
    idx ^ 1
}

struct TwoSat {
    num_vars: usize,
    adj: Vec<Vec<usize>>,
    adj_rev: Vec<Vec<usize>>,
}

impl TwoSat {
    fn new(num_vars: usize) -> Self {
        let num_nodes = num_vars * 2;
        Self {
            num_vars,
            adj: vec![vec![]; num_nodes],
            adj_rev: vec![vec![]; num_nodes],
        }
    }

    fn add_clause(&mut self, u: i32, v: i32) {
        let u_idx = literal_to_index(u);
        let v_idx = literal_to_index(v);

        // Implication 1: ¬u -> v
        self.adj[negate_index(u_idx)].push(v_idx);
        self.adj_rev[v_idx].push(negate_index(u_idx));

        // Implication 2: ¬v -> u
        self.adj[negate_index(v_idx)].push(u_idx);
        self.adj_rev[u_idx].push(negate_index(v_idx));
    }

    /// Solves the 2-SAT instance using an explicit stack-based implementation of Kosaraju's algorithm.
    fn is_satisfiable(&self) -> bool {
        let num_nodes = self.num_vars * 2;
        
        // --- PASS 1: Generate Finishing Times via Iterative Post-Order DFS ---
        let mut order = Vec::with_capacity(num_nodes);
        let mut visited = vec![false; num_nodes];
        
        // State tracking array for tracking our position in the neighbor lists
        let mut edge_idx = vec![0; num_nodes];
        let mut stack = Vec::with_capacity(num_nodes);

        for i in 0..num_nodes {
            if visited[i] {
                continue;
            }
            
            stack.push(i);
            visited[i] = true;

            while let Some(&u) = stack.last() {
                let neighbors = &self.adj[u];
                let idx = edge_idx[u];

                if idx < neighbors.len() {
                    let v = neighbors[idx];
                    edge_idx[u] += 1;
                    if !visited[v] {
                        visited[v] = true;
                        stack.push(v);
                    }
                } else {
                    // All neighbors explored; node finished
                    order.push(u);
                    stack.pop();
                }
            }
        }

        // --- PASS 2: Assign SCC IDs using the Reversed Graph ---
        let mut visited = vec![false; num_nodes];
        let mut scc_id = vec![0; num_nodes];
        let mut current_scc = 0;

        // Reset our state trackers for reuse
        edge_idx.fill(0);

        for &start_node in order.iter().rev() {
            if visited[start_node] {
                continue;
            }

            stack.push(start_node);
            visited[start_node] = true;
            scc_id[start_node] = current_scc;

            while let Some(&u) = stack.last() {
                let neighbors = &self.adj_rev[u];
                let idx = edge_idx[u];

                if idx < neighbors.len() {
                    let v = neighbors[idx];
                    edge_idx[u] += 1;
                    if !visited[v] {
                        visited[v] = true;
                        scc_id[v] = current_scc;
                        stack.push(v);
                    }
                } else {
                    stack.pop();
                }
            }
            current_scc += 1;
        }

        // --- FINAL CHECK: Look for contradictions ---
        // Verify that x and ¬x do not share the exact same SCC index
        for i in (0..num_nodes).step_by(2) {
            if scc_id[i] == scc_id[i + 1] {
                return false; 
            }
        }

        true
    }
}

fn process_file(filename: &str) -> Result<bool, std::io::Error> {
    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    // Read metadata from the first line
    let first_line = lines.next().unwrap()?;
    let num_vars: usize = first_line.trim().parse().unwrap();

    let mut solver = TwoSat::new(num_vars);

    // Efficiently stream and parse the remaining lines
    for line in lines {
        let line_str = line?;
        let mut tokens = line_str.split_whitespace();
        if let (Some(u_str), Some(v_str)) = (tokens.next(), tokens.next()) {
            let u: i32 = u_str.parse().unwrap();
            let v: i32 = v_str.parse().unwrap();
            solver.add_clause(u, v);
        }
    }

    Ok(solver.is_satisfiable())
}

fn main() {
    let files = ["2sat1.txt", "2sat2.txt", "2sat3.txt", "2sat4.txt", "2sat5.txt", "2sat6.txt"];
    let mut final_result_string = String::new();

    println!("Starting 2-SAT Evaluation across Stanford datasets...");
    let total_start = Instant::now();

    for file in &files {
        let start = Instant::now();
        match process_file(file) {
            Ok(is_sat) => {
                let bit = if is_sat { "1" } else { "0" };
                final_result_string.push_str(bit);
                println!("File: {} -> Satisfiable: {} (Parsed & Solved in {:?}", file, is_sat, start.elapsed());
            }
            Err(e) => {
                println!("Failed to read {}! Ensure it is placed in the local folder. Error: {}", file, e);
                return;
            }
        }
    }

    println!("--------------------------------------------------");
    println!("Final Submission Code: {}", final_result_string);
    println!("Total execution time: {:?}", total_start.elapsed());
}
