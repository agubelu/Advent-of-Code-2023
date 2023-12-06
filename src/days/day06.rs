use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").unwrap();
    
    let races = parse(&input);
    let big_race = parse(&input.replace(' ', ""))[0];

    let sol1: u64 = races.iter()
        .map(|&r| find_race_solutions(r))
        .product();
    let sol2 = find_race_solutions(big_race);

    (Solution::from(sol1), Solution::from(sol2))
}

/*
    Let T = race time, p = pressing down time, R = record
    We need to find values of t such that R < p * (T-p)
    Expanding:
    R < Tp - p²
    p² - Tp + R < 0

    Solving the quadratic for when the right side = 0
    p = (T ± sqrt(T² - 4R)) / 2

    So the values of p for which the original inequality
    holds are those between the two previous solutions.
*/

fn find_race_solutions((max_time, record): (u64, u64)) -> u64 {
    let t = max_time as f64;
    let r = record as f64;
    let sqrt = (t*t - 4.0*r).sqrt();

    let p1 = ((t + sqrt) / 2.0).ceil();
    let p2 = ((t - sqrt) / 2.0).floor();
    (p1 - p2) as u64 - 1
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();

    parse_line(lines.next().unwrap())
        .zip(parse_line(lines.next().unwrap()))
        .collect()
}

fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_once(':').unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
}
