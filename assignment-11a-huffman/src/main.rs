use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;
use std::collections::{BinaryHeap};

#[derive(Eq, PartialEq)]
struct Node {
    weight: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Custom ordering to turn Rust's default Max-Heap into a Min-Heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// --- 2. CORE ALGORITHM ---

fn build_huffman_tree(weights: Vec<i64>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    // Initialize the Min-Heap with leaf nodes
    for weight in weights {
        heap.push(Box::new(Node {
            weight,
            left: None,
            right: None,
        }));
    }

    // Combine the two lowest-weight nodes until only the root remains
    while heap.len() > 1 {
        let node1 = heap.pop().unwrap();
        let node2 = heap.pop().unwrap();

        let parent = Box::new(Node {
            weight: node1.weight + node2.weight,
            left: Some(node1),
            right: Some(node2),
        });

        heap.push(parent);
    }

    heap.pop() // returns root
}

// --- 3. TREE TRAVERSAL ---

fn find_leaf_depths(node: &Option<Box<Node>>, current_depth: u32, depths: &mut Vec<u32>) {
    if let Some(n) = node {
        // If it's a leaf node (no children), record its depth
        if n.left.is_none() && n.right.is_none() {
            depths.push(current_depth);
            return;
        }

        // Otherwise, recursively check children, increasing the depth counter
        find_leaf_depths(&n.left, current_depth + 1, depths);
        find_leaf_depths(&n.right, current_depth + 1, depths);
    }
}



fn main() {
    let path = "huffman.txt"; 

    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file).lines();

    // The first line of Coursera's file contains the total number of symbols
    let num_symbols: usize = lines
        .next()
        .unwrap().unwrap()
        .trim()
        .parse()
        .expect("Failed to parse the number of symbols");

    let mut weights: Vec<i64> = Vec::with_capacity(num_symbols);

    // Parse all subsequent lines as integer weights
    for line in lines {
        let line_unwrap: String = line.unwrap();
        let ip: i64 = line_unwrap.trim().parse::<i64>().unwrap();
        weights.push(ip);}
        
    

    println!("Loaded {} weights from file.", weights.len());

    // Run the algorithm
    if let Some(root) = build_huffman_tree(weights) {
        let mut depths = Vec::new();
        find_leaf_depths(&Some(root), 0, &mut depths);

        if let (Some(&max_depth), Some(&min_depth)) = (depths.iter().max(), depths.iter().min()) {
           
            println!("Maximum codeword length: {}", max_depth);
            println!("Minimum codeword length: {}", min_depth);

        }
    } else {
        println!("The tree could not be constructed.");
    }


   







    





}
