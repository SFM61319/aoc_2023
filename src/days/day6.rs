#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Race {
    allowed_time: u64,
    distance_record: u64,
}

impl Race {
    #[inline]
    pub const fn new() -> Self {
        Self::with(u64::MIN, u64::MIN)
    }

    #[inline]
    pub const fn with(allowed_time: u64, distance_record: u64) -> Self {
        Self {
            allowed_time,
            distance_record,
        }
    }

    #[inline]
    pub fn get_number_of_ways_to_win(&self) -> u64 {
        // hold_time * (allowed_time - hold_time) < distance_record
        // =>  x * (t - x) < d
        // => tx - x^2 < d
        // => tx - x^2 - d < 0
        // => -tx + x^2 + d > 0
        // => x^2 - tx + d > 0
        // a = 1, b = -t, c = d
        // disc = b^2 - 4ac = (-t)^2 - 4(1)(d) = t^2 - 4d
        // x = (-b +- sqrt(disc)) / 2a = (t +- sqrt(disc)) / 2

        let disc = self.allowed_time.pow(2) - 4 * self.distance_record;
        let disc = (disc as f64).sqrt() - 2.0f64;

        let x1 = (self.allowed_time as f64 + disc) / 2.0f64;
        let x2 = (self.allowed_time as f64 - disc) / 2.0f64;

        let x1 = x1.ceil() as u64;
        let x2 = x2.floor() as u64;

        x1 - x2 + 1
    }
}

#[inline]
fn filter_numbers(numbers: &str) -> &str {
    numbers.split(':').last().unwrap().trim()
}

#[inline]
fn parse_numbers(numbers: &str) -> impl Iterator<Item = u64> + '_ {
    filter_numbers(numbers)
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
}

#[aoc_runner_derive::aoc(day6, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut input = input.lines();

    let allowed_times = input.next().unwrap();
    let distance_records = input.next().unwrap();

    let allowed_times = parse_numbers(allowed_times);
    let distance_records = parse_numbers(distance_records);

    allowed_times
        .zip(distance_records)
        .map(|(allowed_time, distance_record)| {
            let race = Race::with(allowed_time, distance_record);
            race.get_number_of_ways_to_win()
        })
        .product()
}

#[inline]
fn parse_number(numbers: &str) -> u64 {
    filter_numbers(numbers).replace(' ', "").parse().unwrap()
}

#[aoc_runner_derive::aoc(day6, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let mut input = input.lines();

    let allowed_time = input.next().unwrap();
    let distance_record = input.next().unwrap();

    let allowed_time = parse_number(allowed_time);
    let distance_record = parse_number(distance_record);

    let race = Race::with(allowed_time, distance_record);
    race.get_number_of_ways_to_win()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(super::solve_part1(input), 288)
    }

    #[test]
    fn test_solve_part2_sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(super::solve_part2(input), 71503)
    }
}
