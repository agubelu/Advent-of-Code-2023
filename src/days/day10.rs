// I will come back to redo this monstrosity

use std::fs::read_to_string;
use itertools::Itertools;
use rustc_hash::{FxHashSet, FxHashMap};
use crate::{Solution, SolutionPair};
use crate::etc::{Coords2D, VecMat};
use Tile::*;

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Vertical, Horizontal, UpRight, UpLeft, DownRight, DownLeft, Ground, Start
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").unwrap();
    let mut grid = parse(&input);

    let start_pos = replace_start(&mut grid);
    let the_loop = find_loop(&grid, start_pos);
    let normals = find_normals(&grid, &the_loop);

    let sol1 = the_loop.len() / 2;
    let sol2 = grid.indexed_iter().filter(|(pos, _)| is_inside_loop(*pos, &the_loop, &normals)).count();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn is_inside_loop(pos: Pos, the_loop: &FxHashSet<Pos>, normals: &FxHashMap<Pos, Pos>) -> bool {
    if the_loop.contains(&pos) {
        return false;
    }

    let mut current = pos;
    while current.y >= 0 {
        if the_loop.contains(&current) {
            return normals[&current].y > 0;
        }

        current += Pos::up();
    }

    false
}

fn find_normals(grid: &VecMat<Tile>, the_loop: &FxHashSet<Pos>) -> FxHashMap<Pos, Pos> {
    let mut current_pos = Pos::origin();
    let mut normals = FxHashMap::default();

    // Find the first F and store it in current
    'outer : for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Pos::new(x as i32, y as i32);
            if the_loop.contains(&pos) {
                current_pos = pos;
                break 'outer;
            }
        }
    }

    let mut current_normal = (1, 1).into();
    let mut current_dir = Pos::down();

    // Go through the loop to store all normals
    while normals.len() < the_loop.len() {
        normals.insert(current_pos, current_normal);
        current_pos += current_dir;

        let next_tile = grid[current_pos];
        current_normal = next_normal(current_normal, current_dir, next_tile);
        current_dir = next_direction(current_dir, next_tile);
    }

    normals
}

fn find_loop(grid: &VecMat<Tile>, start: Pos) -> FxHashSet<Pos> {
    let mut dir = match grid[start] {
        Horizontal => Pos::right(),
        Vertical => Pos::down(),
        UpLeft => Pos::up(),
        UpRight => Pos::up(),
        DownLeft => Pos::down(),
        DownRight => Pos::down(),
        _ => unreachable!()
    };

    let mut current = start;
    let mut the_loop = FxHashSet::default();
    the_loop.insert(current);

    while current + dir != start {
        current += dir;
        dir = next_direction(dir, grid[current]);
        the_loop.insert(current);
    }

    the_loop
}

fn next_direction(prev_direction: Pos, tile: Tile) -> Pos {
    let [up, down, left, right] = Pos::origin().neighbors();

    match (tile, prev_direction) {
        (Vertical, d) if d == up => up,
        (Vertical, d) if d == down => down,
        (Horizontal, d) if d == left => left,
        (Horizontal, d) if d == right => right,
        (UpRight, d) if d == down => right,
        (UpRight, d) if d == left => up,
        (DownRight, d) if d == up => right,
        (DownRight, d) if d == left => down,
        (DownLeft, d) if d == right => down,
        (DownLeft, d) if d == up => left,
        (UpLeft, d) if d == down => left,
        (UpLeft, d) if d == right => up,
        _ => unreachable!()
    }
}

fn next_normal(prev_normal: Pos, prev_dir: Pos, tile: Tile) -> Pos {
    let [up, down, left, right] = Pos::origin().neighbors();

    match (tile, prev_dir, prev_normal) {
        (Vertical, _, n) => Pos::new(n.x, 0),
        (Horizontal, _, n) => Pos::new(0, n.y),
        (UpRight, d, n) if d.y > 0 && n.x > 0 ||
                        d.x < 0 && n.y < 0 => up + right,
        (UpRight, _, _) => down + left,
        (UpLeft, d, n) if d.y > 0 && n.x > 0 ||
                        d.x > 0 && n.y > 0 => down + right,
        (UpLeft, _, _) => up + left,
        (DownLeft, d, n) if d.x > 0 && n.y > 0 ||
                         d.y < 0 && n.x < 0 => down + left,
        (DownLeft, _, _) => up + right,
        (DownRight, d, n) if d.x < 0 && n.y > 0 ||
                          d.y < 0 && n.x > 0 => down + right,
        (DownRight, _, _) => up + left,
        _ => unreachable!()
    }
}

fn replace_start(grid: &mut VecMat<Tile>) -> Pos {
    let start_index = grid.indexed_iter::<u32>().position(|x| x.1 == Start).unwrap();
    let start_pos = grid.coords(start_index);

    let up = matches!(get(grid, start_pos + Pos::up()), Vertical | DownLeft | DownRight);
    let down = matches!(get(grid, start_pos + Pos::down()), Vertical | UpLeft | UpRight);
    let left = matches!(get(grid, start_pos + Pos::left()), Horizontal | UpRight | DownRight);
    let right = matches!(get(grid, start_pos + Pos::right()), Horizontal | UpLeft | DownLeft);

    let new_tile = match (up, down, left, right) {
        (true, true, _, _) => Vertical,
        (true, _, true, _) => UpLeft,
        (true, _, _, true) => UpRight,
        (_, true, true, _) => DownLeft,
        (_, true, _, true) => DownRight,
        (_, _, true, true) => Horizontal,
        _ => unreachable!()
    };

    grid[start_pos] = new_tile;
    start_pos
}

fn parse(input: &str) -> VecMat<Tile> {
    let width = input.lines().next().unwrap().len();
    let data = input.chars().filter(|ch| !ch.is_whitespace())
        .map(|ch| match ch {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => UpRight,
            'J' => UpLeft,
            'F' => DownRight,
            '7' => DownLeft,
            '.' => Ground,
            'S' => Start,
             _  => unreachable!()
        }).collect_vec();
    VecMat::from_data(width, data.len() / width, data)
}

fn get(grid: &VecMat<Tile>, pos: Pos) -> Tile {
    if grid.is_in_bounds(pos) {
        grid[pos]
    } else {
        Ground
    }
}
