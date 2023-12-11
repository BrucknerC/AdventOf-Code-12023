const STRINGIFIED_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn retrieve_calibration_value_part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line_part1)
        .map(|(ten, one)| ten * 10 + one)
        .sum()
}

fn parse_line_part1(line: &str) -> (u32, u32) {
    let ten = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10))
        .unwrap()
        .unwrap();
    let one = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10))
        .unwrap()
        .unwrap();

    (ten, one)
}

pub fn retrieve_calibration_value_part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line_part2)
        .map(|(ten, one)| ten * 10 + one)
        .sum()
}

fn parse_line_part2(line: &str) -> (u32, u32) {
    let mut strings_to_be_matched = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    strings_to_be_matched.extend(STRINGIFIED_NUMBERS);

    let mut stringified_matches = strings_to_be_matched
        .into_iter()
        .flat_map(|stringified_number| line.match_indices(stringified_number))
        .collect::<Vec<(usize, &str)>>();

    stringified_matches.sort_by(|a, b| a.0.cmp(&b.0));

    (
        map_to_u32(stringified_matches[0].1),
        map_to_u32(stringified_matches.last().unwrap().1),
    )
}

fn map_to_u32(input: &str) -> u32 {
    match input.parse() {
        Ok(i) => i,
        Err(_) => {
            (STRINGIFIED_NUMBERS
                .into_iter()
                .position(|s| s == input)
                .unwrap()
                + 1) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, retrieve_calibration_value_part1(input));
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all calibration values is {}",
            retrieve_calibration_value_part1(input)
        );
    }

    #[test]
    fn part_2_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, retrieve_calibration_value_part2(input));
    }

    #[test]
    fn part_2() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all calibration values is {}",
            retrieve_calibration_value_part2(input)
        );
    }
}
