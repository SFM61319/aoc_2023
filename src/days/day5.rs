use std::str::Lines;

use ahash::AHashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: AHashMap<u32, u32>,
    soil_to_fertilizer: AHashMap<u32, u32>,
    fertilizer_to_water: AHashMap<u32, u32>,
    water_to_light: AHashMap<u32, u32>,
    light_to_temperature: AHashMap<u32, u32>,
    temperature_to_humidity: AHashMap<u32, u32>,
    humidity_to_location: AHashMap<u32, u32>,
}

impl Almanac {
    #[inline]
    pub fn new() -> Self {
        Self::with(
            Vec::new(),
            AHashMap::new(),
            AHashMap::new(),
            AHashMap::new(),
            AHashMap::new(),
            AHashMap::new(),
            AHashMap::new(),
            AHashMap::new(),
        )
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn with(
        seeds: Vec<u32>,
        seed_to_soil: AHashMap<u32, u32>,
        soil_to_fertilizer: AHashMap<u32, u32>,
        fertilizer_to_water: AHashMap<u32, u32>,
        water_to_light: AHashMap<u32, u32>,
        light_to_temperature: AHashMap<u32, u32>,
        temperature_to_humidity: AHashMap<u32, u32>,
        humidity_to_location: AHashMap<u32, u32>,
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
    fn parse_seeds(seeds: &str) -> Vec<u32> {
        seeds
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    }

    fn parse_maps(lines: &mut Lines) -> AHashMap<u32, u32> {
        let mut map = AHashMap::<u32, u32>::new();
        for line in lines.take_while(|&line| !line.is_empty()) {
            let (dest_range_start, src_range_start, range_len) = Self::parse_range(line);
            for i in u32::MIN..range_len {
                map.insert(src_range_start + i, dest_range_start + i);
            }
        }

        map
    }

    #[inline]
    fn parse_range(range: &str) -> (u32, u32, u32) {
        let range = range
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<u32>>();

        (range[0], range[1], range[2])
    }

    fn get_nearest_location(&self) -> u32 {
        self.seeds
            .iter()
            .map(|&seed| self.get_seed_location(seed))
            .min()
            .unwrap()
    }

    fn get_seed_location(&self, seed: u32) -> u32 {
        let soil = Self::get_map_value_or(&self.seed_to_soil, seed);
        let fertilizer = Self::get_map_value_or(&self.soil_to_fertilizer, soil);
        let water = Self::get_map_value_or(&self.fertilizer_to_water, fertilizer);
        let light = Self::get_map_value_or(&self.water_to_light, water);
        let temperature = Self::get_map_value_or(&self.light_to_temperature, light);
        let humidity = Self::get_map_value_or(&self.temperature_to_humidity, temperature);

        Self::get_map_value_or(&self.humidity_to_location, humidity)
    }

    #[inline]
    fn get_map_value_or(hash_map: &AHashMap<u32, u32>, key_and_default: u32) -> u32 {
        hash_map
            .get(&key_and_default)
            .copied()
            .unwrap_or(key_and_default)
    }
}

#[inline]
#[aoc_runner_derive::aoc_generator(day5)]
pub fn generate_input(input: &str) -> Almanac {
    Almanac::parse_str(input)
}

#[inline]
#[aoc_runner_derive::aoc(day5, part1)]
pub fn solve_part1(input: &Almanac) -> u32 {
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
