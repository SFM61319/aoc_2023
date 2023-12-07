use std::cmp::Ordering;

#[inline]
fn card_cmp(card_a: u8, card_b: u8, ordering: &[u8; 35]) -> Ordering {
    ordering[card_a as usize].cmp(&ordering[card_b as usize])
}

#[inline]
fn hand_cmp(hand_a: &str, hand_b: &str, card_ordering: &[u8; 35]) -> Ordering {
    hand_a
        .bytes()
        .zip(hand_b.bytes())
        .fold(Ordering::Equal, |ordering, (card_a, card_b)| {
            ordering.then_with(|| card_cmp(card_a - b'2', card_b - b'2', card_ordering))
        })
}

#[inline]
fn hand_bid_pair_cmp(
    pair_a: &([u8; 35], &str, u64),
    pair_b: &([u8; 35], &str, u64),
    card_ordering: &[u8; 35],
) -> Ordering {
    pair_a
        .0
        .cmp(&pair_b.0)
        .then_with(|| hand_cmp(pair_a.1, pair_b.1, card_ordering))
}

#[aoc_runner_derive::aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u64 {
    const ORDERING: [u8; 35] = [
        1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 10, 12, 0, 0, 0,
        0, 0, 11, 0, 0, 9,
    ];
    let mut hand_bid_pairs = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(' ').unwrap();
            let bid = bid.parse().unwrap();

            let mut card_count = [u8::MIN; 35];
            for ch in hand.bytes() {
                let i = (ch - b'2') as usize;
                card_count[i] += 1;
            }

            card_count.sort();
            card_count.reverse();

            (card_count, hand, bid)
        })
        .collect::<Vec<([u8; 35], &str, u64)>>();

    hand_bid_pairs.sort_by(|pair_a, pair_b| hand_bid_pair_cmp(pair_a, pair_b, &ORDERING));
    hand_bid_pairs
        .into_iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| bid * (1 + rank as u64))
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
