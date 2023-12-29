use std::fs::read_to_string;
use std::hash::BuildHasherDefault;
use itertools::Itertools;

use petgraph::visit::{Dfs, Walker};
use petgraph::prelude::{UnGraph, EdgeRef, NodeIndex, Outgoing};
use priority_queue::PriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Graph<'a> = UnGraph<&'a str, u32>;
type IndexMap<'a> = FxHashMap<&'a str, NodeIndex>;
type NodeQueue = PriorityQueue<NodeIndex, u32, BuildHasherDefault<FxHasher>>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day25.txt").unwrap();
    let edges = parse_pairs(&input);
    let (graph, _) = build_graph(edges);

    let sol1 = solve_min_cut(graph);
    let sol2 = "Merry Christmas!";

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

// An implementation of the Stoer-Wagner algorithm to find the minimum cut
// in the graph. In this case, we already know that the minimum cut has
// weight 3, so we immediately stop when we reach it.
fn solve_min_cut(mut g: Graph) -> usize {
    let mut contractions = vec![];
    let nodes = g.node_count();

    loop {
        let (cut, s, t) = minimum_cut_phase(&g, 0.into());
        contractions.push((
            *g.node_weight(s).unwrap(),
            *g.node_weight(t).unwrap()
        ));

        if cut == 3 {
            let comp = uncontract(&contractions);
            break comp * (nodes - comp);
        };

        merge(&mut g, s, t);
    }
}

// Performs an interation of the minimum cut algorithm for a given starting
// node, returning the minimum cut weight and the two nodes to merge after.
fn minimum_cut_phase(g: &Graph, start: NodeIndex) -> (u32, NodeIndex, NodeIndex) {
    let mut visited = FxHashSet::default();
    visited.insert(start);

    let mut q = NodeQueue::with_capacity_and_default_hasher(g.node_count());
    let mut next_best = start; // Will change

    // Populate the priority queue with all neighbors of the start node
    for edge in g.edges_directed(start, Outgoing) {
        q.push(edge.target(), *edge.weight());
    }

    // Add nodes to the visited set until only one (s) remains
    while visited.len() < g.node_count() - 1 {
        next_best = q.pop().unwrap().0;
        visited.insert(next_best);

        // Update the queue with all neighbors of the newly added node
        for edge in g.edges_directed(next_best, Outgoing) {
            let target = edge.target();
            if !visited.contains(&target) {
                let weight = q.get_priority(&target).copied().unwrap_or(0) + edge.weight();
                q.push(target, weight);
            }
        }
    }

    let (s, cut_weight) = q.pop().unwrap();
    (cut_weight, s, next_best)
}

// The results returned by the Stoer-Wagner algorithms contains nodes that are
// contractions of two or more nodes. This function re-builds the graph
// from the performed contractions and calculates the number of nodes
// in the resulting connected component.
fn uncontract(contrs: &[(&str, &str)]) -> usize {
    let components = &contrs[..contrs.len()-1];
    let start = &contrs.last().unwrap().0;

    let (g, ids) = build_graph(components.iter().copied());
    Dfs::new(&g, ids[start]).iter(&g).count()
}

// Merges the node s(ource) into t(arget). Edges between s and t are dropped.
// All edges from s to other nodes are moved to t. If t already has an edge
// to the same node, their weights are combined instead.
fn merge(g: &mut Graph, s: NodeIndex, t: NodeIndex) {
    let edges_merge = g.edges_directed(s, Outgoing)
        .map(|edge| (edge.target(), *edge.weight()))
        .collect_vec();

    for (other, weight) in edges_merge {
        if other != t {
            upsert_edge(g, t, other, weight);
        }
    }

    g.remove_node(s);
}

// Inserts an edge between two nodes with a given weight. If the edge
// already exists, the provided weight is instead added to the existing edge.
fn upsert_edge(g: &mut Graph, a: NodeIndex, b: NodeIndex, mut w: u32) {
    if let Some(edge) = g.edges_connecting(a, b).next() {
        // Edge already exists, update weight
        w += edge.weight();
    }

    g.update_edge(a, b, w);
}

// Builds an undirected graph from a set of edges
fn build_graph<'a, E>(edges: E) -> (Graph<'a>, IndexMap<'a>)
where E: Iterator<Item = (&'a str, &'a str)> {
    let mut ids = FxHashMap::default();
    let mut graph = Graph::new_undirected();

    for (n1, n2) in edges {
        let n1_ix = *ids.entry(n1).or_insert_with(|| graph.add_node(n1));
        let n2_ix = *ids.entry(n2).or_insert_with(|| graph.add_node(n2));
        graph.add_edge(n1_ix, n2_ix, 1);
    }

    (graph, ids)
}

// Input parser
fn parse_pairs(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.lines()
         .flat_map(|line| {
            let (n1, rest) = line.split_once(": ").unwrap();
            rest.split_whitespace().map(move |n2| (n1, n2))
         })
}
