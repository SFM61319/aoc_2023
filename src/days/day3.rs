fn get_empty_parts(part_count: usize) -> String {
    let mut empty_parts = String::with_capacity(part_count);
    for _ in usize::MIN..part_count {
        empty_parts.push('.');
    }

    empty_parts
}

#[inline]
fn get_line_or<'a>(lines: &'a [&'a str], line_no: usize, default: &'a str) -> &'a [u8] {
    lines.get(line_no).copied().unwrap_or(default).as_bytes()
}

#[inline]
fn is_part(ch: u8) -> bool {
    ch != b'.' && !(b'0'..=b'9').contains(&ch)
}

fn get_part_numbers(lines: &[&str], line_no: usize, curr_line: &str) -> u32 {
    let curr_line = curr_line.as_bytes();
    let empty_parts = get_empty_parts(curr_line.len());

    let next_line = get_line_or(lines, line_no + 1, &empty_parts);
    let prev_line = if line_no == usize::MIN {
        &empty_parts.as_bytes()
    } else {
        get_line_or(lines, line_no - 1, &empty_parts)
    };

    let mut curr_number = u32::MIN;
    let mut is_curr_part = false;

    let mut parts = u32::MIN;
    for (i, ch) in curr_line.iter().enumerate() {
        if ch.is_ascii_digit() {
            curr_number = curr_number * 10u32 + (ch - b'0') as u32;
            if !is_curr_part {
                if let Some(&top_char) = prev_line.get(i) {
                    is_curr_part = is_part(top_char);
                }
                if is_curr_part {
                    continue;
                }
                if let Some(&top_right_char) = prev_line.get(i + 1) {
                    is_curr_part = is_part(top_right_char);
                }
                if is_curr_part {
                    continue;
                }
                if let Some(&right_char) = curr_line.get(i + 1) {
                    is_curr_part = is_part(right_char);
                }
                if is_curr_part {
                    continue;
                }
                if let Some(&bottom_right_char) = next_line.get(i + 1) {
                    is_curr_part = is_part(bottom_right_char);
                }
                if is_curr_part {
                    continue;
                }
                if let Some(&bottom_char) = next_line.get(i) {
                    is_curr_part = is_part(bottom_char);
                }
                if is_curr_part {
                    continue;
                }
                if i > usize::MIN {
                    if let Some(&bottom_left_char) = next_line.get(i - 1) {
                        is_curr_part = is_part(bottom_left_char);
                    }
                    if is_curr_part {
                        continue;
                    }
                    if let Some(&left_char) = curr_line.get(i - 1) {
                        is_curr_part = is_part(left_char);
                    }
                    if is_curr_part {
                        continue;
                    }
                    if let Some(&top_left_char) = prev_line.get(i - 1) {
                        is_curr_part = is_part(top_left_char);
                    }
                    if is_curr_part {
                        continue;
                    }
                }
            }
        } else {
            if is_curr_part {
                parts += curr_number;
            }

            curr_number = u32::MIN;
            is_curr_part = false;
        }
    }

    if is_curr_part {
        parts += curr_number;
    }
    parts
}

#[aoc_runner_derive::aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .enumerate()
        .map(|(line_no, &line)| get_part_numbers(&lines, line_no, line))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(super::solve_part1(input), 4361)
    }
}
