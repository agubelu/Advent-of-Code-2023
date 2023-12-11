use std::fs::read_to_string;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use crate::{Solution, SolutionPair};
use crate::etc::{Coords2D, VecMat};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").unwrap();
    let mut grid = parse(&input);

    let start_pos = replace_start(&mut grid);
    let the_loop = find_loop(&grid, start_pos);

    let sol1 = the_loop.len() / 2;
    let sol2 = calculate_area(&grid, &the_loop);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn calculate_area(grid: &VecMat<char>, the_loop: &FxHashSet<Pos>) -> u32 {
    let mut area = 0;

    for y in 0..grid.height() {
        let mut inside = false;
        let mut prev_turn = ' ';

        for x in 0..grid.width() {
            let pos = (x as i32, y as i32).into();
            let tile = grid[pos];

            if the_loop.contains(&pos) {
                if matches!((tile, prev_turn), ('|', _) | ('7', 'L') | ('J', 'F')) {
                    inside = !inside;
                }

                if matches!(tile, 'F' | 'L' | '7' | 'J') {
                    prev_turn = tile;
                }
            } else if inside {
                area += 1;
            }
        }
    }

    area
}

fn find_loop(grid: &VecMat<char>, start: Pos) -> FxHashSet<Pos> {
    let mut dir = match grid[start] {
        '-' => Pos::right(),
        '|' => Pos::down(),
        'J' => Pos::up(),
        'L' => Pos::up(),
        '7' => Pos::down(),
        'F' => Pos::down(),
        _ => unreachable!()
    };

    let mut current = start;
    let mut the_loop = FxHashSet::default();

    while !the_loop.contains(&current) {
        the_loop.insert(current);
        current += dir;
        dir = next_direction(dir, grid[current]);
    }

    the_loop
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

fn parse(input: &str) -> VecMat<char> {
    let width = input.lines().next().unwrap().len();
    let data = input.chars().filter(|ch| !ch.is_whitespace()).collect_vec();
    VecMat::from_data(width, data.len() / width, data)
}
