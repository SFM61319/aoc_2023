#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Card {
    common_numbers: u128,
}

impl Card {
    #[inline]
    pub const fn new() -> Self {
        Self::with_common_numbers(u128::MIN)
    }

    #[inline]
    pub const fn with_common_numbers(common_numbers: u128) -> Self {
        Self { common_numbers }
    }

    #[inline]
    pub fn parse_str(s: &str) -> Self {
        let mut numbers = s.split(':').last().unwrap().split('|');

        let winning_numbers = Card::parse_numbers(numbers.next().unwrap());
        let current_numbers = Card::parse_numbers(numbers.next().unwrap());

        Self::with_common_numbers(winning_numbers & current_numbers)
    }

    #[inline]
    const fn count_common_numbers(&self) -> u32 {
        self.common_numbers.count_ones()
    }

    #[inline]
    const fn get_points(&self) -> u32 {
        (1 << self.count_common_numbers()) >> 1
    }

    #[inline]
    fn parse_numbers(numbers: &str) -> u128 {
        let mut number_set = u128::MIN;
        numbers
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .for_each(|num| number_set |= 1 << num);

        number_set
    }
}

#[inline]
#[aoc_runner_derive::aoc_generator(day4)]
pub fn generate_input(input: &str) -> Vec<Card> {
    input.lines().map(Card::parse_str).collect()
}

#[inline]
#[aoc_runner_derive::aoc(day4, part1)]
pub fn solve_part1(input: &[Card]) -> u32 {
    input.iter().map(Card::get_points).sum()
}

fn scratch_card(card_no: usize, card: &Card, card_counts: &mut [u32]) {
    let next_cards_count = card.count_common_numbers() as usize;
    for i in usize::MIN..next_cards_count {
        card_counts[card_no + i + 1] += card_counts[card_no];
    }
}

#[aoc_runner_derive::aoc(day4, part2)]
pub fn solve_part2(input: &[Card]) -> u32 {
    let mut card_counts = vec![1u32; input.len()];
    input
        .iter()
        .enumerate()
        .for_each(|(card_no, card)| scratch_card(card_no, card, &mut card_counts));

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = super::generate_input(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(super::solve_part1(&input), 13)
    }

    #[test]
    fn test_solve_part2_sample() {
        let input = super::generate_input(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(super::solve_part2(&input), 30)
    }
}
