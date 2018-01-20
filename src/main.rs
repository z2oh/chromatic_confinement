extern crate petgraph;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot, Config};

fn main() {
    let mut colors = vec![
        vec![128u8, 128u8, 128u8],
        vec![127u8, 0u8, 10u8],
        vec![150u8, 250u8, 250u8],
        vec![150u8, 150u8, 150u8],
        vec![16u8, 32u8, 48u8],
        vec![64u8, 33u8, 0u8],
        vec![200u8, 200u8, 200u8],
        vec![64u8, 31u8, 5u8],
        vec![0u8, 0u8, 0u8],
    ];

    let kd_tree = construct_kd_tree(colors, 3);
    println!("{:?}", Dot::with_config(&kd_tree, &[]));
}

fn construct_kd_tree(mut v: Vec<Vec<u8>>, dimension: u8) -> Graph<Vec<u8>, u8> {
    let mut graph = Graph::<Vec<u8>, u8>::new();

    let root = construct_kd_tree_recursive(&mut v[..], &mut graph, 0, dimension);

    return graph;
}

fn construct_kd_tree_recursive(v: &mut [Vec<u8>], mut graph: &mut Graph<Vec<u8>, u8>, current_dim: u8, max_dim: u8) -> NodeIndex {
    if v.len() == 1 {
        return graph.add_node(v[0].clone());
    }
    else if v.len() == 2 {
        v.sort_unstable_by_key(|k| k[current_dim as usize]);
        let parent_idx = graph.add_node(v[1].clone());
        let child_idx = graph.add_node(v[0].clone());
        graph.add_edge(parent_idx, child_idx, 0);
        return parent_idx;
    }

    v.sort_unstable_by_key(|k| k[current_dim as usize]);
    let middle = v.len() / 2;
    let middle_idx = graph.add_node(v[middle].clone());

    let next_dim = (current_dim + 1) % max_dim;

    let left_idx = construct_kd_tree_recursive(&mut v[..middle], graph, next_dim, max_dim);
    let right_idx = construct_kd_tree_recursive(&mut v[middle+1..], graph, next_dim, max_dim);

    graph.add_edge(middle_idx, left_idx, 0);
    graph.add_edge(middle_idx, right_idx, 1);

    return middle_idx;
}
