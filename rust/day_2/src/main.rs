mod game {
    use std::cmp::max;

    #[derive(PartialEq, Debug)]
    pub struct Hand {
        red: u32,
        green: u32,
        blue: u32,
    }

    impl Hand {
        pub fn parse(input: &str) -> Self {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;
            for qubes in input.split(", ") {
                let qubes: Vec<&str> = qubes.split(" ").collect();
                if qubes.len() < 2 {
                    continue;
                }
                match qubes[1] {
                    "red" => red = qubes[0].parse().unwrap(),
                    "green" => green = qubes[0].parse().unwrap(),
                    "blue" => blue = qubes[0].parse().unwrap(),
                    _ => ()
                }
            }

            Self { red, green, blue }
        }

        pub fn validate(&self, red: u32, green: u32, blue: u32) -> bool {
            self.red <= red && self.green <= green && self.blue <= blue
        }

        pub fn power(&self) -> u32 {
            self.red * self.green * self.blue
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Game {
        pub id: u32,
        hands: Vec<Hand>,
    }

    impl Game {
        pub fn parse(input: &str) -> Self{
            let pieces: Vec<&str> = input.split(": ").collect();
            let id: u32= pieces[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

            let hands: Vec<Hand> = pieces[1].split("; ").map(|hand| Hand::parse(hand)).collect();
            Self{id, hands}
        }

        pub fn validate(&self, red: u32, green: u32, blue: u32) -> bool {
            self.hands.iter().all(|hand| hand.validate(red, green, blue))
        }

        pub fn max_hand(&self) -> Hand {
            self.hands.iter().fold(Hand{red:0, green:0, blue:0},|max_hand,hand| Hand{
                red: max(max_hand.red, hand.red),
                green: max(max_hand.green, hand.green),
                blue: max(max_hand.blue, hand.blue),
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_hand_parse() {
            assert_eq!(Hand::parse("1 green, 2 blue, 3 red"), Hand { red: 3, green: 1, blue: 2 });
            assert_eq!(Hand::parse("2 blue, 3 red"), Hand { red: 3, green: 0, blue: 2 });
            assert_eq!(Hand::parse("bad"), Hand { red: 0, green: 0, blue: 0 });
        }

        #[test]
        fn test_hand_validate() {
            let hand = Hand { red: 1, green: 2, blue: 3 };

            assert!(hand.validate(1, 2, 3));
            assert!(!hand.validate(0, 0, 0));
            assert!(!hand.validate(2, 2, 2));
            assert!(hand.validate(3, 4, 5));
        }

        #[test]
        fn test_game_parse() {
            assert_eq!(Game::parse("Game 2: 2 blue, 4 red, 7 green; 3 blue, 2 green; 3 green, 14 red, 1 blue"),
                       Game {id: 2, hands:vec![
                           Hand { red: 4, green: 7, blue:2},
                           Hand { red: 0, green: 2, blue:3},
                           Hand { red: 14, green: 3, blue:1},
                       ]});
            assert_eq!(Game::parse("Game 2: 2 blue, 4 red, 7 green; 17 red, 3 blue, 2 green; 3 green, 14 red, 1 blue"),
                       Game {id: 2, hands:vec![
                           Hand { red: 4, green: 7, blue:2},
                           Hand { red: 17, green: 2, blue:3},
                           Hand { red: 14, green: 3, blue:1},
                       ]});
        }

        #[test]
        fn test_game_validate() {
            let game = Game::parse("Game 2: 2 blue, 4 red, 7 green; 3 blue, 2 green; 3 green, 14 red, 1 blue");
            assert!(game.validate(14,7,3));
            assert!(!game.validate(13,7,3));
        }

        #[test]
        fn test_game_max_hand() {
            let game = Game::parse("Game 2: 2 blue, 4 red, 7 green; 3 blue, 2 green; 3 green, 14 red, 1 blue");
            assert_eq!(game.max_hand(), Hand{red:14, green:7, blue:3})
        }

    }
}

mod exercise1 {
    use common::load_aoc_input;
    use crate::game::Game;

    pub fn compute(input_file: &str) -> u32{
        let input = load_aoc_input(input_file);
        let red: u32 = 12;
        let green: u32 = 13;
        let blue: u32 = 14;

        input.iter()
            .map(|game| Game::parse(game))
            .filter(|game|game.validate(red,green,blue))
            .map(|game| game.id)
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            assert_eq!(compute("test_data/e1.txt"), 8)
        }
    }
}

mod exercise2 {
    use common::load_aoc_input;
    use crate::game::Game;

    pub fn compute(input_file: &str) -> u32 {
        let input = load_aoc_input(input_file);
        input.iter()
            .map(|game| Game::parse(game).max_hand().power())
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_compute() {
            assert_eq!(compute("test_data/e1.txt"), 2286);
        }
    }
}
fn main() {
    println!("{}",exercise1::compute("test_data/puzzle1.txt"));
    println!("{}",exercise2::compute("test_data/puzzle1.txt"));
}
