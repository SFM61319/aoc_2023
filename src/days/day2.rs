#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl GameSet {
    #[inline]
    pub const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

fn parse_set(set: &str) -> GameSet {
    const COLOR_SEPARATOR: char = ',';
    const COLOR_COUNT_SEPARATOR: char = ' ';

    let mut red = u32::MIN;
    let mut green = u32::MIN;
    let mut blue = u32::MIN;

    set.split(COLOR_SEPARATOR).for_each(|color_count| {
        let mut color_count = color_count.trim().split(COLOR_COUNT_SEPARATOR);

        let count = color_count.next().unwrap().parse::<u32>().unwrap();
        let color = color_count.last().unwrap();

        match color {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => unreachable!(),
        }
    });
    GameSet::new(red, green, blue)
}

#[inline]
fn parse_all_sets(line: &str) -> Vec<GameSet> {
    const KEY_VALUE_SEPARATOR: char = ':';
    const SET_SEPARATOR: char = ';';

    line.split(KEY_VALUE_SEPARATOR)
        .last()
        .unwrap()
        .split(SET_SEPARATOR)
        .map(parse_set)
        .collect()
}

#[aoc_runner_derive::aoc_generator(day2)]
pub fn generate_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let id = (id + 1) as u32;
            let sets = parse_all_sets(line);

            Game { id, sets }
        })
        .collect()
}
