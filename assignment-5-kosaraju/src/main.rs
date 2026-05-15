use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;




pub struct Graph {
    pub vertices: usize,
    adj_list: Vec<Vec<usize>>,
    transpose_adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adj_list: vec![vec![]; vertices],
            transpose_adj_list: vec![vec![]; vertices],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.transpose_adj_list[v].push(u);
    }

    pub fn dfs(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
        visited[node] = true;
        for &neighbor in &self.adj_list[node] {
            if !visited[neighbor] {
                self.dfs(neighbor, visited, stack);
            }
        }
        stack.push(node);
    }

    pub fn dfs_scc(&self, node: usize, visited: &mut Vec<bool>, scc: &mut Vec<usize>) {
        visited[node] = true;
        scc.push(node);
        for &neighbor in &self.transpose_adj_list[node] {
            if !visited[neighbor] {
                self.dfs_scc(neighbor, visited, scc);
            }
        }
    }
}

pub fn kosaraju(graph: &Graph) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.vertices];
    let mut stack = Vec::new();

    for i in 0..graph.vertices {
        if !visited[i] {
            graph.dfs(i, &mut visited, &mut stack);
        }
    }

    let mut sccs = Vec::new();
    visited = vec![false; graph.vertices];

    while let Some(node) = stack.pop() {
        if !visited[node] {
            let mut scc = Vec::new();
            graph.dfs_scc(node, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }

    sccs
}

fn main() {
    let file = File::open("KosarajuFile.txt").expect("Failed to open file");
    let buf_file = BufReader::new(file);

    let mut graph = Graph::new(875714 + 1);

    for line in buf_file.lines(){
        let line_digits: Vec<usize> = line.expect("Failed to read line")
        .split_whitespace()
        .map(|s: &str| s.parse::<usize>()
        .unwrap())
        .collect();

        graph.add_edge(line_digits[0], line_digits[1])


    }


    let sccs = kosaraju(&graph);

    let mut sccs_size: Vec<usize> = sccs.iter().map(|scc| scc.len() as usize).collect();

    sccs_size.sort_unstable_by(|a, b| b.cmp(a));

    for size in sccs_size.iter().take(5) {
        println!("{}", size);}


    
}
