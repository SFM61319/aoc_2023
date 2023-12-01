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
            const ZERO: u8 = b'0';

            let text = line.as_bytes();
            let mut digits: Vec<u8> = Vec::with_capacity(line.len());

            for (i, ch) in text.iter().enumerate() {
                if ch.is_ascii_digit() {
                    digits.push(ch - ZERO);
                } else {
                    let curr = *ch;
                    let next1 = text.get(i + 1).copied();
                    let next2 = text.get(i + 2).copied();
                    let next3 = text.get(i + 3).copied();
                    let next4 = text.get(i + 4).copied();

                    if curr == b'o'
                        && next1.is_some()
                        && next2.is_some()
                        && next1.clone().unwrap() == b'n'
                        && next2.clone().unwrap() == b'e'
                    {
                        digits.push(1);
                    } else if curr == b't' && next1.is_some() && next2.is_some() {
                        let next1 = next1.clone().unwrap();
                        let next2 = next2.clone().unwrap();

                        if next1 == b'w' && next2 == b'o' {
                            digits.push(2);
                        } else if next3.is_some() && next4.is_some() {
                            let next3 = next3.clone().unwrap();
                            let next4 = next4.clone().unwrap();

                            if next1 == b'h' && next2 == b'r' && next3 == b'e' && next4 == b'e' {
                                digits.push(3);
                            }
                        }
                    } else if curr == b'f' && next1.is_some() && next2.is_some() && next3.is_some()
                    {
                        let next1 = next1.clone().unwrap();
                        let next2 = next2.clone().unwrap();
                        let next3 = next3.clone().unwrap();

                        if next1 == b'o' && next2 == b'u' && next3 == b'r' {
                            digits.push(4);
                        } else if next1 == b'i' && next2 == b'v' && next3 == b'e' {
                            digits.push(5);
                        }
                    } else if curr == b's' && next1.is_some() && next2.is_some() {
                        let next1 = next1.clone().unwrap();
                        let next2 = next2.clone().unwrap();

                        if next1 == b'i' && next2 == b'x' {
                            digits.push(6);
                        } else if next3.is_some() && next4.is_some() {
                            let next3 = next3.clone().unwrap();
                            let next4 = next4.clone().unwrap();

                            if next1 == b'e' && next2 == b'v' && next3 == b'e' && next4 == b'n' {
                                digits.push(7);
                            }
                        }
                    } else if curr == b'e'
                        && next1.is_some()
                        && next2.is_some()
                        && next3.is_some()
                        && next4.is_some()
                    {
                        let next1 = next1.clone().unwrap();
                        let next2 = next2.clone().unwrap();
                        let next3 = next3.clone().unwrap();
                        let next4 = next4.clone().unwrap();

                        if next1 == b'i' && next2 == b'g' && next3 == b'h' && next4 == b't' {
                            digits.push(8);
                        }
                    } else if curr == b'n' && next1.is_some() && next2.is_some() && next3.is_some()
                    {
                        let next1 = next1.clone().unwrap();
                        let next2 = next2.clone().unwrap();
                        let next3 = next3.clone().unwrap();

                        if next1 == b'i' && next2 == b'n' && next3 == b'e' {
                            digits.push(9);
                        }
                    }
                }
            }

            let digit1 = digits[0];
            let digit2 = digits[digits.len() - 1];
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
