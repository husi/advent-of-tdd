
mod map {
    use std::collections::HashSet;
    use std::fmt::{Debug, Formatter};
    use crate::map::Direction::{East, North, South, West};

    #[derive(PartialEq, Debug)]
    pub enum MapNode {
        Empty,
        NorthSouth,
        EastWest,
        NorthEast,
        NorthWest,
        SouthEast,
        SouthWest,
        StartPoint,
    }

    impl MapNode {
        pub fn to_display(&self) -> char {
            match *self {
                MapNode::Empty => '\u{00B7}',
                MapNode::NorthSouth => '\u{2551}',
                MapNode::EastWest => '\u{2550}',
                MapNode::NorthEast => '\u{255A}',
                MapNode::NorthWest => '\u{255D}',
                MapNode::SouthEast => '\u{2554}',
                MapNode::SouthWest => '\u{2557}',
                MapNode::StartPoint => 'S',
            }
        }
        fn from_char(c: &char) -> Self {
            match c {
                '.' => MapNode::Empty,
                '|' => MapNode::NorthSouth,
                '-' => MapNode::EastWest,
                'L' => MapNode::NorthEast,
                'J' => MapNode::NorthWest,
                '7' => MapNode::SouthWest,
                'F' => MapNode::SouthEast,
                'S' => MapNode::StartPoint,
                _ => panic!("invalid char {c}")
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Direction {
        South,
        North,
        East,
        West,
    }

    impl From<&Direction> for (isize, isize) {
        fn from(value: &Direction) -> Self {
            match value {
                East => (1, 0),
                South => (0, 1),
                West => (-1, 0),
                North => (0, -1),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Critter {
        pub pos: (usize, usize),
        pub direction: Direction,
    }

    impl Critter {
        pub fn new(pos: (usize, usize), direction: Direction) -> Self {
            Critter { pos, direction }
        }
    }

    #[derive(PartialEq)]
    pub struct Map {
        pub map: Vec<Vec<MapNode>>,
        pub start_point: (usize, usize),
    }


    impl Map {
        pub fn parse(lines: Vec<String>) -> Self {
            let mut start_point: (usize, usize) = (0, 0);
            let map: Vec<Vec<MapNode>> = lines.iter()
                .enumerate()
                .map(|(line_idx, line)| line.chars()
                    .enumerate()
                    .map(|(idx, c)| {
                        let node = MapNode::from_char(&c);
                        if node == MapNode::StartPoint {
                            start_point = (idx, line_idx);
                        }
                        node
                    })
                    .collect())
                .collect();


            Self { map, start_point }
        }

        pub fn get(&self, x: usize, y: usize) -> Option<&MapNode> {
            self.map.get(y)?.get(x)
        }

        pub fn get_next_position(&self, x: &usize, y: &usize, direction: &Direction) -> Result<((usize, usize), Direction), String> {
            let (xoffset, yoffset) = direction.into();
            let new_x = x.saturating_add_signed(xoffset);
            let new_y = y.saturating_add_signed(yoffset);

            if new_x == *x && new_y == *y {
                return Err("moving off the map".to_string());
            }

            let new_pos = (new_x, new_y);
            let next_node = self.get(new_x, new_y);
            match direction {
                East => match next_node {
                    Some(MapNode::EastWest) => {
                        Ok((new_pos, East))
                    }
                    Some(MapNode::NorthWest) => {
                        Ok((new_pos, North))
                    }
                    Some(MapNode::SouthWest) => {
                        Ok((new_pos, South))
                    }
                    Some(MapNode::StartPoint) => {
                        Ok((new_pos, East))
                    }
                    _ => Err(format!("Could not move to {direction:?} {new_pos:?} {next_node:?}"))
                },
                South => match next_node {
                    Some(MapNode::NorthSouth) => {
                        Ok((new_pos, South))
                    }
                    Some(MapNode::NorthEast) => {
                        Ok((new_pos, East))
                    }
                    Some(MapNode::NorthWest) => {
                        Ok((new_pos, West))
                    }
                    Some(MapNode::StartPoint) => {
                        Ok((new_pos, South))
                    }
                    _ => Err(format!("Could not move to {direction:?} {new_pos:?} {next_node:?}"))
                },
                West => match next_node {
                    Some(MapNode::EastWest) => {
                        Ok((new_pos, West))
                    }
                    Some(MapNode::NorthEast) => {
                        Ok((new_pos, North))
                    }
                    Some(MapNode::SouthEast) => {
                        Ok((new_pos, South))
                    }
                    Some(MapNode::StartPoint) => {
                        Ok((new_pos, West))
                    }
                    _ => Err(format!("Could not move to {direction:?} {new_pos:?} {next_node:?}"))
                },
                North => match next_node {
                    Some(MapNode::NorthSouth) => {
                        Ok((new_pos, North))
                    }
                    Some(MapNode::SouthEast) => {
                        Ok((new_pos, East))
                    }
                    Some(MapNode::SouthWest) => {
                        Ok((new_pos, West))
                    }
                    Some(MapNode::StartPoint) => {
                        Ok((new_pos, North))
                    }
                    _ => Err(format!("Could not move to {direction:?} {new_pos:?} {next_node:?}"))
                }
            }
        }

        pub fn get_possible_start_critters(&self) -> Vec<Critter> {
            let (start_x, start_y) = self.start_point;
            let mut result = Vec::new();
            for direction in vec![East, South, West, North] {
                if let Ok(_) = self.get_next_position(&start_x, &start_y, &direction) {
                    result.push(Critter::new(self.start_point, direction));
                }
            }

            result
        }

        pub fn move_critter(&self, critter: &mut Critter) -> Result<(), String> {
            let (posx, posy) = &critter.pos;
            let direction = &critter.direction;

            match self.get_next_position(posx, posy, direction) {
                Ok((new_pos, new_direction)) => {
                    critter.pos = new_pos;
                    critter.direction = new_direction;
                    Ok(())
                }
                Err(error) => Err(format!("Could not move critter because: {error}"))
            }
        }

        pub fn move_critter_around(&self, critter: &mut Critter) -> Result<HashSet<(usize, usize)>, String> {
            let mut route = HashSet::new();
            loop {
                match self.move_critter(critter) {
                    Ok(_) => {
                        route.insert(critter.pos);
                    }
                    Err(error) => return Err(error)
                };

                if critter.pos == self.start_point {
                    break;
                }
            }

            Ok(route)
        }
    }


    impl Debug for Map {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let _ = write!(f, "\n");
            for line in &self.map {
                let line: String = line.iter()
                    .map(|node| node.to_display())
                    .collect();
                let _ = write!(f, "{line}\n");
            }
            Ok(())
        }
    }


    #[cfg(test)]
    mod tests {
        use common::load_aoc_input;
        use crate::map::MapNode::{EastWest, Empty, NorthEast, NorthSouth, NorthWest, SouthWest, StartPoint};
        use super::*;

        #[test]
        fn test_parse() {
            let input = load_aoc_input("test_data/e1.txt");
            assert_eq!(Map::parse(input), Map {
                map: vec![
                    vec![Empty, Empty, Empty, Empty, Empty],
                    vec![Empty, StartPoint, EastWest, SouthWest, Empty],
                    vec![Empty, NorthSouth, Empty, NorthSouth, Empty],
                    vec![Empty, NorthEast, EastWest, NorthWest, Empty],
                    vec![Empty, Empty, Empty, Empty, Empty],
                ],
                start_point: (1, 1),
            });
        }

        #[test]
        fn test_start_critters() {
            let input = load_aoc_input("test_data/e1.txt");
            let map = Map::parse(input);

            assert_eq!(map.get_possible_start_critters(),
                       vec![Critter::new((1, 1), Direction::East),
                            Critter::new((1, 1), Direction::South)]
            );
        }

        #[test]
        fn test_move_critter() {
            let input = load_aoc_input("test_data/e1.txt");
            let map = Map::parse(input);

            let mut critter = Critter::new(map.start_point, East);

            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((2, 1), East));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((3, 1), South));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((3, 2), South));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((3, 3), West));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((2, 3), West));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((1, 3), North));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new((1, 2), North));
            assert!(map.move_critter(&mut critter).is_ok());
            assert_eq!(critter, Critter::new(map.start_point, North));
        }

        #[test]
        fn test_move_critter_around() {
            let input = load_aoc_input("test_data/e1.txt");
            let map = Map::parse(input);

            let mut critter = Critter::new(map.start_point, East);

            assert_eq!(map.move_critter_around(&mut critter),
                       Ok(HashSet::from([
                           (2, 1), (3, 1),
                           (3, 2), (3, 3),
                           (2, 3), (1, 3),
                           (1, 2), (1, 1),
                       ])));
        }
    }
}

