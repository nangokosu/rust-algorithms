use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

// performs Dijsktra's algorithm on the given graph from the given start
// the graph is a positively-weighted directed graph
//
// returns a map that for each reachable vertex associates the distance and the predecessor
// since the start has no predecessor but is reachable, map[start] will be None
//
// Time: O(E * logV). For each vertex, we traverse each edge, resulting in O(E). For each edge, we
// insert a new shortest path for a vertex into the tree, resulting in O(E * logV).
// Space: O(V). The tree holds up to V vertices.
pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BTreeSet::new();

    // start is the special case that doesn't have a predecessor
    ans.insert(start, None);

    for (new, weight) in &graph[&start] {
        ans.insert(*new, Some((start, *weight)));
        prio.insert((*weight, *new));
    }

    while let Some((path_weight, vertex)) = prio.pop_first() {
        for (next, weight) in &graph[&vertex] {
            let new_weight = path_weight + *weight;
            match ans.get(next) {
                // if ans[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if new_weight >= *dist_next => {}
                // if ans[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                Some(None) => {}
                // the new path is shorter, either new was not in ans or it was farther
                _ => {
                    if let Some(Some((_, prev_weight))) =
                        ans.insert(*next, Some((vertex, new_weight)))
                    {
                        prio.remove(&(prev_weight, *next));
                    }
                    prio.insert((new_weight, *next));
                }
            }
        }
    }

    ans
}


fn main() {
    let file = File::open("djikdata.txt").expect("Failed to open file");
    let buf_file = BufReader::new(file);

    let mut graph = BTreeMap::new();


    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
        graph.entry(v2).or_default();
    }


    for line in buf_file.lines(){
        let line_result: String = line.expect("Failed to read line");

        let line_node: usize = line_result.split_whitespace().find(|s: &&str| !s.contains(",")).unwrap().parse::<usize>().unwrap();

        let line_tuples: Vec<(usize, usize)> = line_result.split_whitespace().filter(|s: &&str| s.contains(","))
        .map(|s| {
            let mut parts = s.split(","); // returns an iterator
            let first = parts.next().expect("Missing first element in tuple").parse::<usize>().unwrap();
            let second = parts.next().expect("Missing second element in tuple").parse::<usize>().unwrap();

            (first, second)
        }
        ).collect();

        for (v2, edge_weight) in line_tuples{
            add_edge(&mut graph, line_node, v2, edge_weight);
        }  
    }

    println!("Original graph: {}", graph.len());


    let dists = dijkstra(&graph, 1);

    println!("returned dist graph: {}", dists.len());

    let arr: &[usize] = &[7,37,59,82,99,115,133,165,188,197]; // reachable vertices

    for i in arr{
        match dists[i]{
            Some(tuple) => println!("Distance to node {} is {}", i, tuple.1),
            None => return,
        }
    }

}
