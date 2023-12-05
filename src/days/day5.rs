use std::{ops::Range, str::Lines};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SrcDstRangeMap {
    src_range: Range<i64>,
    dst_range: Range<i64>,
}

impl SrcDstRangeMap {
    #[inline]
    pub fn new() -> Self {
        Self::with(Range::default(), Range::default())
    }

    #[inline]
    pub const fn with(src_range: Range<i64>, dst_range: Range<i64>) -> Self {
        Self {
            src_range,
            dst_range,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Vec<SrcDstRangeMap>,
    soil_to_fertilizer: Vec<SrcDstRangeMap>,
    fertilizer_to_water: Vec<SrcDstRangeMap>,
    water_to_light: Vec<SrcDstRangeMap>,
    light_to_temperature: Vec<SrcDstRangeMap>,
    temperature_to_humidity: Vec<SrcDstRangeMap>,
    humidity_to_location: Vec<SrcDstRangeMap>,
}

impl Almanac {
    #[inline]
    pub fn new() -> Self {
        Self::with(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn with(
        seeds: Vec<i64>,
        seed_to_soil: Vec<SrcDstRangeMap>,
        soil_to_fertilizer: Vec<SrcDstRangeMap>,
        fertilizer_to_water: Vec<SrcDstRangeMap>,
        water_to_light: Vec<SrcDstRangeMap>,
        light_to_temperature: Vec<SrcDstRangeMap>,
        temperature_to_humidity: Vec<SrcDstRangeMap>,
        humidity_to_location: Vec<SrcDstRangeMap>,
    ) -> Self {
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    pub fn parse_str(s: &str) -> Self {
        let mut lines = s.lines();

        let seeds = lines.next().unwrap();
        let seeds = Self::parse_seeds(seeds);

        _ = lines.nth(1);
        let seed_to_soil = Self::parse_maps(&mut lines);

        _ = lines.nth(1);
        let soil_to_fertilizer = Self::parse_maps(&mut lines);

        _ = lines.nth(1);
        let fertilizer_to_water = Self::parse_maps(&mut lines);

        _ = lines.nth(1);
        let water_to_light = Self::parse_maps(&mut lines);

        _ = lines.nth(1);
        let light_to_temperature = Self::parse_maps(&mut lines);

        _ = lines.nth(1);
        let temperature_to_humidity = Self::parse_maps(&mut lines);

        _ = lines.nth(1);
        let humidity_to_location = Self::parse_maps(&mut lines);

        Self::with(
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        )
    }

    #[inline]
    fn parse_seeds(seeds: &str) -> Vec<i64> {
        seeds
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    }

    fn parse_maps(lines: &mut Lines) -> Vec<SrcDstRangeMap> {
        let mut maps = Vec::new();
        for line in lines.take_while(|&line| !line.is_empty()) {
            let (src_range_start, dst_range_start, range_len) = Self::parse_range(line);

            let src_range_end = src_range_start + range_len;
            let dst_range_end = dst_range_start + range_len;

            let src_range = src_range_start..src_range_end;
            let dst_range = dst_range_start..dst_range_end;

            let map = SrcDstRangeMap::with(src_range, dst_range);
            maps.push(map);
        }

        maps
    }

    #[inline]
    fn parse_range(range: &str) -> (i64, i64, i64) {
        let range = range
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i64>>();

        (range[1], range[0], range[2])
    }

    fn get_nearest_location(&self) -> i64 {
        self.seeds
            .iter()
            .map(|&seed| self.get_seed_location(seed))
            .min()
            .unwrap()
    }

    fn get_seed_location(&self, seed: i64) -> i64 {
        let soil = Self::get_map_value_or(&self.seed_to_soil, seed);
        let fertilizer = Self::get_map_value_or(&self.soil_to_fertilizer, soil);
        let water = Self::get_map_value_or(&self.fertilizer_to_water, fertilizer);
        let light = Self::get_map_value_or(&self.water_to_light, water);
        let temperature = Self::get_map_value_or(&self.light_to_temperature, light);
        let humidity = Self::get_map_value_or(&self.temperature_to_humidity, temperature);

        Self::get_map_value_or(&self.humidity_to_location, humidity)
    }

    #[inline]
    fn get_map_value_or(maps: &[SrcDstRangeMap], key_and_default: i64) -> i64 {
        maps.iter()
            .find(|&map| map.src_range.contains(&key_and_default))
            .map_or(key_and_default, |map| {
                Self::get_map_value(map, key_and_default)
            })
    }

    #[inline]
    const fn get_map_value(map: &SrcDstRangeMap, key_and_default: i64) -> i64 {
        key_and_default + map.dst_range.start - map.src_range.start
    }
}

#[inline]
#[aoc_runner_derive::aoc_generator(day5)]
pub fn generate_input(input: &str) -> Almanac {
    Almanac::parse_str(input)
}

#[inline]
#[aoc_runner_derive::aoc(day5, part1)]
pub fn solve_part1(input: &Almanac) -> i64 {
    input.get_nearest_location()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = super::generate_input(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(super::solve_part1(&input), 35)
    }
}
