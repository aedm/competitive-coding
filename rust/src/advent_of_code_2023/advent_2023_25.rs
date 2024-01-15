use crate::utils::read_lines_split;
use rs_graph::maxflow::edmondskarp;
use rs_graph::{Buildable, Builder, VecGraph};
use std::collections::{HashMap, HashSet};

pub fn solve() -> usize {
    let mut g = HashMap::new();
    for v in read_lines_split("advent_2023/25.txt", &[':', ' ']) {
        for t in &v[2..] {
            g.entry(v[0].clone()).or_insert(HashSet::new()).insert(t.clone());
            g.entry(t.clone()).or_insert(HashSet::new()).insert(v[0].clone());
        }
    }
    let indices = g.keys().enumerate().map(|(i, k)| (k.clone(), i)).collect::<HashMap<_, _>>();

    let mut b = VecGraph::<usize>::new_builder();
    let n = b.add_nodes(indices.len());
    for (v1, e) in &g {
        for v2 in e {
            b.add_edge(n[indices[v1]], n[indices[v2]]);
        }
    }
    let graph = b.into_graph();

    // Parsing over, let's solve the problem.
    let c = (1..n.len()).filter(|i| edmondskarp(&graph, n[0], n[*i], |_| 1).0 == 3).count();
    c * (n.len() - c)
}
