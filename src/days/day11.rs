use std::fs::read_to_string;
use std::cmp::{max, min};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use crate::etc::Coords2D;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<u64>;
type RowColData = (FxHashSet<u64>, FxHashSet<u64>);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11.txt").unwrap();
    let galaxies = find_galaxies(&input);
    let dilations = find_empty(&input);

    let sol1 = calculate_distances(&galaxies, &dilations, 2);
    let sol2 = calculate_distances(&galaxies, &dilations, 1_000_000);
    
    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn calculate_distances(galaxies: &[Pos], data: &RowColData, dilation: u64) -> u64 {
    galaxies.iter().tuple_combinations()
        .map(|(g1, g2)| pair_distance(*g1, *g2, data, dilation))
        .sum()
}

fn pair_distance(g1: Pos, g2: Pos, (rows, cols): &RowColData, dilation: u64) -> u64 {
    let (x1, x2) = (min(g1.x, g2.x), max(g1.x, g2.x));
    let (y1, y2) = (min(g1.y, g2.y), max(g1.y, g2.y));

    let dx = x2 - x1 + (dilation - 1) * (x1..=x2).filter(|x| cols.contains(x)).count() as u64;
    let dy = y2 - y1 + (dilation - 1) * (y1..=y2).filter(|y| rows.contains(y)).count() as u64;

    dx + dy
}

fn find_empty(input: &str) -> RowColData {
    let empty_rows = input.lines().enumerate()
        .filter(|(_, line)| line.chars().all(|x| x == '.'))
        .map(|x| x.0 as u64)
        .collect();

    let line_len = input.lines().next().unwrap().len() as u64;
    let empty_cols = (0..line_len).filter(|i| {
        input.lines().all(|line| line.chars().nth(*i as usize).unwrap() == '.')
    }).collect();

    (empty_rows, empty_cols)
}

fn find_galaxies(input: &str) -> Vec<Pos> {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter(|(_, ch)| *ch == '#').map(move |(x, _)| (x as u64, y as u64).into())
    }).collect()
}
