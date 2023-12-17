use std::fs::read_to_string;
use pathfinding::directed::astar::astar;
use crate::etc::{Coords2D, VecMat};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    prev_dir: Pos,
    repeats: u8,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day17.txt").unwrap();
    let grid = VecMat::map_from_str(&input, |x| x.to_digit(10).unwrap());

    let sol1 = find_best_path(&grid, false);
    let sol2 = find_best_path(&grid, true);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_best_path(grid: &VecMat<u32>, ultra: bool) -> u32 {
    let goal = Pos::new(grid.width() as i32 - 1, grid.height() as i32 - 1);

    astar(
        &State { pos: Pos::origin(), prev_dir: Pos::origin(), repeats: 1 },
        |state| available_steps(state, grid, ultra),
        |state| state.pos.manhattan_dist(&goal) as u32,
        |state| state.pos == goal
    ).unwrap().1
}

fn available_steps(state: &State, grid: &VecMat<u32>, ultra: bool) -> Vec<(State, u32)> {
    let (min_repeats, max_repeats) = if ultra { (4, 10) } else { (1, 3) };

    state.pos.neighbors().into_iter()
        .map(|new_pos| (new_pos, new_pos - state.pos))
        .filter(|&(new_pos, new_dir)| {
            // Force a minimum amount of direction repetitions
            // (only applies to part 2, and if not in the starting point)
            (state.repeats >= min_repeats || new_dir == state.prev_dir || state.prev_dir == Pos::origin()) &&
            new_dir != -state.prev_dir && // Don't backtrack
            grid.is_in_bounds(new_pos) && // Don't go OOB
            !(state.repeats >= max_repeats && new_dir == state.prev_dir) // Don't repeat directions
        })
        .map(|(new_pos, new_dir)| {
            let repeats = if new_dir != state.prev_dir { 1 } else { state.repeats + 1 };
            let state = State { pos: new_pos, prev_dir: new_dir, repeats };
            (state, grid[new_pos])
        }).collect()
}
