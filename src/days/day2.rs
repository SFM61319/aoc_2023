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

#[inline]
fn is_game_possible(game: &Game) -> bool {
    const RED: u32 = 12u32;
    const GREEN: u32 = 13u32;
    const BLUE: u32 = 14u32;

    game.sets
        .iter()
        .all(|game_set| game_set.red <= RED && game_set.green <= GREEN && game_set.blue <= BLUE)
}

#[inline]
#[aoc_runner_derive::aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|&game| is_game_possible(game))
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_part1_sample() {
        let input = super::generate_input(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(super::solve_part1(&input), 8)
    }
}
