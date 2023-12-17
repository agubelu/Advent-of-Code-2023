use std::fs::read_to_string;

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

use crate::etc::{VecMat, Coords2D};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day16.txt").unwrap();
    let grid = VecMat::from_str(&input);

    let sol1 = project_ray(&grid, Pos::origin(), Pos::right());
    let sol2 = find_best(&grid);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_best(grid: &VecMat<char>) -> usize {
    let (w, h) = (grid.width() as i32, grid.height() as i32);

    let iters = [
        (0..w, 0..1, Pos::down()),
        (0..w, h-1..h, Pos::up()),
        (0..1, 0..h, Pos::right()),
        (w-1..w, 0..h, Pos::left()),
    ];

    // Builds a vec with all starting positions along the borders
    // and their associated start directions
    let starts = iters.into_iter().flat_map(|(iter_x, iter_y, dir)| {
        iter_x.flat_map(move |x| iter_y.clone().map(move |y| (Pos::new(x, y), dir)))
    }).collect_vec();

    starts.into_par_iter().map(|(start, dir)| project_ray(grid, start, dir)).max().unwrap()
}

fn project_ray(grid: &VecMat<char>, start_pos: Pos, start_dir: Pos) -> usize {
    let mut heads = vec![(start_pos, start_dir)];
    let mut visited = FxHashSet::default();

    while !heads.is_empty() {
        let mut new_heads = vec![];

        for (pos, dir) in heads {
            visited.insert((pos, dir));
            // Find the continuation(s) for this ray that remain in-bounds
            // and haven't been visited before
            let conts = next_directions(grid, (pos, dir)).into_iter()
                .map(|new_dir| (pos + new_dir, new_dir))
                .filter(|&(new_pos, new_dir)| grid.is_in_bounds(new_pos) && !visited.contains(&(new_pos, new_dir)));
            new_heads.extend(conts);
        }

        heads = new_heads;
    }

    // The same position may appear more than once with different ray directions
    visited.into_iter().map(|(pos, _)| pos).unique().count()
}

fn next_directions(grid: &VecMat<char>, (current, prev_dir): (Pos, Pos)) -> Vec<Pos> {
    let [up, down, left, right] = Pos::origin().neighbors();

    match (grid[current], prev_dir) {
        ('.', d) => vec![d],
        ('|', d) if d.x == 0 => vec![d],
        ('|', _) => vec![up, down],
        ('-', d) if d.y == 0 => vec![d],
        ('-', _) => vec![left, right],
        ('/', Pos{x: 0, y}) => vec![(-y, 0).into()],
        ('/', Pos{x, ..}) => vec![(0, -x).into()],
        ('\\', Pos{x: 0, y}) => vec![(y, 0).into()],
        ('\\', Pos{x, ..}) => vec![(0, x).into()],
        _ => unreachable!()
    }
}
