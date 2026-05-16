use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn add_edge<V: Ord + Copy, E: Ord + Add + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_default().insert(v2, c);
    graph.entry(v2).or_default().insert(v1, c);
}

// selects a start and run the algorithm from it
pub fn prim<V: Ord + Copy + std::fmt::Debug, E: Ord + Add + Copy + std::fmt::Debug>(
    graph: &Graph<V, E>,
) -> Graph<V, E> {
    match graph.keys().next() {
        Some(v) => prim_with_start(graph, *v),
        None => BTreeMap::new(),
    }
}

// only works for a connected graph
// if the given graph is not connected it will return the MST of the connected subgraph
pub fn prim_with_start<V: Ord + Copy, E: Ord + Add + Copy>(
    graph: &Graph<V, E>,
    start: V,
) -> Graph<V, E> {
    // will contain the MST
    let mut mst: Graph<V, E> = Graph::new();
    // a priority queue based on a binary heap, used to get the cheapest edge
    // the elements are an edge: the cost, destination and source
    let mut prio = BinaryHeap::new();

    mst.insert(start, BTreeMap::new());

    for (v, c) in &graph[&start] {
        // the heap is a max heap, we have to use Reverse when adding to simulate a min heap
        prio.push(Reverse((*c, v, start)));
    }

    while let Some(Reverse((dist, t, prev))) = prio.pop() {
        // the destination of the edge has already been seen
        if mst.contains_key(t) {
            continue;
        }

        // the destination is a new vertex
        add_edge(&mut mst, prev, *t, dist);

        for (v, c) in &graph[t] {
            if !mst.contains_key(v) {
                prio.push(Reverse((*c, v, *t)));
            }
        }
    }

    mst
}


fn calculate_mst_cost<V,E>(mst: &Graph<V,E>) -> E 
where E: std::ops::Add<Output = E> + std::ops::Div<Output = E> + Default + Copy + From<u8>,
{
    let total_doubled: E = mst
        .values()                     // Get the inner BTreeMaps
        .flat_map(|edges| edges.values()) // Flatten into an iterator of weights (&E)
        .copied()                     // Dereference the weights
        .fold(E::default(), |acc, x| acc + x);

    total_doubled / E::from(2)
}

fn main() {
    let file = File::open("Prim.txt").unwrap();
    let mut buf_file = BufReader::new(file).lines();

    let first_line: Vec<usize> = buf_file.next().unwrap().ok().expect("Failed to read first line")
    .split_whitespace().map(|s: &str| s.parse::<usize>().unwrap()).collect();

    let number_of_nodes = first_line[0];

    //let number_of_edges = first_line[1];

    let mut graph = Graph::new();

    for line in buf_file{
        let line_unwrap = line.unwrap();
        let line_elements: Vec<i64> = line_unwrap.split_whitespace().map(|s: &str| s.parse::<i64>().unwrap()).collect();
        add_edge(&mut graph,line_elements[0], line_elements[1], line_elements[2]);
    }

    println!("Number of vertices is {} versus {}", graph.len(), number_of_nodes);

    let result = prim(&graph);

    let cost = calculate_mst_cost(&result);

    println!("MST cost is {}", cost);






    
}
