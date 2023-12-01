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

#[aoc_runner_derive::aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let mut digit1 = Option::<u8>::None;
            let mut digit2 = Option::<u8>::None;

            for (i, ch) in line.iter().enumerate() {
                let digit = match (
                    ch,
                    line.get(i + 1),
                    line.get(i + 2),
                    line.get(i + 3),
                    line.get(i + 4),
                ) {
                    (b'0'..=b'9', _, _, _, _) => Some(ch - b'0'),
                    (b'o', Some(b'n'), Some(b'e'), _, _) => Some(1),
                    (b't', Some(b'w'), Some(b'o'), _, _) => Some(2),
                    (b't', Some(b'h'), Some(b'r'), Some(b'e'), Some(b'e')) => Some(3),
                    (b'f', Some(b'o'), Some(b'u'), Some(b'r'), _) => Some(4),
                    (b'f', Some(b'i'), Some(b'v'), Some(b'e'), _) => Some(5),
                    (b's', Some(b'i'), Some(b'x'), _, _) => Some(6),
                    (b's', Some(b'e'), Some(b'v'), Some(b'e'), Some(b'n')) => Some(7),
                    (b'e', Some(b'i'), Some(b'g'), Some(b'h'), Some(b't')) => Some(8),
                    (b'n', Some(b'i'), Some(b'n'), Some(b'e'), _) => Some(9),
                    _ => None,
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

    #[test]
    fn test_solve_part2_sample() {
        assert_eq!(
            super::solve_part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"),
            281
        )
    }
}
