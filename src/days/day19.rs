use std::fs::read_to_string;
use std::cmp::{min, max};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use crate::etc::DOUBLE_NEWLINE;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r"(\w*)(?:([<>])(\d+):(.*))?").unwrap();
}

type WorkflowMap<'a> = FxHashMap<&'a str, Vec<Rule<'a>>>;
type Bounds = [(i64, i64); 4];
struct Piece {x: i64, m: i64, a: i64, s: i64}

enum Condition {
    Greater(i64, char),
    Less(i64, char),
    Always
}

struct Rule<'a> {
    cond: Condition,
    target: &'a str
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day19.txt").unwrap();
    let (rules_str, pieces_str) = input.split_once(DOUBLE_NEWLINE).unwrap();

    let rules: WorkflowMap = rules_str.lines().map(parse_workflow).collect();
    let pieces = pieces_str.lines().map(parse_piece).collect_vec();

    let sol1: i64 = pieces.iter()
        .filter(|p| process_piece(p, &rules, "in"))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum();

    let bounds = [(1, 4000); 4];
    let sol2 = find_accepts("in", &rules, bounds);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_accepts(label: &str, wfs: &WorkflowMap, mut bounds: Bounds) -> i64 {
    if label == "A" { return bounds_size(bounds) }
    if label == "R" { return 0 }
    let mut total = 0;

    for rule in &wfs[label] {
        let next_bounds = apply_condition_bounds(bounds, &rule.cond);
        total += find_accepts(rule.target, wfs, next_bounds);
        bounds = apply_condition_bounds_reverse(bounds, &rule.cond);
    }

    total
}

fn bounds_size(bounds: Bounds) -> i64 {
    bounds.into_iter().map(|(lower, upper)| max(0, upper - lower + 1)).product()
}

fn apply_condition_bounds(bounds: Bounds, cond: &Condition) -> Bounds {
    let (val, prop) = match cond {
        Condition::Always => return bounds,
        Condition::Greater(x, y) => (x, y),
        Condition::Less(x, y) => (x, y),
    };

    let ix = match prop {
        'x' => 0,
        'm' => 1,
        'a' => 2,
         _  => 3
    };

    let mut new_bounds = bounds;
    let prop_bounds = new_bounds[ix];

    match cond {
        Condition::Greater{..} => new_bounds[ix] = (max(val + 1, prop_bounds.0), prop_bounds.1),
        Condition::Less{..} => new_bounds[ix] = (prop_bounds.0, min(prop_bounds.1, val - 1)),
        _ => unreachable!()
    };
    new_bounds
}

fn apply_condition_bounds_reverse(bounds: Bounds, cond: &Condition) -> Bounds {
    let new_cond = match cond {
        Condition::Always => return bounds,
        Condition::Greater(val, prop) => Condition::Less(val + 1, *prop),
        Condition::Less(val, prop) => Condition::Greater(val - 1, *prop)
    };

    apply_condition_bounds(bounds, &new_cond)
}

fn process_piece(piece: &Piece, wfs: &WorkflowMap, target: &str) -> bool {
    if target == "A" { return true }
    if target == "R" { return false }

    for rule in &wfs[target] {
        if matches_rule(piece, rule) {
            return process_piece(piece, wfs, rule.target);
        }
    }

    unreachable!()
}

fn matches_rule(piece: &Piece, rule: &Rule) -> bool {
    match rule.cond {
        Condition::Always => true,
        Condition::Greater(val, attr) => get_attr(piece, attr) > val,
        Condition::Less(val, attr) => get_attr(piece, attr) < val,
    }
}

fn get_attr(piece: &Piece, attr: char) -> i64 {
    match attr {
        'x' => piece.x,
        'm' => piece.m,
        'a' => piece.a,
         _  => piece.s
    }
}

fn parse_workflow(line: &str) -> (&str, Vec<Rule>) {
    let (label, rest) = line.split_once('{').unwrap();
    let body = rest.trim_end_matches('}').split(',')
        .map(parse_rule)
        .collect_vec();

    (label, body)
}

fn parse_rule(text: &str) -> Rule {
    let m = RE_RULE.captures(text).unwrap();

    if m.get(2).is_none() {
        let target = m.get(1).unwrap().as_str();
        Rule { target, cond: Condition::Always }
    } else {
        let prop = m.get(1).unwrap().as_str().chars().next().unwrap();
        let op = m.get(2).unwrap().as_str();
        let val = m.get(3).unwrap().as_str().parse().unwrap();
        let target = m.get(4).unwrap().as_str();

        let cond = match op {
            ">" => Condition::Greater(val, prop),
             _  => Condition::Less(val, prop)
        };
        Rule { target, cond }
    }
}

fn parse_piece(line: &str) -> Piece {
    let clean = &line[1..line.len() - 1]; // Remove wrapping { }
    let (x, m, a, s) = clean.split(',').map(|x| x.split('=').nth(1).unwrap().parse().unwrap())
        .collect_tuple().unwrap();
    Piece { x, m, a, s }
}
