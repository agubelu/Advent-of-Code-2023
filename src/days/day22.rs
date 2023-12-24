use std::fs::read_to_string;
use std::cmp::min;

use itertools::Itertools;
use petgraph::prelude::{Graph, NodeIndex, Direction, EdgeRef};
use rustc_hash::{FxHashMap, FxHashSet};
use sscanf::{sscanf, FromScanf};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type PosMap = FxHashMap<Pos3D, usize>;
type SupportGraph = Graph<(), ()>;

#[derive(PartialEq, Eq, Hash, FromScanf)]
#[sscanf(format = "{x},{y},{z}")]
struct Pos3D {
    x: i32, y: i32, z: i32
}

#[derive(FromScanf)]
#[sscanf(format = "{start}~{end}")]
struct Block {
    start: Pos3D,
    end: Pos3D
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day22.txt").unwrap();
    let mut blocks = input.lines().map(Block::from_line).collect_vec();
    let mut occupied = FxHashMap::default();
    blocks.sort_by_key(|p| p.start.z);

    simulate_fall(&mut blocks, &mut occupied);
    let graph = build_support_graph(&blocks, &occupied);
    let essentials = essential_blocks(&graph);

    let sol1 = blocks.len() - essentials.len();
    
    let sol2: usize = essentials.into_iter()
        .map(|node| all_dependent_blocks(node, &graph))
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn essential_blocks(graph: &SupportGraph) -> FxHashSet<NodeIndex> {
    let mut res = FxHashSet::default();

    for node in graph.node_indices() {
        let edges = graph.edges_directed(node, Direction::Incoming).collect_vec();
        if edges.len() == 1 {
            res.insert(edges[0].source());
        }
    }
    
    res
}

fn all_dependent_blocks(node: NodeIndex, graph: &SupportGraph) -> usize {
    let mut fallen = FxHashSet::default();
    fallen.insert(node);
    let mut to_visit: FxHashSet<_> = graph.neighbors(node).collect();

    while !to_visit.is_empty() {
        let mut new_to_visit = FxHashSet::default();

        for new_node in to_visit {
            // This node falls if all its supporting nodes have fallen
            let falls = graph.neighbors_directed(new_node, Direction::Incoming)
                .all(|n| fallen.contains(&n));
            if falls {
                fallen.insert(new_node);
                new_to_visit.extend(graph.neighbors(new_node));
            }
        }

        to_visit = new_to_visit;
    }

    fallen.len() - 1
}

fn build_support_graph(blocks: &[Block], occupied: &PosMap) -> SupportGraph {
    let mut id_map = FxHashMap::default();
    let mut graph = SupportGraph::new();

    for (i, block) in blocks.iter().enumerate() {
        let this_ix = *id_map.entry(i).or_insert(graph.add_node(()));
        let supporting_blocks = block.bottom_hitboxes().filter_map(|pos| occupied.get(&pos)).unique();

        for &supp in supporting_blocks {
            let supp_ix = *id_map.entry(supp).or_insert(graph.add_node(()));
            graph.add_edge(supp_ix, this_ix, ());
        }
    }

    graph
}

fn simulate_fall(blocks: &mut [Block], occupied: &mut PosMap) {
    for (i, block) in blocks.iter_mut().enumerate() {
        while block.can_move_down(occupied) {
            block.move_down();
        }

        for pos in block.occupied_spaces() {
            occupied.insert(pos, i);
        }
    }
}

impl Block {
    fn from_line(line: &str) -> Self {
        sscanf!(line, "{Block}").unwrap()
    }

    fn bottom_hitboxes(&self) -> impl Iterator<Item = Pos3D> + '_ {
        let lower_z = min(self.start.z, self.end.z) - 1;

        (self.start.x ..= self.end.x).flat_map(move |x| {
            (self.start.y ..= self.end.y).map(move |y| {
                Pos3D { x, y, z: lower_z }
            })
        })
    }

    fn occupied_spaces(&self) -> impl Iterator<Item = Pos3D> + '_ {
        (self.start.x ..= self.end.x).flat_map(move |x| {
            (self.start.y ..= self.end.y).flat_map(move |y| {
                (self.start.z ..= self.end.z).map(move |z| {
                    Pos3D { x, y, z }
                })
            })
        })
    }

    fn can_move_down(&self, occupied: &PosMap) -> bool {
        self.bottom_hitboxes().all(|pos| pos.z >= 1 && !occupied.contains_key(&pos))
    }

    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }
}
