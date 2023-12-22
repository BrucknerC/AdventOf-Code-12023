use itertools::Itertools;

struct Race {
    time: u32,
    distance_to_beat: u32,
}

impl From<(u32, u32)> for Race {
    fn from(value: (u32, u32)) -> Self {
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

pub fn get_number_of_ways_to_win(input: &str) -> Vec<usize> {
    parse_races(input)
        .iter()
        .map(Race::get_number_of_ways_to_win)
        .collect_vec()
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines: (&str, &str) = input.lines().collect_tuple().unwrap();
    let times = lines
        .0
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    let distances_to_beat = lines
        .1
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    times.zip(distances_to_beat).map(Race::from).collect_vec()
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
            get_number_of_ways_to_win(input).iter().product::<usize>()
        );
    }

    #[test]
    fn part_1() {
        let input = "Time:        47     84     74     67
Distance:   207   1394   1209   1014";
        println!(
            "The product of ways to win is {}",
            get_number_of_ways_to_win(input).iter().product::<usize>()
        );
    }

    /* #[test]
                fn part_2_example() {
                    let input = "Time:      7  15   30
    Distance:  9  40  200";
                    assert_eq!(30, count_copies_of_cards(input))
                } */

    /*     #[test]
        fn part_2() {
            let input = "Time:        47     84     74     67
    Distance:   207   1394   1209   1014";
            println!(
                "The sum of all scratch cards is {}",
                count_copies_of_cards(input)
            );
        } */
}
