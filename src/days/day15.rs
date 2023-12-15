use std::array;
use std::fs::read_to_string;
use itertools::Itertools;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Lens<'a> = (&'a str, usize);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day15.txt").unwrap();
    let instrs = input.trim().split(',').collect_vec();

    let sol1: usize = instrs.iter().copied().map(hash_value).sum();
    let sol2 = calc_focus(&instrs);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn calc_focus(instrs: &[&str]) -> usize {
    let mut boxes: [Vec<Lens>; 256] = array::from_fn(|_| Vec::new());

    for instr in instrs {
        let (label, focal) = match instr.split_once('=') {
            Some((label, focal)) => (label, focal.parse().ok()),
            None => (instr.strip_suffix('-').unwrap(), None)
        };

        let the_box = &mut boxes[hash_value(label)];

        match focal {
            Some(focal_val) => insert_lens(label, focal_val, the_box),
            None => remove_lens(label, the_box),
        };
    }

    boxes.iter().enumerate().map(box_value).sum()
}

fn box_value((box_ix, cont): (usize, &Vec<Lens>)) -> usize {
    cont.iter().enumerate().map(|(i, (_, focal))| focal * (box_ix + 1) * (i + 1)).sum()
}

fn remove_lens(label: &str, cont: &mut Vec<Lens>) {
    if let Some(ix) = cont.iter().position(|lens| lens.0 == label) {
        cont.remove(ix);
    }
}

fn insert_lens<'a>(label: &'a str, focal: usize, cont: &mut Vec<Lens<'a>>) {
    if let Some(ix) = cont.iter().position(|lens| lens.0 == label) {
        cont[ix] = (label, focal);
    } else {
        cont.push((label, focal));
    }
}

fn hash_value(data: &str) -> usize {
    data.chars().fold(0, |prev, ch| (prev + ch as usize) * 17 % 256)
}
