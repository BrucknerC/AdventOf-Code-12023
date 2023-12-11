use std::collections::HashSet;

use array2d::Array2D;
use itertools::Itertools;

pub fn get_numbers_adjacent_to_symbols(input: &str) -> Vec<u32> {
    let lines = input
        .lines()
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();
    let schematic = Array2D::from_rows(&lines).unwrap();

    let number_adresses: HashSet<(usize, usize)> = get_indices_of_symbols(&schematic)
        .iter()
        .flat_map(|&address| get_neighbouring_digits(&schematic, address))
        .map(|address| get_start_index_of_number(&schematic, address))
        .collect();

    number_adresses
        .iter()
        .map(|&address| get_number_at(&schematic, address))
        .collect_vec()
}

fn get_number_at(schematic: &Array2D<char>, (row_index, col_index): (usize, usize)) -> u32 {
    let mut col = col_index;
    let mut number_as_string = "".to_owned();
    while let Some(val) = schematic.get(row_index, col) {
        if val.is_ascii_digit() {
            number_as_string.push(*val);
            col += 1;
        } else {
            break;
        }
    }
    number_as_string.parse().unwrap()
}

fn get_indices_of_symbols(schematic: &Array2D<char>) -> Vec<(usize, usize)> {
    let mut symbol_indices = vec![];
    for (row_index, row) in schematic.rows_iter().enumerate() {
        for (col_index, &element) in row.enumerate() {
            if !element.is_ascii_digit() && element != '.' {
                symbol_indices.push((row_index, col_index));
            }
        }
    }

    symbol_indices
}

fn get_neighbouring_digits(
    schematic: &Array2D<char>,
    (row_index, col_index): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbour_indices = vec![];

    for row in 1.max(row_index) - 1..=row_index + 1 {
        for col in 1.max(col_index) - 1..=col_index + 1 {
            if let Some(&val) = schematic.get(row, col) {
                if val.is_ascii_digit() {
                    neighbour_indices.push((row, col));
                }
            }
        }
    }

    neighbour_indices
}

fn get_start_index_of_number(
    schematic: &Array2D<char>,
    (row_index, col_index): (usize, usize),
) -> (usize, usize) {
    let mut col = col_index;
    while let Some(val) = schematic.get(row_index, col) {
        if col == 0 && val.is_ascii_digit() {
            return (row_index, col);
        } else if val.is_ascii_digit() {
            col -= 1;
        } else {
            return (row_index, col + 1);
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(
            4361,
            get_numbers_adjacent_to_symbols(input).iter().sum::<u32>()
        );
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!(
            "The sum of all part numbers is {}",
            get_numbers_adjacent_to_symbols(input).iter().sum::<u32>()
        );
    }
}
