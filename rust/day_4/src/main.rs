mod parser {
    use std::collections::HashSet;

    use itertools::Itertools;

    pub fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
        line
            .split(&[':', '|'][..])
            .skip(1)
            .map(|nums| {
                nums.trim()
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>()
            })
            .collect_tuple::<(HashSet<u32>, HashSet<u32>)>().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_line() {
            assert_eq!(parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                       (HashSet::from([41, 48, 83, 86, 17]), HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])));
        }
    }
}

mod exercise1 {
    use common::load_aoc_input;

    use crate::parser::parse_line;

    pub fn compute(input_file: &str) -> u32 {
        let lines = load_aoc_input(input_file);
        lines.iter()
            .map(|line| parse_line(line))
            .map(|(winning, have)| {
                let count = winning.intersection(&have).count() as u32;
                if count > 0 {
                    2u32.pow(count - 1)
                } else { 0 }
            })
            .sum()
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            assert_eq!(compute("test_data/e1.txt"), 13)
        }
    }
}

mod exercise2 {
    use common::load_aoc_input;

    use crate::parser::parse_line;

    pub fn compute(input_file: &str) -> u32 {
        let lines = load_aoc_input(input_file);
        let mut num_cards = vec![1u32; lines.len()];
        let matchings: Vec<u32> = lines.iter()
            .map(|line| parse_line(line))
            .map(|(winning, have)| winning.intersection(&have).count() as u32)
            .collect();

        matchings.iter()
            .enumerate()
            .for_each(|(index, count)| {
                ((index + 1)..=(index + (*count as usize))).for_each(|card_index| {
                    num_cards[card_index] += num_cards[index];
                })
            });

        num_cards.iter().sum()
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            assert_eq!(compute("test_data/e1.txt"), 30)
        }
    }
}

fn main() {
    println!("{}", exercise1::compute("test_data/puzzle1.txt"));
    println!("{}", exercise2::compute("test_data/puzzle1.txt"));
}
