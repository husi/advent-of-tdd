
mod map {
    use std::collections::HashMap;
    use std::ops::Rem;

    use itertools::Itertools;

    #[derive(PartialEq, Debug)]
    pub struct MapNode {
        pub id: String,
        pub left: String,
        pub right: String,
    }

    #[derive(PartialEq, Debug)]
    pub struct Navigation {
        instructions: String,
        map: HashMap<String, MapNode>,
    }


    #[derive(Debug)]
    pub struct Journey<'a> {
        nav: &'a Navigation,
        pub node: &'a MapNode,
        position: usize,
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
pub struct Position {
        pub position: usize,
        pub node_id: String
    }
    impl Journey<'_> {
        pub fn pos(&self) -> Position {
            Position{
                position: self.position.rem(&self.nav.instructions.len()),
                node_id: self.node.id.clone(),
            }
        }
    }

    impl<'a> Iterator for Journey<'a> {
        type Item = &'a MapNode;
        fn next(&mut self) -> Option<Self::Item> {
            let instr = &self.nav.instructions;
            let instr_char = instr.chars().nth(self.position.rem(instr.len())).unwrap();
            let node = match instr_char {
                'R' => &self.node.right,
                'L' => &self.node.left,
                _ => panic!("Invalid instruction {instr_char}")
            };
            self.node = self.nav.map.get(node).unwrap();
            self.position += 1;

            Some(self.node)
        }
    }

    impl MapNode {
        pub fn parse(input: &str) -> Self {
            let (id, left, right) = input.split(['=', ' ', '(', ',', ')'])
                .filter_map(|token| if !token.is_empty() { Some(token.to_string()) } else { None })
                .collect_tuple().unwrap();

            Self { id, left, right }
        }
    }

    impl Navigation {
        pub fn load_map(input: &Vec<String>) -> Self {
            let mut iter = input.iter();
            let instructions = iter.next().unwrap().clone();
            iter.next();

            let map = iter.map(|line| {
                let node = MapNode::parse(line);
                (node.id.clone(), node)
            }).collect();

            Self { instructions, map }
        }

        pub fn journey(&self, start_node: &str) -> Journey {
            Journey { nav: &self, node: self.map.get(start_node).unwrap(), position: 0 }
        }

        pub fn ghost_start(&self) -> Vec<String> {
            self.map.keys().filter_map(|key| if key.ends_with("A") {Some(key.clone())} else {None}).collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use common::load_aoc_input;

        use super::*;

        #[test]
        fn test_map_node_parse() {
            assert_eq!(MapNode::parse("AAA = (BBB, CCC)"), MapNode { id: "AAA".to_string(), left: "BBB".to_string(), right: "CCC".to_string() });
        }

        #[test]
        fn test_navigation_parse() {
            let input = load_aoc_input("test_data/e2.txt");
            assert_eq!(Navigation::load_map(&input),
                       Navigation {
                           instructions: "LLR".to_string(),
                           map: HashMap::from([
                               ("AAA".to_string(), MapNode::parse("AAA = (BBB, BBB)")),
                               ("BBB".to_string(), MapNode::parse("BBB = (AAA, ZZZ)")),
                               ("ZZZ".to_string(), MapNode::parse("ZZZ = (ZZZ, ZZZ)")),
                           ]),
                       })
        }

        #[test]
        fn test_navigation_journey() {
            let input = load_aoc_input("test_data/e2.txt");
            let navigation = Navigation::load_map(&input);
            let mut journey = navigation.journey("AAA");
            let a_node = MapNode::parse("AAA = (BBB, BBB)");
            let b_node = MapNode::parse("BBB = (AAA, ZZZ)");
            let z_node = MapNode::parse("ZZZ = (ZZZ, ZZZ)");
            assert_eq!(journey.next(), Some(&b_node));
            assert_eq!(journey.next(), Some(&a_node));
            assert_eq!(journey.next(), Some(&b_node));
            assert_eq!(journey.next(), Some(&a_node));
            assert_eq!(journey.next(), Some(&b_node));
            assert_eq!(journey.next(), Some(&z_node));
            assert_eq!(journey.next(), Some(&z_node));
            assert_eq!(journey.next(), Some(&z_node));
            assert_eq!(journey.next(), Some(&z_node));
        }
    }
}

