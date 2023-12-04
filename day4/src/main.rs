use common::{boilerplate, Itertools};
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    Card::parse_all(input)
        .map(|card| match card.matching_count() {
            0 => 0,
            c => 2usize.pow(c as u32 - 1),
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cards = Card::parse_all(input).collect_vec();
    for i in 0..cards.len() {
        let matching_count = cards[i].matching_count();
        let count = cards[i].count;
        for card in cards[i + 1..].iter_mut().take(matching_count) {
            card.count += count;
        }
    }
    cards.into_iter().map(|card| card.count).sum()
}

#[derive(Debug)]
struct Card {
    count: usize,
    left: HashSet<usize>,
    right: HashSet<usize>,
}

impl Card {
    fn matching_count(&self) -> usize {
        self.left.intersection(&self.right).count()
    }

    fn parse_all(input: &str) -> impl Iterator<Item = Self> + '_ {
        input.lines().map(|line| {
            let (_id, line) = line.split_once(':').unwrap();
            let (left, right) = line.split_once(" | ").unwrap();
            Self {
                count: 1,
                left: parse_numbers(left),
                right: parse_numbers(right),
            }
        })
    }
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .split_ascii_whitespace()
        .map(|nr| nr.parse().unwrap())
        .collect()
}

boilerplate! {
    part1 => { test -> 13, real -> 20407 }
    part2 => { test -> 30, real -> 23806951 }
}
