#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Race {
    allowed_time: u32,
    distance_record: u32,
}

impl Race {
    #[inline]
    pub const fn new() -> Self {
        Self::with(u32::MIN, u32::MIN)
    }

    #[inline]
    pub const fn with(allowed_time: u32, distance_record: u32) -> Self {
        Self {
            allowed_time,
            distance_record,
        }
    }

    #[inline]
    pub fn get_number_of_ways_to_win(&self) -> u32 {
        let hold_times = u32::MIN..=self.allowed_time;
        hold_times
            .map(|hold_time| self.get_distance_after(hold_time))
            .filter(|&distance| self.can_win(distance))
            .count() as u32
    }

    #[inline]
    const fn get_distance_after(&self, hold_time: u32) -> u32 {
        let speed = hold_time;
        let remaining_time = self.allowed_time - hold_time;

        speed * remaining_time
    }

    #[inline]
    const fn can_win(&self, distance: u32) -> bool {
        distance > self.distance_record
    }
}

#[inline]
fn filter_numbers(numbers: &str) -> &str {
    numbers.split(':').last().unwrap().trim()
}

#[inline]
fn parse_numbers(numbers: &str) -> impl Iterator<Item = u32> + '_ {
    filter_numbers(numbers)
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
}

#[inline]
#[aoc_runner_derive::aoc(day6, part1)]
pub fn solve_part1(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(super::solve_part1(input), 288)
    }
}
