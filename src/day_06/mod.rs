use itertools::Itertools;

struct Race {
    time: u64,
    distance_to_beat: u64,
}

impl From<(u64, u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
        Race {
            time: value.0,
            distance_to_beat: (value.1),
        }
    }
}

impl Race {
    fn get_number_of_ways_to_win(&self) -> usize {
        let mut possibilities = Vec::new();
        for i in 0..self.time {
            possibilities.push((i, self.time - i));
        }

        possibilities
            .iter()
            .map(|(velocity, remaining_time)| velocity * remaining_time)
            .filter(|&distance_traveled| distance_traveled > self.distance_to_beat)
            .count()
    }
}

pub fn get_number_of_ways_to_win_part1(input: &str) -> Vec<usize> {
    parse_races_part1(input)
        .iter()
        .map(Race::get_number_of_ways_to_win)
        .collect_vec()
}

fn parse_races_part1(input: &str) -> Vec<Race> {
    let lines: (&str, &str) = input.lines().collect_tuple().unwrap();
    let times = lines
        .0
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let distances_to_beat = lines
        .1
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    times.zip(distances_to_beat).map(Race::from).collect_vec()
}

pub fn get_number_of_ways_to_win_part2(input: &str) -> usize {
    parse_races_part2(input).get_number_of_ways_to_win()
}

fn parse_races_part2(input: &str) -> Race {
    let lines: (&str, &str) = input.lines().collect_tuple().unwrap();
    let time = lines
        .0
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance_to_beat = lines
        .1
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    Race::from((time, distance_to_beat))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(
            288,
            get_number_of_ways_to_win_part1(input)
                .iter()
                .product::<usize>()
        );
    }

    #[test]
    fn part_1() {
        let input = "Time:        47     84     74     67
Distance:   207   1394   1209   1014";
        println!(
            "The product of ways to win is {}",
            get_number_of_ways_to_win_part1(input)
                .iter()
                .product::<usize>()
        );
    }

    #[test]
    fn part_2_example() {
        let input = "Time:      7  15   30
    Distance:  9  40  200";
        assert_eq!(71503, get_number_of_ways_to_win_part2(input))
    }

    #[test]
    fn part_2() {
        let input = "Time:        47     84     74     67
    Distance:   207   1394   1209   1014";
        println!(
            "The number of ways to win is {}",
            get_number_of_ways_to_win_part2(input)
        );
    }
}