mod example1 {
    use common::load_aoc_input;

    use crate::map::{MapNode, Navigation};

    pub fn compute(input_file: &str) -> usize {
        let input = load_aoc_input(input_file);
        let navigation = Navigation::load_map(&input);

        navigation.journey("AAA").take_while(|MapNode { id, .. }| id != "ZZZ").count() + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            for (example, expected) in [("test_data/e1.txt", 2), ("test_data/e2.txt", 6)] {
                assert_eq!(compute(example), expected);
            }
        }
    }
}

mod example2 {
    use itertools::Itertools;
    use rayon::prelude::*;

    use common::load_aoc_input;

    use crate::map::{Journey, Navigation, Position};

    fn find_loop (journey: & mut Journey) -> (Vec<(Position, usize)>, usize) {
        let mut zeros: Vec<(Position, usize)> = vec![(journey.pos(), 0)];
        let mut count: usize = 0;

        loop {
            while !journey.pos().node_id.ends_with("Z") {
                journey.next();
                count += 1;
                // println!("{count}: {:?}", journey.pos());
            };

            println!("zero: {:?} {count}", journey.pos());

            let found_position = zeros.iter().enumerate().find_map(|(idx,(pos, _))| {
                if pos.eq(&journey.pos()) {
                    Some(idx)
                } else {
                    None
                }});

            zeros.push((journey.pos(), count));
            if let Some(idx) = found_position
            {
                println!("loop back to {idx}: {:?}", journey.pos());
                return (zeros, idx)

            }
            journey.next();
            count = 1;
        }
    }

    #[derive(Debug)]
    struct GhostZeroes {
        pos: usize,
        idx: usize,
        zeros: Vec<(Position, usize)>,
        loop_back_index: usize
    }

    impl GhostZeroes {

        pub fn new(zeros: Vec<(Position, usize)>, loop_back_index: usize) -> Self {
            GhostZeroes {pos:0, idx:0, zeros, loop_back_index}
        }
    }
    impl Iterator for GhostZeroes {

        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            self.idx += 1;
            self.pos += self.zeros[self.idx].1;

            if self.idx == self.zeros.len()-1 {
                self.idx = self.loop_back_index
            }
            Some(self.pos)

        }
    }

    pub fn compute(input_file: &str) -> usize {
        let input = load_aoc_input(input_file);
        let navigation = Navigation::load_map(&input);
        let mut ghost_journeys: Vec<Journey> = navigation.ghost_start()
            .into_iter()
            .map(|start_point| navigation.journey(&start_point)).collect();

        let mut ghost_zeros:Vec<GhostZeroes> = ghost_journeys
            .iter_mut()
            .map(|j| {
                let (zeros, loop_back_index) = find_loop(j);
                GhostZeroes::new(zeros, loop_back_index)
            })
            .collect();

        let mut max_steps: usize = 0;
        let mut print_target: usize = 0;
        let delta: usize = 1_000_000_000_000;

        let mut tlist = &mut ghost_zeros[..];

        loop {
            for mut ghost in &mut tlist.iter_mut() {
                if ghost.pos==0 || ghost.pos < max_steps {
                    max_steps = ghost.find(|pos| *pos>= max_steps).unwrap()
                }
            }

            let ghost_positions = tlist.iter().map(|ghost| ghost.pos ).collect::<Vec<usize>>();

            if max_steps > print_target {
                println!("{max_steps} {:?}", ghost_positions);
                print_target += delta;
            }

            if ghost_positions.iter().all_equal() {
                break
            }




        }

        max_steps

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            for (example, expected) in [("test_data/e3.txt", 6)] {
                assert_eq!(compute(example), expected);
            }
        }
    }
}


fn main() {

    // println!("{}", example1::compute("test_data/puzzle1.txt"));
    println!("{}", example2::compute("test_data/puzzle1.txt"));
    // println!("{}", example2::compute("test_data/e3.txt"));
}
