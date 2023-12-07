use std::cmp::Ordering;

use ahash::AHashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    pub const fn parse(ch: u8) -> Self {
        match ch {
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::T,
            b'J' => Self::J,
            b'Q' => Self::Q,
            b'K' => Self::K,
            b'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn parse(s: &str) -> Self {
        let mut card_count = AHashMap::<Card, u64>::new();
        for ch in s.bytes() {
            let card = Card::parse(ch);
            card_count
                .entry(card)
                .and_modify(|bid| *bid += 1)
                .or_insert(1);
        }

        let card_count = card_count.into_values().collect::<Vec<_>>();
        Self::from_card_count(card_count)
    }

    fn from_card_count(mut card_count: Vec<u64>) -> Self {
        card_count.sort();
        match card_count.as_slice() {
            [1, 1, 1, 1, 1] => Self::HighCard,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 3] => Self::ThreeOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 4] => Self::FourOfAKind,
            [5] => Self::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandBidPair<'a>(HandType, &'a str, u64);

impl<'a> HandBidPair<'a> {
    pub fn parse(s: &'a str) -> Self {
        let (hand, bid) = s.trim().split_once(' ').unwrap();
        let hand_type = HandType::parse(hand);
        let bid = bid.parse().unwrap();

        Self(hand_type, hand, bid)
    }
}

fn hand_cmp(hand_a: &str, hand_b: &str) -> Ordering {
    hand_a
        .bytes()
        .zip(hand_b.bytes())
        .fold(Ordering::Equal, |ordering, (card_a, card_b)| {
            let card_a = Card::parse(card_a);
            let card_b = Card::parse(card_b);

            ordering.then(card_a.cmp(&card_b))
        })
}

#[aoc_runner_derive::aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut hand_bid_pairs = input.lines().map(HandBidPair::parse).collect::<Vec<_>>();

    hand_bid_pairs.sort_by(|a, b| a.0.cmp(&b.0).then(hand_cmp(a.1, b.1)));
    hand_bid_pairs
        .into_iter()
        .enumerate()
        .map(|(rank, hand_bid_pair)| hand_bid_pair.2 * (1 + rank as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(super::solve_part1(input), 6440)
    }
}
