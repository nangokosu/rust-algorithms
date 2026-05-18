use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Represents a directed edge in the graph for the Bellman-Ford pass.
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

/// A structure to hold elements inside our priority queue for Dijkstra.
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

/// We implement custom ordering to turn Rust's default Max-Heap into a Min-Heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Parses the assignment files. Converts 1-indexed file nodes into 0-indexed structures.
fn parse_graph(filename: &str) -> io::Result<(usize, Vec<Edge>, Vec<Vec<(usize, i32)>>)> {
    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    // Read the header line containing metadata
    let first_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Empty graph file"))??;
    let tokens: Vec<&str> = first_line.split_whitespace().collect();
    if tokens.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Malformed header"));
    }
    
    let num_vertices: usize = tokens[0].parse().unwrap();
    let num_edges: usize = tokens[1].parse().unwrap();

    let mut edge_list = Vec::with_capacity(num_edges);
    let mut adj_list = vec![Vec::new(); num_vertices];

    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 { continue; }
        
        let from: usize = parts[0].parse::<usize>().unwrap() - 1; // 1-indexed to 0-indexed
        let to: usize = parts[1].parse::<usize>().unwrap() - 1;
        let weight: i32 = parts[2].parse().unwrap();

        edge_list.push(Edge { from, to, weight });
        adj_list[from].push((to, weight));
    }

    Ok((num_vertices, edge_list, adj_list))
}

/// Runs Bellman-Ford on a virtual ghost source node.
/// Returns vertex prices `h` if stable, or `None` if a negative cycle is detected.
fn bellman_ford_reweight(num_vertices: usize, edges: &[Edge]) -> Option<Vec<i32>> {
    // 1. Explicitly create the virtual ghost source (Node index = num_vertices)
    // It can reach every node with an edge weight of 0.
    let ghost_source = num_vertices;
    let mut h = vec![i32::MAX; num_vertices + 1];
    h[ghost_source] = 0;

    // Run N rounds (since total vertices = num_vertices + 1)
    // Round 1 initializes all real nodes to 0 because ghost_source (0) + edge (0) = 0
    for _ in 0..num_vertices {
        let mut changed = false;
        
        // Relax the virtual edges implicitly
        for v in 0..num_vertices {
            if h[ghost_source] != i32::MAX && h[ghost_source] + 0 < h[v] {
                h[v] = 0;
                changed = true;
            }
        }

        // Relax all real edges in the graph
        for edge in edges {
            if h[edge.from] != i32::MAX { // Crucial guard: prevent infinity overflow
                // Safe check against overflow before adding
                if let Some(new_dist) = h[edge.from].checked_add(edge.weight) {
                    if new_dist < h[edge.to] {
                        h[edge.to] = new_dist;
                        changed = true;
                    }
                }
            }
        }
        if !changed { break; } 
    }

    // N-th round validation check for negative cycles
    for edge in edges {
        if h[edge.from] != i32::MAX {
            if let Some(new_dist) = h[edge.from].checked_add(edge.weight) {
                if new_dist < h[edge.to] {
                    return None; // Real negative cycle confirmed!
                }
            }
        }
    }

    // Pop off the ghost source value and just return the real node prices
    h.truncate(num_vertices);
    Some(h)
}

/// Executes single-source Dijkstra on a safely reweighted, non-negative graph state.
fn dijkstra(
    source: usize, 
    num_vertices: usize, 
    adj_list: &[Vec<(usize, i32)>], 
    h: &[i32]
) -> Vec<i32> {
    let mut dist = vec![i32::MAX; num_vertices];
    let mut heap = BinaryHeap::new();

    dist[source] = 0;
    heap.push(State { cost: 0, position: source });

    while let Some(State { cost, position }) = heap.pop() {
        // If we found a shorter path to this node already, bypass stale data
        if cost > dist[position] { continue; }

        for &(neighbor, raw_weight) in &adj_list[position] {
            // Apply Johnson's reweighting: w' = w + h(u) - h(v)
            let reweighted_weight = raw_weight + h[position] - h[neighbor];
            let next_dist = cost + reweighted_weight;

            if next_dist < dist[neighbor] {
                dist[neighbor] = next_dist;
                heap.push(State { cost: next_dist, position: neighbor });
            }
        }
    }

    // Convert distances back to original, true graph metrics
    for v in 0..num_vertices {
        if dist[v] != i32::MAX {
            dist[v] = dist[v] - h[source] + h[v];
        }
    }

    dist
}

/// Executes the core orchestration logic for each individual file.
fn solve_assignment(filename: &str) {
    println!("Processing {}...", filename);
    
    let (num_vertices, edge_list, adj_list) = match parse_graph(filename) {
        Ok(data) => data,
        Err(e) => {
            println!("Error opening or reading file {}: {}", filename, e);
            return;
        }
    };

    // Run preliminary Bellman-Ford validation pass
    let h = match bellman_ford_reweight(num_vertices, &edge_list) {
        Some(prices) => prices,
        None => {
            println!("==> RESULT: NULL (Graph contains a negative cycle)\n");
            return;
        }
    };

    // Graph is proven safe and reweighted! Proceed to run Dijkstra N times sequentially
    let mut global_min = i32::MAX;

    for source in 0..num_vertices {
        let distances = dijkstra(source, num_vertices, &adj_list, &h);
        for d in distances {
            // Make sure we ignore unreached infinity markers when tracking minimum paths
            if d != i32::MAX && d < global_min {
                global_min = d;
            }
        }
    }

    println!("==> RESULT: Shortest of all shortest paths is: {}\n", global_min);
}

fn main() {
    let files = ["g1.txt", "g2.txt", "g3.txt"];
    for file in &files {
        solve_assignment(file);
    }
}
