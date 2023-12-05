use std::fs::read_to_string;
use itertools::Itertools;
use crate::etc::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};
use RangeOverlap::*;

///////////////////////////////////////////////////////////////////////////////

type Range = (i64, i64);
type RangeMapData = (i64, i64, i64);
type Mapper = Vec<RangeMapData>;

enum RangeOverlap {
    Disjoint,
    Contained,
    Contains,
    Left,
    Right
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").unwrap();
    let mut sections = input.split(DOUBLE_NEWLINE);

    let seeds = sections.next().unwrap()
        .split_once(": ").unwrap().1
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let maps = sections.map(parse_map).collect_vec();

    let seeds_p1 = seeds.iter().map(|&val| (val, val)).collect_vec();
    let seeds_p2 = seeds.iter().chunks(2).into_iter().map(|iter| {
        let (&start, &size) = iter.collect_tuple().unwrap();
        (start, start + size - 1)
    }).collect_vec();

    let sol1 = solve_for_ranges(&seeds_p1, &maps);
    let sol2 = solve_for_ranges(&seeds_p2, &maps);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn solve_for_ranges(ranges: &[Range], maps: &[Mapper]) -> i64 {
    ranges.iter()
          .map(|&r| apply_maps(r, maps).into_iter().map(|rs| rs.0).min().unwrap())
          .min().unwrap()
}

fn apply_maps(range: Range, maps: &[Mapper]) -> Vec<Range> {
    maps.iter()
        .fold(
            vec![range], 
            |prev, map| prev.iter().flat_map(|r| map_range(map, *r)).collect()
        )
}

fn map_range(map: &Mapper, range: Range) -> Vec<Range> {
    let mut remaining = vec![range];
    let mut mapped = vec![];

    for &(dst, src, size) in map {
        let mapping_range = (src, src + size - 1);
        let mut new_remaining = vec![];

        for range in remaining {
            let (outside, inside) = split_ranges(range, mapping_range);
            new_remaining.extend(outside);

            if let [to_map] = inside.as_slice() {
                let start = dst + (to_map.0 - src);
                let end = dst + (to_map.1 - src);
                mapped.push((start, end))
            }
        }

        remaining = new_remaining;
    }

    mapped.extend(remaining);
    mapped
}

fn split_ranges(a: Range, b: Range) -> (Vec<Range>, Vec<Range>) {
    match find_overlap(a, b) {
        Disjoint => (vec![a], vec![]),
        Contained => (vec![], vec![a]),
        Left => (vec![(a.0, b.0 - 1)], vec![(b.0, a.1)]),
        Right => (vec![(b.1 + 1, a.1)], vec![(a.0, b.1)]),
        Contains => (vec![(a.0, b.0 - 1), (b.1 + 1, a.1)], vec![(b.0, b.1)]),
    }
}

fn find_overlap((start_a, end_a): Range, (start_b, end_b): Range) -> RangeOverlap {
    if start_a >= start_b && end_a <= end_b {
        Contained
    } else if start_a <= start_b && end_a >= end_b {
        Contains
    } else if start_a < start_b && end_a >= start_b && end_a < end_b {
        Left
    } else if start_a > start_b && start_a < end_b && end_a > end_b {
        Right
    } else {
        Disjoint
    }
}

///////////////////////////////////////////////////////////////////////////////

fn parse_map(lines: &str) -> Mapper {
    lines.lines()
         .skip(1)
         .map(parse_line)
         .collect()
}

fn parse_line(line: &str) -> RangeMapData {
    line.split(' ').map(|x| x.parse().unwrap()).collect_tuple().unwrap()
}
