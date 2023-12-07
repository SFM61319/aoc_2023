use std::array;

const fn evaluate_card(card: u8, has_joker: bool) -> u8 {
    match card {
        b'2'..=b'9' => card - b'0',
        b'T' => 10,
        b'J' => {
            if has_joker {
                1
            } else {
                11
            }
        }
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => unreachable!(),
    }
}

fn count_cards(cards: [u8; 5]) -> (usize, u64) {
    let mut counts = [(u8::MIN, u64::MIN); 5];
    let mut unique_count = 0;
    let mut joker_count = 0;

    'card: for card in cards {
        if card == 1 {
            joker_count += 1;
            continue;
        }

        for (counted_card, card_count) in counts.iter_mut() {
            if *counted_card == card {
                *card_count += 1;
                continue 'card;
            }
        }

        counts[unique_count] = (card, 1);
        unique_count += 1;
    }

    let largest_count = counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .1;
    (unique_count, largest_count + joker_count)
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
    fn from_cards(cards: [u8; 5]) -> Self {
        match count_cards(cards) {
            (5, 1) => Self::HighCard,
            (4, 2) => Self::OnePair,
            (3, 2) => Self::TwoPair,
            (3, 3) => Self::ThreeOfAKind,
            (2, 3) => Self::FullHouse,
            (2, 4) => Self::FourOfAKind,
            (0 | 1, 5) => Self::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bid: u64,
}

impl Hand {
    #[inline]
    pub const fn with(hand_type: HandType, cards: [u8; 5], bid: u64) -> Self {
        Self {
            hand_type,
            cards,
            bid,
        }
    }

    pub fn parse_all(input: &str, has_joker: bool) -> Vec<Self> {
        input
            .lines()
            .map(|line| Hand::parse(line, has_joker))
            .collect()
    }

    pub fn parse(s: &str, has_joker: bool) -> Self {
        let (cards, bid) = s.trim().split_once(' ').unwrap();
        let cards = cards.as_bytes();
        let bid = bid.parse().unwrap();

        let cards = array::from_fn(|i| evaluate_card(cards[i], has_joker));
        let hand_type = HandType::from_cards(cards);

        Self::with(hand_type, cards, bid)
    }
}

fn get_winnings(hands: &mut [Hand]) -> u64 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (1 + rank as u64))
        .sum()
}

#[aoc_runner_derive::aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut hands = Hand::parse_all(input, false);
    get_winnings(&mut hands)
}

#[aoc_runner_derive::aoc(day7, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let mut hands = Hand::parse_all(input, true);
    get_winnings(&mut hands)
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

    #[test]
    fn test_solve_part2_sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(super::solve_part2(input), 5905)
    }
}
