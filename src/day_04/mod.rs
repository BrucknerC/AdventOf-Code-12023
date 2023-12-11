use itertools::Itertools;

#[allow(dead_code)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

impl Card {
    fn get_score(self) -> u32 {
        self.scratched_numbers
            .iter()
            .filter(|&scratched_number| self.winning_numbers.contains(scratched_number))
            .fold(1, |acc, _| acc << 1)
            >> 1
    }
}

pub fn get_points_from_cards(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(parse_line)
        .map(Card::get_score)
        .collect_vec()
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

    Card {
        id: game_id.parse().unwrap(),
        winning_numbers,
        scratched_numbers,
    }
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
    #[ignore]
    fn part_2_example() {
        let input = "";
        //assert_eq!(467835, get_gear_ratios(input).iter().sum::<u32>())
    }

    #[test]
    #[ignore]
    fn part_2() {
        let input = include_str!("input.txt");
        /*println!(
            "The sum of all gear ratios is {}",
            get_gear_ratios(input).iter().sum::<u32>()
        );*/
    }
}
