use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Card {
    winning_numbers: HashSet<u32>,
    current_numbers: HashSet<u32>,
}

impl Card {
    #[inline]
    pub fn new() -> Self {
        Self {
            winning_numbers: HashSet::new(),
            current_numbers: HashSet::new(),
        }
    }

    #[inline]
    pub fn parse_str(s: &str) -> Self {
        let mut card = Self::new();
        let mut numbers = s.split(':').last().unwrap().split('|');

        Card::parse_numbers(numbers.next().unwrap(), &mut card.winning_numbers);
        Card::parse_numbers(numbers.next().unwrap(), &mut card.current_numbers);

        card
    }

    #[inline]
    fn get_points(&self) -> u32 {
        const TWO: u32 = 2;
        let common_count = self
            .current_numbers
            .intersection(&self.winning_numbers)
            .count() as u32;

        if common_count == u32::MIN {
            common_count
        } else {
            TWO.pow(common_count - 1)
        }
    }

    #[inline]
    fn parse_numbers(numbers: &str, number_set: &mut HashSet<u32>) {
        let numbers = numbers
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap());

        number_set.extend(numbers);
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
}