mod exercise1 {
    use common::load_aoc_input;
    use crate::map::Map;

    pub fn calculate(input_file: &str) -> usize {
        let input = load_aoc_input(input_file);
        let map = Map::parse(input);

        let steps = map.get_possible_start_critters().iter_mut()
            .filter_map(|critter| match map.move_critter_around(critter) {
                Ok(steps) => Some(steps),
                Err(error) => {
                    println!("Critter failed: {error}");
                    None
                }
            }).nth(0).unwrap();

        steps.len() / 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate() {
            for (input_file, expected) in vec![
                ("test_data/e1.txt", 4),
                ("test_data/e2.txt", 4),
                ("test_data/e3.txt", 8),
            ] {
                assert_eq!(calculate(input_file), expected)
            }
        }
    }
}

mod exercise2 {
    use common::load_aoc_input;
    use crate::map::{Direction, Map, MapNode};
    use crate::map::MapNode::{NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest, StartPoint, EastWest};

    pub fn calculate(input_file: &str) -> usize {
        let input = load_aoc_input(input_file);
        let map = Map::parse(input);

        let mut possible_critters = map.get_possible_start_critters();
        if possible_critters.len()!=2 {
            panic!("Critters number not good"); // let see if we have these in teh examples
        }

        let start_node = match (&possible_critters.get(0).unwrap().direction, &possible_critters.get(1).unwrap().direction){
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => EastWest,
            (Direction::South, Direction::North) | (Direction::North, Direction::South) => NorthSouth,
            (Direction::North, Direction::East) | (Direction::East, Direction::North) => NorthEast,
            (Direction::North, Direction::West) | (Direction::West, Direction::North) => NorthWest,
            (Direction::South, Direction::East) | (Direction::East, Direction::South) => SouthEast,
            (Direction::South, Direction::West) | (Direction::West, Direction::South) => SouthWest,
            _ => panic!("Not possible start node")
        };

        let route = match map.move_critter_around(possible_critters.get_mut(0).unwrap()) {
                Ok(steps) => Some(steps),
                Err(error) => {
                    println!("Critter failed: {error}");
                    None
                }
            }.unwrap();


        let mut count: usize = 0;
        let mut last_corner: Option<&MapNode> = None;

        for (y, map_line) in map.map.iter().enumerate() {
            let mut in_loop = false;
            for (x, mut node) in map_line.iter().enumerate() {
                if route.contains(&(x, y)) {
                    if *node == StartPoint {
                        node = &start_node;
                    }
                    match node {
                        NorthSouth => {
                            in_loop = !in_loop;
                        }
                        NorthEast | SouthEast => {
                            last_corner = Some(&node);
                        }
                        NorthWest => {
                            if let Some(SouthEast) = last_corner {
                                in_loop = !in_loop;
                            }
                        }
                        SouthWest => {
                            if let Some(NorthEast) = last_corner {
                                in_loop = !in_loop;
                            }
                        }
                        StartPoint => {}
                        _ => {}
                    };
                } else {
                    if in_loop {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate() {
            for (input_file, expected) in vec![
                ("test_data/e4.txt", 4),
                ("test_data/e5.txt", 8),
                ("test_data/e6.txt", 10),
                ("test_data/e7.txt", 4),
            ] {
                assert_eq!(calculate(input_file), expected)
            }
        }
    }
}


fn main() {
    println!("{}", exercise1::calculate("test_data/puzzle1.txt"));
    println!("{}", exercise2::calculate("test_data/puzzle1.txt"));
}
