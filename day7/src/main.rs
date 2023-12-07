use common::{boilerplate, Itertools};

fn part1(input: &str) -> usize {
    let mut hands = Hand::parse_all(input);
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.2)
        .sum()
}

fn part2(input: &str) -> usize {
    // Using '1' instead of 'J' to keep most logic the same as in `part1`.
    part1(&input.replace('J', "1"))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    /// High card, where all cards' labels are distinct: 23456
    HighCard,
    /// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    /// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    /// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    /// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    /// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind,
}

impl HandType {
    fn eval(hand: &[Card; 5]) -> Self {
        let mut counts = hand.iter().counts();
        let joker_count = counts.remove(&Card::JOKER).unwrap_or_default();
        let mut counts = counts.values().copied().collect_vec();
        counts.sort_unstable();
        let Some(highest_count) = counts.last_mut() else {
            // Five jokers, woohoo!!!
            return Self::FiveOfAKind;
        };
        *highest_count += joker_count;
        match &counts[..] {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => unreachable!("{counts:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand(HandType, [Card; 5], usize);

impl Hand {
    fn parse_all(input: &str) -> Vec<Self> {
        input.lines().map(Hand::parse).collect()
    }

    fn parse(input: &str) -> Self {
        let (cards, bid) = input.split_whitespace().collect_tuple().unwrap();
        let hand = cards
            .chars()
            .map(Card::new)
            .collect_vec()
            .try_into()
            .unwrap();
        Self(HandType::eval(&hand), hand, bid.parse().unwrap())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Card(usize);

impl Card {
    // Using '1' instead of 'J' to keep most logic the same as in `part1`.
    const JOKER: Self = Self(1);

    fn new(label: char) -> Self {
        Self(match label {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            c @ '1'..='9' => c as usize - '0' as usize,
            c => panic!("invalid card: {c}"),
        })
    }
}

boilerplate! {
    part1 => { test -> 6440, real -> 251106089 }
    part2 => { test -> 5905, real -> 249620106 }
}
