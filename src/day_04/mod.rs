use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

#[derive(Eq)]
struct Card {
    id: u32,
    win_count: u32,
}

impl Card {
    fn new(id: u32, winning_numbers: Vec<u32>, scratched_numbers: Vec<u32>) -> Self {
        Card {
            id,
            win_count: scratched_numbers
                .iter()
                .filter(|&scratched_number| winning_numbers.contains(scratched_number))
                .count() as u32,
        }
    }

    fn get_score(self) -> u32 {
        2u32.pow(self.win_count) / 2
    }
}

impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn get_points_from_cards(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(parse_line)
        .map(Card::get_score)
        .collect_vec()
}

pub fn count_copies_of_cards(input: &str) -> u32 {
    let cards = input.lines().map(parse_line).collect_vec();
    let mut copies_per_card: HashMap<&Card, u32> = HashMap::new();

    for card in cards.iter() {
        copies_per_card.insert(card, 1);
    }

    for card in cards.iter() {
        for card_to_update in cards
            .iter()
            .skip(card.id as usize)
            .take(card.win_count as usize)
        {
            *copies_per_card.entry(card_to_update).or_insert(1) +=
                *copies_per_card.entry(card).or_default();
        }
    }

    copies_per_card.values().sum::<u32>()
}

fn parse_line(line: &str) -> Card {
    let split_by_colon = line.split(':').collect_tuple::<(&str, &str)>().unwrap();
    let game_id: &str = split_by_colon
        .0
        .split_ascii_whitespace()
        .collect_tuple::<(&str, &str)>()
        .unwrap()
        .1;

    let (winning_numbers, scratched_numbers) = parse_numbers(split_by_colon.1);

    Card::new(game_id.parse().unwrap(), winning_numbers, scratched_numbers)
}

fn parse_numbers(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (winning_numbers, scratched_numbers) =
        line.split('|').collect_tuple::<(&str, &str)>().unwrap();
    (
        winning_numbers
            .split_ascii_whitespace()
            .filter_map(|number| number.parse().ok())
            .collect_vec(),
        scratched_numbers
            .split_ascii_whitespace()
            .filter_map(|number| number.parse().ok())
            .collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, get_points_from_cards(input).iter().sum::<u32>());
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all points is {}",
            get_points_from_cards(input).iter().sum::<u32>()
        );
    }

    #[test]
    fn part_2_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, count_copies_of_cards(input))
    }

    #[test]
    fn part_2() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all scratch cards is {}",
            count_copies_of_cards(input)
        );
    }
}
