use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Card = (u128, u128);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").unwrap();
    let cards = input.lines().map(parse_line).collect_vec();
    
    let sol1: u64 = cards.iter().map(card_value).sum();
    
    let mut copies = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(i, card)| update_copies(i, card, &mut copies));
    let sol2: u64 = copies.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn update_copies(id: usize, card: &Card, copies: &mut [u64]) {
    let this_copies = copies[id];
    let won = (card.0 & card.1).count_ones() as usize;
    (id+1..id+won+1).for_each(|i| copies[i] += this_copies);
}

fn card_value((have, correct): &Card) -> u64 {
    match (have & correct).count_ones() {
        0 => 0,
        n => 1 << (n - 1),
    }
}

fn parse_line(line: &str) -> Card {
    let (_, body) = line.split_once(": ").unwrap();
    body.split(" | ").map(parse_numbers).collect_tuple().unwrap()
}

fn parse_numbers(data: &str) -> u128 {
    data.split_whitespace()
        .map(|x| 1 << x.parse::<u32>().unwrap())
        .fold(0, |a, b| a | b)
}
