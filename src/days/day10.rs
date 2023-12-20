use std::fs::read_to_string;
use itertools::Itertools;
use crate::{Solution, SolutionPair};
use crate::etc::{Coords2D, VecMat};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i64>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").unwrap();
    let mut grid = VecMat::from_str(&input);

    let start_pos = replace_start(&mut grid);
    let vertices = find_loop_vertices(&grid, start_pos);

    let outside = outside_points(&vertices);
    let sol1 = outside / 2;
    let sol2 = inside_points(&vertices, outside);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

// https://en.wikipedia.org/wiki/Shoelace_formula
fn total_area(vertices: &[Pos]) -> i64 {
    vertices.iter().tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>().abs() / 2
}

pub fn outside_points(vertices: &[Pos]) -> i64 {
    vertices.iter().tuple_windows().map(|(a, b)| a.manhattan_dist(b)).sum()
}

// https://en.m.wikipedia.org/wiki/Pick's_theorem
pub fn inside_points(vertices: &[Pos], outside: i64) -> i64 {
    let area = total_area(vertices);
    area - (outside / 2) + 1
}

fn find_loop_vertices(grid: &VecMat<char>, start: Pos) -> Vec<Pos> {
    let mut dir = match grid[start] {
        '|' | '7' | 'F' => Pos::down(),
        'J' | 'L' => Pos::up(),
        _ => Pos::right(),
    };

    let mut current = start;
    let mut vertices = vec![];

    loop {
        if !matches!(grid[current], '-' | '|') {
            vertices.push(current);
        }
        current += dir;
        dir = next_direction(dir, grid[current]);
        if current == start { break }
    }

    vertices.push(vertices[0]);
    vertices
}

fn next_direction(prev_direction: Pos, tile: char) -> Pos {
    let [up, down, left, right] = Pos::origin().neighbors();

    match (tile, prev_direction) {
        ('|', d) | ('-', d) => d,
        ('L', d) if d == down => right,
        ('L', d) if d == left => up,
        ('F', d) if d == up => right,
        ('F', d) if d == left => down,
        ('7', d) if d == right => down,
        ('7', d) if d == up => left,
        ('J', d) if d == down => left,
        ('J', d) if d == right => up,
        _ => unreachable!()
    }
}

fn replace_start(grid: &mut VecMat<char>) -> Pos {
    let start_index = grid.indexed_iter::<u32>().position(|x| x.1 == 'S').unwrap();
    let start_pos = grid.coords(start_index);

    let up = matches!(grid.get_or(start_pos.go_up(), '.'), '|' | '7' | 'F');
    let down = matches!(grid.get_or(start_pos.go_down(), '.'), '|' | 'J' | 'L');
    let left = matches!(grid.get_or(start_pos.go_left(), '.'), '-' | 'L' | 'F');
    let right = matches!(grid.get_or(start_pos.go_right(), '.'), '-' | 'J' | '7');

    let new_tile = match (up, down, left, right) {
        (true, true, _, _) => '|',
        (true, _, true, _) => 'J',
        (true, _, _, true) => 'L',
        (_, true, true, _) => '7',
        (_, true, _, true) => 'F',
        (_, _, true, true) => '-',
        _ => unreachable!()
    };

    grid[start_pos] = new_tile;
    start_pos
}
