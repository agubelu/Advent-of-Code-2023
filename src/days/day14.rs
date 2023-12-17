use std::fs::read_to_string;
use rustc_hash::FxHashMap;
use crate::etc::{VecMat, Coords2D};
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Grid = VecMat<char>;
type Pos = Coords2D<i32>;
const PART_2: u64 = 1_000_000_000;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day14.txt").unwrap();
    let mut grid1 = VecMat::from_str(&input);
    let mut grid2 = grid1.clone();

    slide_up(&mut grid1);
    let sol1 = calculate_load(&grid1);

    let (offset, cycle_len) = find_repetition(&mut grid2.clone());
    let n = offset + (PART_2 - offset) % cycle_len;

    for _ in 0..n {
        cycle(&mut grid2);
    }
    let sol2 = calculate_load(&grid2);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_repetition(grid: &mut Grid) -> (u64, u64) {
    let mut i = 0;
    let mut memory = FxHashMap::default();

    loop {
        if let Some(&prev) = memory.get(grid) {
            break (prev, i - prev);
        }
        memory.insert(grid.clone(), i);
        cycle(grid);
        i += 1;
    }
}

fn cycle(grid: &mut Grid) {
    for _ in 0..4 {
        slide_up(grid);
        grid.rotate_right();
    }
}

fn slide_up(grid: &mut Grid) {
    for y in 0..grid.height() as i32 {
        for x in 0..grid.width() as i32 {
            slide_rock(grid, (x, y).into(), Pos::up());
        }
    }
}

fn slide_rock(grid: &mut Grid, pos: Pos, dir: Pos) {
    if grid[pos] != 'O' {
        return;
    }
    
    let mut current = pos;
    while grid.get_or(current + dir, '#') == '.' {
        current += dir;
    }

    grid[pos] = '.';
    grid[current] = 'O';
}

fn calculate_load(grid: &Grid) -> usize {
    grid.indexed_iter::<usize>()
        .filter(|(_, ch)| *ch == 'O')
        .map(|(pos, _)| grid.height() - pos.y)
        .sum()
}
