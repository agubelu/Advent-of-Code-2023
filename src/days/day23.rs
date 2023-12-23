use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::prelude::{Graph, NodeIndex};
use petgraph::algo::all_simple_paths;
use rustc_hash::{FxHashSet, FxHashMap};

use crate::etc::{VecMat, Coords2D};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;
type GridGraph = Graph<(), u32>;
type IndexMap = FxHashMap<Pos, NodeIndex>;

#[derive(Clone)]
struct ExploreState<'a> {
    last_fork: Pos,
    current: Pos,
    goal: Pos,
    visited: FxHashSet<Pos>,
    len: u32,
    grid: &'a VecMat<char>,
    can_climb: bool,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day23.txt").unwrap();
    let grid = VecMat::from_str(&input);
    
    let sol1 = find_longest_path(&grid, false);
    let sol2 = find_longest_path(&grid, true);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_longest_path(grid: &VecMat<char>, can_climb: bool) -> u32 {
    let (start, end) = find_start_end(grid);
    let (graph, id_map) = build_graph(grid, start, end, can_climb);

    all_simple_paths::<Vec<_>, _>(&graph, id_map[&start], id_map[&end], 0, None)
        .map(|path| path_length(&graph, &path))
        .max().unwrap()
}

fn path_length(graph: &GridGraph, path: &[NodeIndex]) -> u32 {
    path.iter().tuple_windows()
        .map(|(a, b)| graph.edges_connecting(*a, *b).next().unwrap().weight())
        .sum()
}

fn build_graph(grid: &VecMat<char>, start: Pos, end: Pos, can_climb: bool) -> (GridGraph, IndexMap) {
    // Builds a graph representing the maze by exploring it with a DFS
    // and connecting the forks together with edges representing their distances.
    let mut graph = Graph::new();
    let mut id_map = IndexMap::default();

    let mut state = ExploreState {
        last_fork: start,
        current: start,
        goal: end,
        visited: FxHashSet::default(),
        len: 0,
        can_climb,
        grid,
    };

    update_graph(&mut state, &mut graph, &mut id_map);
    (graph, id_map)
}

fn update_graph(state: &mut ExploreState, graph: &mut GridGraph, id_map: &mut IndexMap) {
    // Mark the current position as explored
    state.visited.insert(state.current);

    // If we have reached the end on this branch, update the
    // graph with the corresponding edge and finish
    if state.current == state.goal {
        add_edges(state, id_map, graph);
        return;
    }

    // Find the neighbors of the current position
    let neighbors = neighbors(state);
    
    #[allow(clippy::comparison_chain)]
    if neighbors.len() == 1 {
        // Only one way to go, keep going forward
        state.current = neighbors[0];
        state.len += 1;
        update_graph(state, graph, id_map);
    } else if neighbors.len() > 1 {
        // Fork reached, check if it has been explored before
        let explored = id_map.contains_key(&state.current);

        // Add the edge from the previous fork to this one
        add_edges(state, id_map, graph);

        // Explore recursively only if this fork hasn't been explored before
        if explored { return }

        for neighbor in neighbors {
            let mut new_state = state.clone();
            new_state.last_fork = state.current;
            new_state.current = neighbor;
            new_state.len = 1;
            update_graph(&mut new_state, graph, id_map);
        }

    }
}

fn get_node(id_map: &mut IndexMap, pos: Pos, graph: &mut GridGraph) -> NodeIndex {
    *id_map.entry(pos).or_insert(graph.add_node(()))
}

fn add_edges(state: &ExploreState, id_map: &mut IndexMap, graph: &mut GridGraph) {
    let prev_ix = get_node(id_map, state.last_fork, graph);
    let current_ix = get_node(id_map, state.current, graph);
    graph.add_edge(prev_ix, current_ix, state.len);
    
    if state.can_climb {
        // If part 2, we need the graph to be undirected in practice
        graph.add_edge(current_ix, prev_ix, state.len);
    }
}

fn neighbors(state: &mut ExploreState) -> Vec<Pos> {
    let ch = state.grid[state.current];
    let neighbors = if ch == '.' || state.can_climb {
        state.current.neighbors().to_vec()
    } else {
        vec![match ch {
            '>' => state.current.go_right(),
            '<' => state.current.go_left(),
            '^' => state.current.go_up(),
             _  => state.current.go_down(),
        }]
    };

    neighbors.into_iter()
        .filter(|new| state.grid.get_or(*new, '#') != '#' && !state.visited.contains(new))
        .collect_vec()
}

fn find_start_end(grid: &VecMat<char>) -> (Pos, Pos) {
    let start_y = 0;
    let end_y = grid.height() - 1;

    let start_x = grid.get_row(start_y).into_iter().position(|c| c == '.').unwrap() as i32;
    let end_x = grid.get_row(end_y).into_iter().position(|c| c == '.').unwrap() as i32;
    (Pos::new(start_x, start_y as i32), Pos::new(end_x, end_y as i32))
}
