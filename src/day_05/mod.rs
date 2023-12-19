use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

type Seed = u64;
type Soil = u64;
type Fertilizer = u64;
type Water = u64;
type Light = u64;
type Temperature = u64;
type Humidity = u64;
type Location = u64;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Numbers {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
}

#[derive(Debug)]
enum Maps {
    SeedToSoil(HashMap<Range<Seed>, Range<Soil>>),
    SoilToFertilizer(HashMap<Range<Soil>, Range<Fertilizer>>),
    FertilizerToWater(HashMap<Range<Fertilizer>, Range<Water>>),
    WaterToLight(HashMap<Range<Water>, Range<Light>>),
    LightToTemperature(HashMap<Range<Light>, Range<Temperature>>),
    TemperatureToHumidity(HashMap<Range<Temperature>, Range<Humidity>>),
    HumidityToLocation(HashMap<Range<Humidity>, Range<Location>>),
}

pub fn get_lowest_location_per_seed(input: &str) -> HashMap<Seed, Location> {
    let mut result: HashMap<Seed, Location> = HashMap::new();
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next().unwrap();
    let maps = parse_to_maps(blocks.collect_vec());
    let chain = vec![
        Numbers::Seed,
        Numbers::Soil,
        Numbers::Fertilizer,
        Numbers::Water,
        Numbers::Light,
        Numbers::Temperature,
        Numbers::Humidity,
    ];

    for seed in seeds
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<Seed>().unwrap())
    {
        let mut temp = seed;
        for number in chain.iter() {
            temp = map_source_to_destination(maps.get(number).unwrap(), temp);
        }

        result.insert(seed, temp);
    }

    result
}

fn map_source_to_destination(map: &Maps, source_value: u64) -> u64 {
    match map {
        Maps::SeedToSoil(maps) => determine_destination(maps, source_value),
        Maps::SoilToFertilizer(maps) => determine_destination(maps, source_value),
        Maps::FertilizerToWater(maps) => determine_destination(maps, source_value),
        Maps::WaterToLight(maps) => determine_destination(maps, source_value),
        Maps::LightToTemperature(maps) => determine_destination(maps, source_value),
        Maps::TemperatureToHumidity(maps) => determine_destination(maps, source_value),
        Maps::HumidityToLocation(maps) => determine_destination(maps, source_value),
    }
}

fn determine_destination(maps: &HashMap<Range<u64>, Range<u64>>, source_value: u64) -> u64 {
    let source_range = maps.keys().find(|&source| source.contains(&source_value));

    match source_range {
        Some(source) => {
            let destination_range = maps.get(source).unwrap();

            destination_range.start + (source_value - source.start)
        }
        None => source_value,
    }
}

fn parse_to_maps(input: Vec<&str>) -> HashMap<Numbers, Maps> {
    let mut maps: HashMap<Numbers, Maps> = HashMap::with_capacity(7);
    for &block in input.iter() {
        let mut lines = block.lines();
        let (number, map) = match lines.next().unwrap() {
            "seed-to-soil map:" => (
                Numbers::Seed,
                Maps::SeedToSoil(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            "soil-to-fertilizer map:" => (
                Numbers::Soil,
                Maps::SoilToFertilizer(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            "fertilizer-to-water map:" => (
                Numbers::Fertilizer,
                Maps::FertilizerToWater(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            "water-to-light map:" => (
                Numbers::Water,
                Maps::WaterToLight(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            "light-to-temperature map:" => (
                Numbers::Light,
                Maps::LightToTemperature(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            "temperature-to-humidity map:" => (
                Numbers::Temperature,
                Maps::TemperatureToHumidity(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            "humidity-to-location map:" => (
                Numbers::Humidity,
                Maps::HumidityToLocation(build_hash_maps(lines.map(parse_line).collect_vec())),
            ),
            _ => panic!("Unkown Map Type"),
        };
        maps.insert(number, map);
    }
    maps
}

fn build_hash_maps(maps: Vec<(Range<u64>, Range<u64>)>) -> HashMap<Range<u64>, Range<u64>> {
    let mut result: HashMap<Range<u64>, Range<u64>> = HashMap::new();
    for (destination_range, source_range) in maps {
        result.insert(source_range, destination_range);
    }
    result
}

fn parse_line(line: &str) -> (Range<u64>, Range<u64>) {
    let numbers: (u64, u64, u64) = line
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();
    (
        numbers.0..(numbers.0 + numbers.2),
        numbers.1..(numbers.1 + numbers.2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        assert_eq!(
            35,
            *get_lowest_location_per_seed(input).values().min().unwrap()
        );
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!(
            "The lowest location is {}",
            *get_lowest_location_per_seed(input).values().min().unwrap()
        );
    }

    /* #[test]
            fn part_2_example() {
                let input = "seeds: 79 14 55 13

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
    56 93 4";
                assert_eq!(30, count_copies_of_cards(input))
            } */

    /*     #[test]
    fn part_2() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all scratch cards is {}",
            count_copies_of_cards(input)
        );
    } */
}
