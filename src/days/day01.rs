use crate::{Solution, SolutionPair};
use itertools::*;
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

static OPTIONS: [&str; 10] = ["\\d", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();
    let lines = input.lines().collect_vec();

    let re1_start = build_regex(&OPTIONS[..1], "");
    let re1_end = build_regex(&OPTIONS[..1], ".*");
    let re2_start = build_regex(&OPTIONS, "");
    let re2_end = build_regex(&OPTIONS, ".*");

    let sol1: usize = lines.iter().map(|line| line_value(line, &re1_start, &re1_end)).sum();
    let sol2: usize = lines.iter().map(|line| line_value(line, &re2_start, &re2_end)).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn line_value(line: &str, re_start: &Regex, re_end: &Regex) -> usize {
    let start_val = re_start.captures(line).unwrap().get(1).unwrap().as_str();
    let end_val = re_end.captures(line).unwrap().get(1).unwrap().as_str();
    substr_to_number(start_val) * 10 + substr_to_number(end_val)
}

fn substr_to_number(s: &str) -> usize {
    s.parse().unwrap_or_else(|_| OPTIONS.iter().position(|n| *n == s).unwrap())
}

fn build_regex(options: &[&str], prefix: &str) -> Regex {
    Regex::new(&format!("{prefix}({})", options.join("|"))).unwrap()
}
