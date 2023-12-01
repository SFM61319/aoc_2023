#[aoc_runner_derive::aoc(day1, part1, Iterators)]
pub fn solve_part1_iterators(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            const ZERO: u8 = b'0';
            let mut digits = line
                .bytes()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch - ZERO);

            let digit1 = digits.next().unwrap();
            let digit2 = digits.last().unwrap_or(digit1);
            let digits = digit1 * 10 + digit2;

            digits as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_iterators_sample() {
        assert_eq!(
            super::solve_part1_iterators("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"),
            142
        )
    }
}
