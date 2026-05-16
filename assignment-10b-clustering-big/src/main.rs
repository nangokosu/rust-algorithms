use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// A standard, efficient Union-Find (Disjoint Set Union) structure
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            count: size,
        }
    }

    /// Finds the representative root of the set containing `i` with path compression
    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        
        // Path compression step
        let mut curr = i;
        while curr != root {
            let nxt = self.parent[curr];
            self.parent[curr] = root;
            curr = nxt;
        }
        root
    }

    /// Merges the sets containing `i` and `j`. Decrements total count if a merge occurs.
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
            self.count -= 1;
            return true;
        }
        false
    }
}

fn main() {
    // 1. Open and stream the file efficiently using a buffered reader
    let file = File::open("clusteringbig.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    // Parse the metadata header (e.g., "200000 24")
    let header = lines.next().unwrap().unwrap();
    let mut header_parts = header.split_whitespace();
    let num_nodes: usize = header_parts.next().unwrap().parse().unwrap();
    let num_bits: u32 = header_parts.next().unwrap().parse().unwrap();

    // 2. Pre-allocate collections to prevent resizing delays
    let mut uf = UnionFind::new(num_nodes);
    let mut node_map: HashMap<u32, usize> = HashMap::with_capacity(num_nodes);

    println!("Parsing data file and compressing distance-0 duplicates...");

    for (idx, line_result) in lines.enumerate() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        // Convert the string representation into a packed u32 integer
        let mut bit_val: u32 = 0;
        for (bit_idx, bit_str) in line.split_whitespace().enumerate() {
            if bit_str == "1" {
                bit_val |= 1 << bit_idx;
            }
        }

        // Distance 0 handling: If we've already seen this exact bitstring,
        // we instantly union them together to collapse the duplicate cluster.
        if let Some(&existing_idx) = node_map.get(&bit_val) {
            uf.union(idx, existing_idx);
        } else {
            node_map.insert(bit_val, idx);
        }
    }

    println!("Processing distance-1 and distance-2 bitwise lookups...");

    // 3. Iterate through unique nodes and calculate virtual edges via XOR bit masking
    for (&val, &idx) in &node_map {
        
        for i in 0..num_bits {
            let mask1 = 1 << i;
            let target_dist1 = val ^ mask1;

            // Check Hamming Distance 1
            if let Some(&target_idx) = node_map.get(&target_dist1) {
                uf.union(idx, target_idx);
            }

            // Check Hamming Distance 2 (using a nested loop avoiding symmetric pairs)
            for j in (i + 1)..num_bits {
                let mask2 = 1 << j;
                let target_dist2 = val ^ mask1 ^ mask2;

                if let Some(&target_idx) = node_map.get(&target_dist2) {
                    uf.union(idx, target_idx);
                }
            }
        }
    }

    
    println!("Largest number of clusters with spacing >= 3: {}", uf.count);
    
}



