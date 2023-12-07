use std::fs::read_to_string;
use itertools::Itertools;
use crate::{Solution, SolutionPair};
use HandStrength::*;

///////////////////////////////////////////////////////////////////////////////

type BetData = (Vec<u8>, u64);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength { HighCard, Pair, TwoPairs, Three, Full, Poker, Repoker }


pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").unwrap();

    let hands_1 = input.lines().map(|line| parse_hand(line, "23456789TJQKA")).collect_vec();
    let hands_2 = input.lines().map(|line| parse_hand(line, "J23456789TQKA")).collect_vec();

    let sol1 = get_winnings(&hands_1, hand_strength_1);
    let sol2 = get_winnings(&hands_2, hand_strength_2);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn get_winnings<F>(hands: &[BetData], strength: F) -> u64
where F: Fn(&[u8]) -> HandStrength {
    hands.iter()
         .sorted_by_cached_key(|(hand, _)| (strength(hand), hand))
         .enumerate()
         .map(|(i, &(_, bet))| (i as u64 + 1) * bet)
         .sum()
}

fn hand_strength_1(hand: &[u8]) -> HandStrength {
    let mut freqs = hand.iter().counts().into_values().sorted().rev();
    let most_common = freqs.next().unwrap();
    let second = freqs.next().unwrap_or(0);

    match (most_common, second) {
        (5, _) => Repoker,
        (4, _) => Poker,
        (3, 2) => Full,
        (3, _) => Three,
        (2, 2) => TwoPairs,
        (2, _) => Pair,
        _ => HighCard
    }
}

fn hand_strength_2(hand: &[u8]) -> HandStrength {
    // Find the second most common card after jokers
    let second = hand.iter()
        .filter(|&x| *x != 0)
        .counts()
        .into_iter()
        .sorted_by_key(|x| x.1)
        .next_back()
        .map(|x| *x.0)
        .unwrap_or(0);

    // All jokers turn into that card
    let new_hand = hand.iter().map(|&x| if x == 0 {second} else {x});
    hand_strength_1(&new_hand.collect_vec())
}

fn parse_hand(line: &str, tier: &str) -> BetData {
    let (hand_txt, bet_txt) = line.split_once(' ').unwrap();
    let bet = bet_txt.parse().unwrap();
    let hand = hand_txt.chars()
        .map(|ch| tier.chars().position(|x| x == ch).unwrap() as u8)
        .collect();

    (hand, bet)
}
