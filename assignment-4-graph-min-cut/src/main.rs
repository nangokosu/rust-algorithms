use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use rand::seq::SliceRandom;

//for random numbers

#[derive(Debug, Clone)]
pub struct Graph{
    pub adj: HashMap<usize, Vec<usize>>,
}

impl Graph{
    fn total_edges(&self) -> usize{
        self.adj.values().map(|edges| edges.len()).sum::<usize>() /2
    }

    fn add_edge(&mut self, u: usize, v: usize){
        self.adj.entry(u).or_default().push(v);
        self.adj.entry(v).or_default().push(u); //needs to add for both vertices

    }

    fn choose_random_edge(&self) -> (usize, usize){
        let mut rng = rand::thread_rng();
        let vertices: Vec<&usize> = self.adj.keys().collect();
        let &u = *vertices.choose(&mut rng).expect("Graph is empty");
        
        // After selecting random 

        let neighbors = &self.adj[&u];
        
        let &v = neighbors.choose(&mut rng).expect("Vertex has no neighbors");

        (u, v)
    }

    // Contracts edge (u,v) by merging node 'v' into node 'u'
    fn contract_edge(&mut self, u: usize, v: usize){
        let v_neighbors = self.adj.remove(&v).expect("Node v not found"); // removes vs vertex, and returns its values
        
        
        if let Some(u_neighbors) = self.adj.get_mut(&u){
            u_neighbors.extend(v_neighbors);
        }

        // going through and replace all references of v to u
        for neighbors in self.adj.values_mut() {
            for neighbor in neighbors.iter_mut() {
                if *neighbor == v {
                    *neighbor = u;
                }
            }
        }

        // removing self loops by essentially removing any reference of u in its own neighbors
        if let Some(u_neighbors) = self.adj.get_mut(&u) {
            u_neighbors.retain(|&neighbor| neighbor != u); 
        }


    }

    fn contract_to_min_cut(&mut self) -> usize {
        while self.adj.len() >2 {
            let (u,v) = self.choose_random_edge();
            self.contract_edge(u,v);
        }
        // The remaining vertices are the "supernodes"
        let remaining_vertices: Vec<&usize> = self.adj.keys().collect();
        let any_key = *remaining_vertices[0];
        self.adj[&any_key].len() // this is the number of min cuts seen for this particular run
    }








}

fn main() {
    let file = File::open("Vertice.txt").expect("Could not read file");
    let buf_reader = BufReader::new(file);

    let mut graph = Graph{
        adj: HashMap::new(),
    };

    // creating the adjacency matrix

    for line in buf_reader.lines(){
        let line_digits: Vec<usize> = line.expect("Failed to read line")
        .split_whitespace()
        .map(|s: &str| s.parse::<usize>()
        .unwrap())
        .collect();
        
        let value: &mut Vec<usize> = graph.adj.entry(line_digits[0]).or_default();
        
        value.extend(&line_digits[1..]);
    };

    let mut min_cut = usize::MAX;

    let n = graph.adj.len();

    let exact_iterations = (n * n * 2); // Not n^2 log n, but close enough for faster running


    for i in 0..exact_iterations{
        let mut clone_graph = graph.clone();
        let cut = clone_graph.contract_to_min_cut();

        if cut < min_cut{
            min_cut = cut
        }

    }

    println!("Min cut is {}", min_cut);



}
