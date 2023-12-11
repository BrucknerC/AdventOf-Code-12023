use itertools::Itertools;

pub const RGB_DICE_COUNTS: DiceCombination = DiceCombination {
    red: 12,
    green: 13,
    blue: 14,
};

struct Game {
    id: u32,
    samples: Vec<DiceCombination>,
}

impl Game {
    fn is_valid(&self, dice_in_bag: &DiceCombination) -> bool {
        self.samples.iter().all(|sample| {
            sample.red <= dice_in_bag.red
                && sample.green <= dice_in_bag.green
                && sample.blue <= dice_in_bag.blue
        })
    }

    fn get_minimal_set(self) -> DiceCombination {
        let mut minimal_set = DiceCombination::new();

        for sample in self.samples {
            minimal_set.red = minimal_set.red.max(sample.red);
            minimal_set.green = minimal_set.green.max(sample.green);
            minimal_set.blue = minimal_set.blue.max(sample.blue);
        }

        minimal_set
    }
}

pub struct DiceCombination {
    red: u32,
    green: u32,
    blue: u32,
}

impl DiceCombination {
    fn new() -> Self {
        DiceCombination {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub fn get_ids_of_possible_games(input: &str, dice_in_bag: DiceCombination) -> Vec<u32> {
    input
        .lines()
        .map(parse_line)
        .filter(|game| game.is_valid(&dice_in_bag))
        .map(|game| game.id)
        .collect_vec()
}

pub fn get_power_of_minimum_dice_sets_per_game(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(parse_line)
        .map(Game::get_minimal_set)
        .map(|minimal_set| minimal_set.red * minimal_set.green * minimal_set.blue)
        .collect_vec()
}

fn parse_line(line: &str) -> Game {
    let split_by_colon = line.split(':').collect_tuple::<(&str, &str)>().unwrap();
    let game_id: &str = split_by_colon
        .0
        .split_ascii_whitespace()
        .collect_tuple::<(&str, &str)>()
        .unwrap()
        .1;

    Game {
        id: game_id.parse().unwrap(),
        samples: parse_samples(split_by_colon.1),
    }
}

fn parse_samples(split_by_colon: &str) -> Vec<DiceCombination> {
    split_by_colon
        .split(';')
        .map(|sample| sample.trim())
        .map(parse_sample)
        .collect_vec()
}

fn parse_sample(samples: &str) -> DiceCombination {
    let samples = samples
        .split(',')
        .map(|sample| sample.trim())
        .map(|sample| {
            sample
                .split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .collect_vec();

    let mut combination = DiceCombination::new();

    for (count, color) in samples {
        match color {
            "red" => combination.red = count.parse().unwrap(),
            "green" => combination.green = count.parse().unwrap(),
            "blue" => combination.blue = count.parse().unwrap(),
            _ => panic!(),
        }
    }

    combination
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            8,
            get_ids_of_possible_games(input, RGB_DICE_COUNTS)
                .iter()
                .sum::<u32>()
        );
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all game IDs is {}",
            get_ids_of_possible_games(input, RGB_DICE_COUNTS)
                .iter()
                .sum::<u32>()
        );
    }

    #[test]
    fn part_2_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            2286,
            get_power_of_minimum_dice_sets_per_game(input)
                .iter()
                .sum::<u32>()
        );
    }

    #[test]
    fn part_2() {
        let input = include_str!("input.txt");
        println!(
            "The sum of powers of minimal sets of all games is {}",
            get_power_of_minimum_dice_sets_per_game(input)
                .iter()
                .sum::<u32>()
        );
    }
}
