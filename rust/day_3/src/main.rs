mod map_reader {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::iter::Extend;
    use itertools::Itertools;

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::map_reader::MapItem::{Marker, Number};

    lazy_static! {
        static ref MAEKER_OR_NUM_PATTERN: Regex = Regex::new(r"([^.\d]|\d+)").unwrap();
    }

    #[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
    pub struct Position {
        x: usize,
        y: usize,
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum MapItem {
        Number { pos: Position, length: usize, value: u32 },
        Marker { pos: Position, sign: char },
    }

    impl MapItem {
        pub fn num(&self) -> Option<u32> {
            match self {
                Number { pos: _, length: _, value } => Some(*value),
                _ => None
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Map {
        map: HashMap<Position, MapItem>,
    }

    impl Map {
        pub fn parse(input: Vec<String>) -> Map {
            let mut map: HashMap<Position, MapItem> = HashMap::new();

            for (ln, line) in input.iter().enumerate() {
                map.extend(Self::parse_line(ln, &line))
            }

            Map { map }
        }

        fn parse_line(ln: usize, line: &str) -> HashMap<Position, MapItem> {
            MAEKER_OR_NUM_PATTERN.find_iter(line)
                .map(|item| {
                    let position = Position { x: item.start(), y: ln };
                    if let Ok(value) = item.as_str().parse() {
                        (position, Number { pos: position, length: item.len(), value })
                    } else {
                        (position, Marker { pos: position, sign: item.as_str().chars().nth(0).unwrap() })
                    }
                })
                .collect()
        }

        pub fn part_numbers(&self) -> Vec<MapItem> {
            self.map.values()
                .filter_map(|item| {
                    match item {
                        Number { pos, length, value: _ } => {
                            if self.neighbours(pos, *length).iter().any(|n| matches!(n, Marker{pos:_,sign:_})) {
                                Some(item)
                            }  else {
                                None
                            }
                        },
                        _ => None
                    }

                })
                .cloned()
                .collect()
        }

        pub fn part_number_values(&self) -> Vec<u32> {
            self.part_numbers().iter().map(|item| item.num().unwrap()).sorted().collect()
        }

        fn neighbours(&self, position: &Position, len: usize) -> Vec<MapItem> {
            let neighbour_indices:Vec<Position> = (-1i32..=len as i32).cartesian_product(-1..=1)
                .filter_map(|(x, y)| {
                    let xi = (position.x as i32) + x;
                    let yi = (position.y as i32) + y;

                    if xi >= 0 && yi >= 0 && (yi as usize != position.y || !(position.x..position.x + len).contains(&(xi as usize))) {
                        Some(Position { x: xi as usize, y: yi as usize })
                    } else {
                        None
                    }
                })
                .collect();

            neighbour_indices.iter().filter_map(|position| self.map.get(position).cloned()).collect()
        }

        pub fn get_gears(&self) -> HashMap<Position, Vec<MapItem>> {
            let mut result:HashMap< crate::map_reader::Position, Vec< crate::map_reader::MapItem >> = HashMap::new();

            for item in self.map.values() {
                match item {
                    Number { pos, length, value: _ } => {
                        for neighbour in self.neighbours(pos, *length) {
                            match neighbour {
                                Marker { pos: marker_pos, sign: '*' } => {
                                    result.entry(marker_pos).or_insert(Vec::new()).push(*item);
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            }

            result
        }
    }

    #[cfg(test)]
    mod tests {
        use common::load_aoc_input;
        use crate::map_reader::MapItem::{Marker, Number};

        use super::*;

        #[test]
        fn test_parse_line() {
            assert_eq!(Map::parse_line(1, &".....+.58."),
                       HashMap::<Position, MapItem>::from([
                           (Position { x: 5, y: 1 }, Marker { pos: Position { x: 5, y: 1 }, sign: '+' }),
                           (Position { x: 7, y: 1 }, Number { pos: Position { x: 7, y: 1 }, length: 2, value: 58 })
                       ]));
        }

        #[test]
        fn test_parse_map() {
            assert_eq!(Map::parse(vec![".....+.58.",
                                       ".....-.18."].iter().map(|line| line.to_string()).collect()),
                       Map {
                           map: HashMap::<Position, MapItem>::from([
                               (Position { x: 5, y: 0 }, Marker { pos: Position { x: 5, y: 0 }, sign: '+' }),
                               (Position { x: 7, y: 0 }, Number { pos: Position { x: 7, y: 0 }, length: 2, value: 58 }),
                               (Position { x: 5, y: 1 }, Marker { pos: Position { x: 5, y: 1 }, sign: '-' }),
                               (Position { x: 7, y: 1 }, Number { pos: Position { x: 7, y: 1 }, length: 2, value: 18 })
                           ])
                       });
        }

        #[test]
        fn test_neighboures() {
            let input = load_aoc_input("test_data/e1.txt");
            let map = Map::parse(input);

            assert_eq!(map.neighbours(&Position { x: 0, y: 0 }, 3), vec![
                Marker { pos: Position { x: 3, y: 1 }, sign: '*' },
            ])
        }

        #[test]
        fn test_part_numbers() {
            let input = load_aoc_input("test_data/e1.txt");
            let map = Map::parse(input);

            assert_eq!(map.part_numbers().iter()
                           .sorted_by(|it1, it2| Ord::cmp(&it1.num().unwrap(), &it2.num().unwrap()))
                           .cloned()
                           .collect::<Vec<MapItem>>(), vec![
                Number { pos: Position { x: 2, y: 2 }, length: 2, value: 35 },
                Number { pos: Position { x: 0, y: 0 }, length: 3, value: 467 },
                Number { pos: Position { x: 2, y: 6 }, length: 3, value: 592 },
                Number { pos: Position { x: 5, y: 9 }, length: 3, value: 598 },
                Number { pos: Position { x: 0, y: 4 }, length: 3, value: 617 },
                Number { pos: Position { x: 6, y: 2 }, length: 3, value: 633 },
                Number { pos: Position { x: 1, y: 9 }, length: 3, value: 664 },
                Number { pos: Position { x: 6, y: 7 }, length: 3, value: 755 }
            ]);
            assert_eq!(map.part_number_values(), vec![35, 467, 592, 598, 617, 633, 664, 755]);
        }

        #[test]
        #[ignore] //need to get rid of ordering mismatch
        fn test_get_gears() {
            let input = load_aoc_input("test_data/e1.txt");
            let map = Map::parse(input);

            assert_eq!(map.get_gears(), HashMap::from( [
                    (Position { x: 3, y: 1 }, vec![Number { pos: Position { x: 0, y: 0 }, length: 3, value: 467 }, Number { pos: Position { x: 2, y: 2 }, length: 2, value: 35 }]),
                    (Position { x: 5, y: 8 }, vec![Number { pos: Position { x: 5, y: 9 }, length: 3, value: 598 }, Number { pos: Position { x: 6, y: 7 }, length: 3, value: 755 }]),
                    (Position { x: 3, y: 4 }, vec![Number { pos: Position { x: 0, y: 4 }, length: 3, value: 617 }])
                    ]
            ))
        }
    }
}

mod exercise1 {
    use common::load_aoc_input;

    pub fn compute(input_file: &str) -> u32 {
        let input = load_aoc_input(input_file);
        let map = crate::map_reader::Map::parse(input);

        map.part_number_values().iter().sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(compute("test_data/e1.txt"), 4361)
        }

    }
}

mod exercise2 {
    use common::load_aoc_input;

    pub fn compute(input_file: &str) -> u32 {
        let input = load_aoc_input(input_file);
        let map = crate::map_reader::Map::parse(input);

        map.get_gears()
            .values()
            .filter_map(|numbers| {
                if numbers.len() == 2 {
                    Some(numbers[0].num().unwrap() * numbers[1].num().unwrap())
                }
                else {
                    None
                }
            })
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(compute("test_data/e1.txt"), 467835)
        }

    }
}


fn main() {
    println!("{}",exercise1::compute("test_data/puzzle1.txt"));
    println!("{}",exercise2::compute("test_data/puzzle1.txt"));
}
