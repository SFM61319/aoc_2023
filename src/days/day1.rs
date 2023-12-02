#[aoc_runner_derive::aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
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
            let digits = digit1 * 10u8 + digit2;

            digits as u32
        })
        .sum()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digit1 = Option::<u8>::None;
            let mut digit2 = Option::<u8>::None;

            for i in usize::MIN..line.len() {
                let slice = &line[i..];
                let digit = match slice {
                    one if one.starts_with("one") => Some(1u8),
                    two if two.starts_with("two") => Some(2u8),
                    three if three.starts_with("three") => Some(3u8),
                    four if four.starts_with("four") => Some(4u8),
                    five if five.starts_with("five") => Some(5u8),
                    six if six.starts_with("six") => Some(6u8),
                    seven if seven.starts_with("seven") => Some(7u8),
                    eight if eight.starts_with("eight") => Some(8u8),
                    nine if nine.starts_with("nine") => Some(9u8),
                    _ => {
                        let digit = slice.bytes().next().unwrap();
                        match digit {
                            b'0'..=b'9' => Some(digit),
                            _ => None,
                        }
                    }
                };

                if digit.is_some() {
                    if digit1.is_none() {
                        digit1 = digit;
                    }

                    digit2 = digit;
                }
            }

            let digit1 = digit1.unwrap();
            let digit2 = digit2.unwrap();
            let digits = digit1 * 10u8 + digit2;

            digits as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        assert_eq!(
            super::solve_part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"),
            142
        )
    }

    #[test]
    fn test_solve_part2_sample() {
        assert_eq!(
            super::solve_part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"),
            281
        )
    }
}
