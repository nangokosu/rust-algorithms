use std::fs::File;
use std::io::{BufRead, BufReader};

// An Edge struct to hold graph data
#[derive(Debug, Clone, Eq, PartialEq)]
struct Edge {
    u: usize,
    v: usize,
    cost: i32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize, // Tracks the current number of independent clusters
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            count: size,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        let mut curr = i;
        while curr != root {
            let nxt = self.parent[curr];
            self.parent[curr] = root;
            curr = nxt;
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            match self.rank[root_i].cmp(&self.rank[root_j]) {
                std::cmp::Ordering::Less => self.parent[root_i] = root_j,
                std::cmp::Ordering::Greater => self.parent[root_j] = root_i,
                std::cmp::Ordering::Equal => {
                    self.parent[root_j] = root_i;
                    self.rank[root_i] += 1;
                }
            }
            self.count -= 1; // Successfully merged two clusters
            return true;
        }
        false
    }
}

fn main() {
    
    let file = File::open("clustering.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

   
    let num_nodes: usize = lines.next().unwrap().ok().unwrap().trim().parse().unwrap();

    
    let mut edges = Vec::new();
    for line_result in lines {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        let cost: i32 = parts.next().unwrap().parse().unwrap();
        
        
        edges.push(Edge { u: u - 1, v: v - 1, cost });
    }

    // 3. Sort edges by cost in ascending order
    edges.sort_by_key(|e| e.cost);

    // Initialize Union-Find structure with the exact number of nodes
    let mut uf = UnionFind::new(num_nodes);
    let target_clusters = 4;

    let mut edge_iter = edges.iter();

    // Cluster until we have exactly 4 components remaining
    while uf.count > target_clusters {
        if let Some(edge) = edge_iter.next() {
            uf.union(edge.u, edge.v);
        } else {
            break; // Ran out of edges unexpectedly
        }
    }

    // 5. Find the maximum spacing
    // The spacing is the cost of the closest pair of nodes in *distinct* clusters.
    // Since our edges are sorted, we just find the next edge crossing separate components.
    let mut max_spacing = 0;
    for edge in edge_iter {
        if uf.find(edge.u) != uf.find(edge.v) {
            max_spacing = edge.cost;
            break;
        }
    }

    
    println!("The maximum spacing of a 4-clustering is: {}", max_spacing);
    


}



