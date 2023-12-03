use crate::{Solution, SolutionPair};
use crate::etc::{Coords2D, VecMat};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i32>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct NumPosition {
    val: u32,
    start: Pos
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").unwrap();
    let matrix = parse_input(&input);

    let sol1: u32 = matrix.indexed_iter()
        .filter(|(_, ch)| !ch.is_ascii_digit() && *ch != '.')
        .flat_map(|(pos, _)| find_numbers_around_pos(&matrix, pos))
        .unique()
        .map(|np| np.val)
        .sum();

    let sol2: u32 = matrix.indexed_iter()
        .filter(|(_, ch)| *ch == '*')
        .map(|(pos, _)| find_numbers_around_pos(&matrix, pos))
        .filter(|ls| ls.len() == 2)
        .map(|ls| ls[0].val * ls[1].val)
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

/** Returns all (deduplicated) numbers around any given position */
fn find_numbers_around_pos(matrix: &VecMat<char>, pos: Pos) -> Vec<NumPosition> {
    pos.neighbors_diag()
       .into_iter()
       .filter(|&pos| get(matrix, pos).is_ascii_digit())
       .map(|pos| read_number(matrix, pos))
       .unique()
       .collect()
}

/** Parses the number present in a position by backtracking until the
    start of the number is found and then parsing forward. */
fn read_number(m: &VecMat<char>, pos: Pos) -> NumPosition {
    let mut current = pos;
    while get(m, current + Pos::left()).is_ascii_digit() {
        current += Pos::left();
    }

    let start = current;
    let mut val = 0;
    while get(m, current).is_ascii_digit() {
        val = val * 10 + get(m, current).to_digit(10).unwrap();
        current += Pos::right();
    }

    NumPosition { val, start }
}

/** Aux function to safely get any value from the matrix, or `.` if 
 trying to access out-of-bounds. */
fn get(matrix: &VecMat<char>, pos: Pos) -> char {
    if pos.x < 0 || pos.y < 0 || pos.x >= matrix.width() as i32 || pos.y >= matrix.height() as i32 {
        '.'
    } else {
        matrix[pos]
    }
}

fn parse_input(input: &str) -> VecMat<char> {
    let width = input.lines().next().unwrap().len();
    let data = input.chars().filter(|ch| !ch.is_whitespace()).collect_vec();
    VecMat::from_data(width, data.len() / width, data)
}
