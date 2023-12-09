use std::fs::read_to_string;
use itertools::Itertools;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Seq = Vec<i64>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09.txt").unwrap();
    let sequences = input.lines().map(parse_line).collect_vec();

    let sol1: i64 = sequences.iter().map(extrapolate_forwards).sum();
    let sol2: i64 = sequences.iter().map(extrapolate_backwards).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn extrapolate_forwards(seq: &Seq) -> i64 {
    let mut sequences = generate_diffs(seq);
    let last = sequences.len() - 1;
    sequences[last].push(0);

    for i in (0..last).rev() {
        let new = sequences[i].last().unwrap() + sequences[i+1].last().unwrap();
        sequences[i].push(new);
    }

    *sequences[0].last().unwrap()
}

fn extrapolate_backwards(seq: &Seq) -> i64 {
    let mut sequences = generate_diffs(seq);
    let last = sequences.len() - 1;
    sequences[last].insert(0, 0);

    for i in (0..last).rev() {
        let new = sequences[i][0] - sequences[i+1][0];
        sequences[i].insert(0, new);
    }

    sequences[0][0]
}

fn generate_diffs(seq: &Seq) -> Vec<Seq> {
    let mut sequences = vec![seq.clone()];
    let mut last = 0;
    
    while !sequences[last].iter().all(|x| *x == 0) {
        let new_seq = sequences[last].windows(2).map(|x| x[1] - x[0]).collect_vec();
        sequences.push(new_seq);
        last += 1;
    }

    sequences
}

fn parse_line(line: &str) -> Seq {
    line.split_whitespace().map(|x| x.parse().unwrap()).collect()
}
