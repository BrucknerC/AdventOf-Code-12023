fn retrieve_calibration_value(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(|(ten, one)| ten * 10 + one)
        .sum()
}

fn parse_line(line: &str) -> (u32, u32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, retrieve_calibration_value(input));
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all calibration values is {}",
            retrieve_calibration_value(input)
        );
    }
}
