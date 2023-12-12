use std::cmp::Ordering::*;
use std::fs::read_to_string;
use std::iter::repeat;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Cache<'a> = FxHashMap<(&'a [char], &'a [usize]), u64>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12.txt").unwrap();

    let data1 = input.lines().map(|line| parse_line(line, 1)).collect_vec();
    let data2 = input.lines().map(|line| parse_line(line, 5)).collect_vec();
    
    let sol1: u64 = data1.iter().map(|(d, g)| arrangements(d, g, &mut Cache::default())).sum();
    let sol2: u64 = data2.iter().map(|(d, g)| arrangements(d, g, &mut Cache::default())).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn arrangements<'b, 'a: 'b>(data: &'a [char], groups: &'a [usize], cache: &mut Cache<'b>) -> u64 {
    if groups.is_empty() {
        return data.iter().all(|c| *c != '#') as u64;
    }

    if data.is_empty() {
        return groups.is_empty() as u64;
    }

    if let Some(&res) = cache.get(&(data, groups)) {
        return res;
    }

    let res = match data[0] {
        '.' => try_skip(data, groups, cache),
        '#' => try_match(data, groups, cache),
         _  => try_skip(data, groups, cache) + try_match(data, groups, cache)
    };

    cache.insert((data, groups), res);
    res
}

fn try_skip<'b, 'a: 'b>(data: &'a [char], groups: &'a [usize], cache: &mut Cache<'b>) -> u64 {
    let can_skip = groups.is_empty() || data.len() > groups.iter().sum::<usize>() + (groups.len() - 1);

    if can_skip {
        arrangements(&data[1..], groups, cache)
    } else { 0 }
}

fn try_match<'b, 'a: 'b>(data: &'a [char], groups: &'a [usize], cache: &mut Cache<'b>) -> u64 {
    let group_len = groups[0];
    let can_start_match = match data.len().cmp(&group_len) {
        Less => false,
        Equal => data.iter().all(|c| *c != '.'),
        Greater => data[..group_len].iter().all(|c| *c != '.') && data[group_len] != '#',
    };

    if can_start_match {
        let mut next_start = groups[0];
        if data.len() > next_start { next_start += 1 }
        arrangements(&data[next_start..], &groups[1..], cache)
    } else { 0 }
}

fn parse_line(line: &str, repeats: usize) -> (Vec<char>, Vec<usize>) {
    let (left, right) = line.split_once(' ').unwrap();

    let chars = repeat(left).take(repeats).join("?");
    let counts = repeat(right).take(repeats).join(",");

    let chars_vec = chars.chars().collect_vec();
    let counts_vec = counts.split(',').map(|x| x.parse().unwrap()).collect_vec();
    (chars_vec, counts_vec)
}
