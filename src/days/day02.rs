use crate::{Solution, SolutionPair};
use itertools::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
 
type CubeInfo<'a> = (u64, &'a str);

struct CubeGame<'a> {
    id: u32,
    data: Vec<CubeInfo<'a>>
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").unwrap();
    let games = input.lines().map(parse_line).collect_vec();

    let sol1: u32 = games.iter().filter(|game| is_valid_game(game)).map(|game| game.id).sum();
    let sol2: u64 = games.iter().map(|game| game_power(game)).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn is_valid_game(game: &CubeGame) -> bool {
    game.data.iter().all(|cube_info| {
        match cube_info {
            (x, "red") => *x <= 12,
            (x, "green") => *x <= 13,
            (x, "blue") => *x <= 14,
            _ => unreachable!()
        }
    })
}

fn game_power(game: &CubeGame) -> u64 {
    ["red", "green", "blue"].iter().map(|color| {
        game.data.iter()
                 .filter(|(_, kind)| kind == color)
                 .map(|(amt, _)| amt)
                 .max()
                 .unwrap_or(&0)
    }).product()
}

fn parse_line(line: &str) -> CubeGame {
    let (head, body) = line.split_once(": ").unwrap();
    let id = head[5..].parse().unwrap();
    let data = body.split("; ").flat_map(parse_cube_group).collect_vec();
    CubeGame { id, data }

}

fn parse_cube_group(group: &str) -> Vec<CubeInfo> {
    group.split(", ").map(|info| {
        let spl = info.split_once(' ').unwrap();
        (spl.0.parse().unwrap(), spl.1)
    }).collect_vec()
}
