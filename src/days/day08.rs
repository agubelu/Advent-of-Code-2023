use std::fs::read_to_string;

use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use rustc_hash::FxHashMap;

use crate::etc::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type CaveMap<'a> = FxHashMap<&'a str, (&'a str, &'a str)>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08.txt").unwrap();
    let (head, body) = input.split_once(DOUBLE_NEWLINE).unwrap();
    let map = parse(body);

    let sol1 = find_dist(&map, "AAA", |to| to == "ZZZ", head.chars().cycle());
    let sol2 = map.keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| find_dist(&map, node, |to| to.ends_with('Z'), head.chars().cycle()))
        .reduce(lcm).unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_dist<I, F>(map: &CaveMap, from: &str, goal: F, mut dirs: I) -> u64
where I: Iterator<Item = char>,
      F: Fn(&str) -> bool 
{
    let mut current = from;
    let mut steps = 0;

    while !goal(current) {
        current = match dirs.next().unwrap() {
            'L' => map[current].0,
             _  => map[current].1,
        };
        steps += 1
    }

    steps
}

fn parse(body: &str) -> CaveMap {
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    body.lines()
        .map(|line| {
            let m = re.captures(line).unwrap();
            let (node, left, right) = [1,2,3].into_iter().map(|i| m.get(i).unwrap().as_str())
                .collect_tuple().unwrap();
            (node, (left, right))
        })
        .collect()
}
