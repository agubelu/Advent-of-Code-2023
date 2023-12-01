use crate::{Solution, SolutionPair};
use itertools::*;
use std::fs::read_to_string;
use SearchOrder::*;
///////////////////////////////////////////////////////////////////////////////

enum SearchOrder { First, Last }

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();
    let lines = input.lines().collect_vec();

    let sol1: u32 = lines.iter().copied().map(line_value).sum();
    let sol2: u32 = lines.iter().map(|line| line_value(&transform_letters(line))).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn transform_letters(line: &str) -> String {
    let transforms = [("one", "o1e"), ("two", "t2o"), ("three", "t3e"),
                      ("four", "f4r"), ("five", "f5e"), ("six", "s6x"),
                      ("seven", "s7n"), ("eight", "e8t"), ("nine", "n9e")];

    let mut res = line.to_owned();
    for (from, to) in transforms {
        res = res.replace(from, to);
    }

    res
}

fn line_value(line: &str) -> u32 {
    find_number(line, First) * 10 + find_number(line, Last)
}

fn find_number(line: &str, order: SearchOrder) -> u32 {
    match order {
        First => line.chars().find_map(|ch| ch.to_digit(10)).unwrap(),
        Last => line.chars().rev().find_map(|ch| ch.to_digit(10)).unwrap()
    }
}
