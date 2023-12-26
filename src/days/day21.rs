use std::fs::read_to_string;

use eqsolver::multivariable::MultiVarNewton;
use itertools::Itertools;
use nalgebra::{Vector3, Matrix3};
use rustc_hash::FxHashSet;

use crate::etc::{Coords2D, VecMat};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const PART2_STEPS: f64 = 26_501_365.;
type Pos = Coords2D<i32>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day21.txt").unwrap();
    let mut grid = VecMat::from_str(&input);
    let start = find_replace_start(&mut grid);

    let reachable_in_steps = do_steps(&grid, 327, start);

    let sol1 = reachable_in_steps[64];

    let (a, b, c) = find_quadratic_terms(&reachable_in_steps);
    let sol2 = (a * PART2_STEPS.powi(2) + b * PART2_STEPS + c) as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_quadratic_terms(reachable: &[u64]) -> (f64, f64, f64) {
    // Semi-general solver for part 2. It's built upon the magical properties
    // of the actual input and the fact that you can solve it by extrapolating
    // the quadratic sequence of steps to reachable tiles, but it attempts to
    // find the specific quadratic terms of the provided input instead of just
    // solving the specific equation for my specific input.
    // It uses the amount of reachable tiles at 65, 196 and 327 steps
    // to figure out the generic formula for the input.

    let f = |v: Vector3<f64>| Vector3::new(v[0] *   65.*65. + v[1] *  65. + v[2] -  reachable[65] as f64,
                                           v[0] * 196.*196. + v[1] * 196. + v[2] - reachable[196] as f64,
                                           v[0] * 327.*327. + v[1] * 327. + v[2] - reachable[327] as f64);

    // Props to this website because i had no clue about what a jacobian matrix
    // is or how to calculate it 🤡
    // https://www.allmath.com/jacobian-matrix-calculator.php
    let j = |_: Vector3<f64>| Matrix3::new(  65.*65.,  65., 1.,
                                           196.*196., 196., 1.,
                                           327.*327., 327., 1.);

    let solution = MultiVarNewton::new(f, j)
                .solve(Vector3::new(1., 1., 1.))
                .unwrap();

    solution.column(0).into_iter().copied().collect_tuple().unwrap()
}

fn do_steps(grid: &VecMat<char>, n_steps: usize, start: Pos) -> Vec<u64> {
    let mut to_visit = FxHashSet::default();
    to_visit.insert(start);
    let mut reachable = vec![1];

    for _ in 0..n_steps {
        let mut new_to_visit = FxHashSet::default();

        for pos in to_visit {
            new_to_visit.extend(
                pos.neighbors().iter().filter(|&n| get_tile_mirror(grid, *n) == '.')
            );
        }

        to_visit = new_to_visit;
        reachable.push(to_visit.len() as u64);
    }

    reachable
}

fn get_tile_mirror(grid: &VecMat<char>, pos: Pos) -> char {
    let x = pos.x.rem_euclid(grid.width() as i32);
    let y = pos.y.rem_euclid(grid.height() as i32);
    grid[(x, y)]
}

fn find_replace_start(grid: &mut VecMat<char>) -> Pos {
    let start = grid.indexed_iter().find(|p| p.1 == 'S')
        .unwrap().0;
    grid[start] = '.';
    start
}
