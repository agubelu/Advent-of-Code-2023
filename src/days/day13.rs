use std::fs::read_to_string;
use itertools::Itertools;

use crate::etc::{DOUBLE_NEWLINE, VecMat};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day13.txt").unwrap();
    let mut grids = input.split(DOUBLE_NEWLINE).map(VecMat::from_str).collect_vec();

    let sol1: usize = grids.iter().map(|g| reflection_values(g)[0]).sum();
    let sol2: usize = grids.iter_mut().map(new_reflection_value).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn new_reflection_value(grid: &mut VecMat<char>) -> usize {
    let old_value = reflection_values(grid)[0];

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let ch = grid[(x, y)];
            let new_ch = if ch == '#' { '.' } else { '#' };
            grid[(x, y)] = new_ch;

            let mut new_values = reflection_values(grid).into_iter().filter(|x| *x != old_value);
            if let Some(new) = new_values.next() {
                return new;
            }
            grid[(x, y)] = ch;
        }
    }

    unreachable!()
}

fn reflection_values(grid: &VecMat<char>) -> Vec<usize> {
   let vert = (0..grid.width() - 1)
        .filter(|x| has_vertical_symmetry(grid, *x))
        .map(|x| x + 1);

    let hor = (0..grid.height() - 1)
        .filter(|x| has_horizontal_symmetry(grid, *x))
        .map(|x| 100 * (x + 1));

    let mut res = vert.collect_vec();
    res.extend(hor);
    res
}

fn has_vertical_symmetry(grid: &VecMat<char>, col: usize) -> bool {
    let mut left = col as i32;
    let mut right = left + 1;

    while left >= 0 && right < grid.width() as i32 {
        if grid.get_col(left as usize) != grid.get_col(right as usize) {
            return false;
        }
        left -= 1;
        right += 1;
    }

    true
}

fn has_horizontal_symmetry(grid: &VecMat<char>, row: usize) -> bool {
    let mut top = row as i32;
    let mut bottom = top + 1;

    while top >= 0 && bottom < grid.height() as i32 {
        if grid.get_row(top as usize) != grid.get_row(bottom as usize) {
            return false;
        }
        top -= 1;
        bottom += 1;
    }

    true
}
