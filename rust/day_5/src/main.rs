mod mapper {
    use std::ops::Range;
    use range_ext::intersect::Intersect;
    use range_ext::intersect::IntersectionExt;

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub struct GardenRange {
        pub(crate) start: usize,
        length: usize,
    }

    impl GardenRange {
        pub fn new(start: usize, length: usize) -> GardenRange {
            Self { start, length }
        }

        pub fn as_range(&self) -> Range<usize> {
            self.start..self.start + self.length
        }
    }


    #[derive(PartialEq, Debug)]
    struct GardenRule {
        source: usize,
        target: usize,
        length: usize,
    }

    impl GardenRule {
        pub fn parse(source: &str) -> Self {
            let [target, source, length] = source
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            Self { target, source, length }
        }

        pub fn map(&self, num: usize) -> Option<usize> {
            match num.checked_sub(self.source) {
                Some(idx) if idx < self.length => Some(self.target + idx),
                _ => None
            }
        }

        pub fn source_range(&self) -> Range<usize> {
            self.source..self.source + self.length
        }

        pub fn map_range(&self, range: &GardenRange) -> (Vec<GardenRange>, Option<GardenRange>) {
            match self.source_range().intersect_ext(&range.as_range())
            {
                IntersectionExt::Less => (vec![], Some(range.clone())),
                IntersectionExt::LessOverlap => {
                    let idx = range.start - self.source;
                    let mapped_length = self.length - idx;
                    (vec![GardenRange { start: self.target + idx, length: mapped_length }],
                     Some(GardenRange { start: range.start + mapped_length, length: range.length - mapped_length }))
                }
                IntersectionExt::Within => {
                    let idx = self.source - range.start;
                    (vec![
                        GardenRange { start: range.start, length: idx },
                        GardenRange { start: self.target, length: self.length },
                    ],
                     Some(GardenRange { start: self.source + self.length, length: range.length - self.length - idx }))
                }
                IntersectionExt::Same => (vec![GardenRange { start: self.target, length: self.length }], None),
                IntersectionExt::GreaterOverlap => {
                    let idx = self.source - range.start;
                    let mapped_length = range.length - idx;
                    (vec![
                        GardenRange { start: range.start, length: idx },
                        GardenRange { start: self.target, length: mapped_length },
                    ],
                     None)
                }
                IntersectionExt::Greater => (vec![range.clone()], None),
                IntersectionExt::Over => {
                    let idx = range.start - self.source;
                    (vec![GardenRange { start: self.target + idx, length: range.length }], None)
                }
                _ => (vec![], None)
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct GardenMap {
        name: String,
        rules: Vec<GardenRule>,
    }

    impl GardenMap {
        pub fn parse<I>(lines: &mut I) -> Self
            where I: Iterator<Item=String>
        {
            let name = lines.next().unwrap()
                .split_ascii_whitespace()
                .nth(0).unwrap()
                .to_string();
            let mut rules: Vec<GardenRule> = lines
                .take_while(|line| line.len() > 0)
                .map(|line| GardenRule::parse(&line))
                .collect();

            rules.sort_by(|rule1, rule2| rule1.source.cmp(&rule2.source));
            Self { name, rules }
        }

        pub fn map(&self, num: usize) -> usize {
            match self.rules.iter().find_map(|rule| rule.map(num)) {
                Some(x) => x,
                _ => num
            }
        }

        pub fn map_range(&self, range: &GardenRange) -> (Vec<GardenRange>, Option<GardenRange>) {
            let empty_ranges: (Vec<GardenRange>, Option<GardenRange>) = (vec![], Some(*range));
            self.rules.iter()
                .fold(empty_ranges, |(head, tail), rule| {
                    match tail {
                        Some(range) => {
                            let (new_ranges, tail) = rule.map_range(&range);
                            (head.into_iter()
                                 .chain(new_ranges.into_iter()
                                     .filter(|range| range.length > 0))
                                 .collect(),
                             tail)
                        }
                        None => (head, tail)
                    }
                })
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_garden_rule_parse() {
            assert_eq!(GardenRule::parse("50 98 2"), GardenRule { source: 98, target: 50, length: 2 });
        }

        #[test]
        fn test_garden_rule_map() {
            let rule = GardenRule::parse("50 98 2");
            assert_eq!(rule.map(97), None);
            assert_eq!(rule.map(98), Some(50));
            assert_eq!(rule.map(99), Some(51));
            assert_eq!(rule.map(100), None);
        }

        #[test]
        fn test_garden_rule_map_range() {
            let rule = GardenRule::parse("50 98 5");
            // After
            assert_eq!(rule.map_range(&GardenRange { start: 104, length: 10 }),
                       (
                           vec![],
                           Some(GardenRange { start: 104, length: 10 }),
                       )
            );
            // Overlap at end
            assert_eq!(rule.map_range(&GardenRange { start: 102, length: 10 }),
                       (
                           vec![GardenRange { start: 54, length: 1 }],
                           Some(GardenRange { start: 103, length: 9 }),
                       )
            );
            assert_eq!(rule.map_range(&GardenRange { start: 100, length: 10 }),
                       (
                           vec![GardenRange { start: 52, length: 3 }],
                           Some(GardenRange { start: 103, length: 7 }),
                       )
            );

            // Contains
            assert_eq!(rule.map_range(&GardenRange { start: 96, length: 10 }),
                       (
                           vec![GardenRange { start: 96, length: 2 },
                                GardenRange { start: 50, length: 5 }],
                           Some(GardenRange { start: 103, length: 3 }),
                       )
            );
            // Same
            assert_eq!(rule.map_range(&GardenRange { start: 98, length: 5 }),
                       (
                           vec![GardenRange { start: 50, length: 5 }],
                           None,
                       )
            );

            // Overlap front
            assert_eq!(rule.map_range(&GardenRange { start: 96, length: 5 }),
                       (
                           vec![
                               GardenRange { start: 96, length: 2 },
                               GardenRange { start: 50, length: 3 },
                           ],
                           None,
                       )
            );

            // Before
            assert_eq!(rule.map_range(&GardenRange { start: 92, length: 5 }),
                       (
                           vec![
                               GardenRange { start: 92, length: 5 },
                           ],
                           None,
                       )
            );
            //Within
            assert_eq!(rule.map_range(&GardenRange { start: 99, length: 3 }),
                       (
                           vec![
                               GardenRange { start: 51, length: 3 },
                           ],
                           None,
                       )
            );
        }

        #[test]
        fn test_garden_map_parse() {
            let lines = example_map();

            let lines = &mut lines.into_iter();

            assert_eq!(GardenMap::parse(lines),
                       GardenMap {
                           name: "seed-to-soil".to_string(),
                           rules: Vec::from([
                               GardenRule::parse("52 50 48"),
                               GardenRule::parse("50 98 2"),
                           ]),
                       });
            assert_eq!(lines.next(), Some("extra".to_string()));
        }

        fn example_map() -> Vec<String> {
            let lines: Vec<String> = Vec::from(["seed-to-soil map:",
                "50 98 2",
                "52 50 48",
                "",
                "extra"])
                .iter()
                .map(|line| line.to_string())
                .collect();
            lines
        }

        #[test]
        fn test_garden_map_map() {
            let lines = example_map();
            let lines = &mut lines.into_iter();
            let garden_map = GardenMap::parse(lines);
            assert_eq!(garden_map.map(49), 49);
            assert_eq!(garden_map.map(50), 52);
            assert_eq!(garden_map.map(51), 53);
            assert_eq!(garden_map.map(96), 98);
            assert_eq!(garden_map.map(97), 99);
            assert_eq!(garden_map.map(98), 50);
            assert_eq!(garden_map.map(99), 51);
            assert_eq!(garden_map.map(100), 100);
        }

        #[test]
        fn test_garden_map_map_range() {
            let lines = example_map();
            let lines = &mut lines.into_iter();
            let garden_map = GardenMap::parse(lines);


            assert_eq!(garden_map.map_range(&GardenRange { start: 40, length: 100 }),
                       (vec![
                           GardenRange { start: 40, length: 10 },
                           GardenRange { start: 52, length: 48 },
                           GardenRange { start: 50, length: 2 },
                       ],
                        Some(GardenRange { start: 100, length: 40 })));
        }
    }
}

mod exercise1 {
    use common::load_aoc_input;

    use crate::mapper::GardenMap;

    pub fn calculate(input_file: &str) -> usize {
        let lines = &mut load_aoc_input(input_file).into_iter().peekable();

        let seeds: Vec<usize> = lines.next().unwrap()
            .split(": ")
            .nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        lines.next();

        let mut garden_maps: Vec<GardenMap> = Vec::new();
        while lines.peek().is_some() {
            garden_maps.push(GardenMap::parse(lines))
        }

        seeds.into_iter()
            .map(|seed| {
                garden_maps.iter()
                    .fold(seed, |place, garden_map| garden_map.map(place))
            })
            .min().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calulate() {
            assert_eq!(calculate("test_data/e1.txt"), 35)
        }
    }
}

mod exercise2 {
    use itertools::Itertools;
    use common::load_aoc_input;

    use crate::mapper::{GardenMap, GardenRange};

    fn to_vec(map_result: (Vec<GardenRange>, Option<GardenRange>)) -> Vec<GardenRange> {
        let mut result = map_result.0.clone();
        if let Some(range) = map_result.1 {
            result.push(range);
        }

        result
    }

    fn map_ranges(ranges: Vec<GardenRange>, map: &GardenMap) -> Vec<GardenRange> {
        ranges.into_iter()
            .flat_map(|range| to_vec(map.map_range(&range)))
            .collect()
    }

    pub fn calculate(input_file: &str) -> usize {
        let lines = &mut load_aoc_input(input_file).into_iter().peekable();

        let seed_line = lines.next().unwrap();
        let seeds_chunks = seed_line
            .split(": ")
            .nth(1).unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .chunks(2);

        let seeds: Vec<GardenRange> = seeds_chunks.into_iter()
            .map(|mut chunk| {
                GardenRange::new(chunk.next().unwrap(), chunk.next().unwrap())
            })
            .collect();

        lines.next();

        let mut garden_maps: Vec<GardenMap> = Vec::new();
        while lines.peek().is_some() {
            garden_maps.push(GardenMap::parse(lines))
        };

        let ranges = garden_maps.iter()
            .fold(seeds, |ranges, map| {
                map_ranges(ranges, map)
            });

        ranges.iter().map(|range| range.start).min().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calulate() {
            assert_eq!(calculate("test_data/e1.txt"), 46)
        }
    }
}


fn main() {
    println!("{}", exercise1::calculate("test_data/puzzle1.txt"));
    println!("{}", exercise2::calculate("test_data/puzzle1.txt"));
}
