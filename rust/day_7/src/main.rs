use crate::exercise::compute;

mod cards {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Formatter};

    use itertools::Itertools;

    use crate::cards::HandClassification::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

    #[derive(PartialEq, Debug, PartialOrd, Ord, Eq)]
    pub enum HandClassification {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialEq, Eq)]
    pub struct Hand <const T: bool>{
        pub cards: [u8; 5],
    }


    impl<const JOKER: bool> Hand<JOKER> {
        pub fn parse(input: &str) -> Self {
            let cards: [u8; 5] = input.chars().map(|c| {
                match c {
                    '1'..='9' => (c as u8 - '1' as u8) + 1,
                    'T' => 10,
                    'J' => if JOKER {0} else {11},
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Invalid Hand: {input}")
                }
            }).collect::<Vec<u8>>().try_into().unwrap();
            Self { cards }
        }

        pub fn classify(&self) -> HandClassification {
            let counts = self.cards.iter().counts();

            let base_classification = match counts.len() {
                5 => HighCard,
                4 => OnePair,
                3 => if counts.values()
                    .any(|count| *count == 3) { ThreeOfAKind } else { TwoPair },
                2 => if counts.values()
                    .any(|count| *count == 4) { FourOfAKind } else { FullHouse }
                1 => FiveOfAKind,
                _ => panic!("Invalid Hand {self:?}")
            };

            if JOKER {
                let joker_count = *counts.get(&0).or(Some(&0usize)).unwrap();
                if joker_count > 0  {
                    return match base_classification {
                        FiveOfAKind => FiveOfAKind,
                        FourOfAKind => FiveOfAKind,
                        FullHouse => FiveOfAKind,
                        ThreeOfAKind => FourOfAKind,
                        TwoPair => if joker_count == 1 {FullHouse} else {FourOfAKind},
                        OnePair => ThreeOfAKind,
                        HighCard => OnePair,
                    }
                }
            }

            base_classification
        }
    }

    impl<const T: bool> PartialOrd for Hand<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<const T: bool> Ord for Hand<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            let card_compares = self.cards.cmp(&other.cards);

            match card_compares {
                Ordering::Equal => Ordering::Equal,
                _ => {
                    let classification_compares = self.classify().cmp(&other.classify());
                    match classification_compares {
                        Ordering::Equal => card_compares,
                        _ => classification_compares
                    }
                }
            }
        }
    }

    impl<const T: bool> Debug for Hand<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let cards = self.cards.map(|card| {
                match card {
                    1..=9 => card.to_string(),
                    10 => "T".to_string(),
                    0 | 11 => "J".to_string(),
                    12 => "Q".to_string(),
                    13 => "K".to_string(),
                    14 => "A".to_string(),
                    _ => "".to_string()
                }
            }).join("");

            write!(f, "Hand [{cards}]",
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        pub type NJHand = Hand<false>;
        pub type JHand = Hand<true>;


        #[test]
        fn test_hand_parse() {
            assert_eq!(NJHand::parse(&"J23TA"), Hand { cards: [11, 2, 3, 10, 14] });
            assert_eq!(JHand::parse(&"J23TA"), Hand { cards: [0, 2, 3, 10, 14] });
        }

        #[test]
        fn test_classify() {
            assert_eq!(NJHand::parse("12345").classify(), HighCard);
            assert_eq!(NJHand::parse("12343").classify(), OnePair);
            assert_eq!(NJHand::parse("1J2JJ").classify(), ThreeOfAKind);
            assert_eq!(NJHand::parse("AA3AA").classify(), FourOfAKind);
            assert_eq!(NJHand::parse("KKKKK").classify(), FiveOfAKind);
            assert_eq!(NJHand::parse("KKKQQ").classify(), FullHouse);
        }

        #[test]
        fn test_classify_jhand_without_joker() {
            assert_eq!(JHand::parse("12345").classify(), HighCard);
            assert_eq!(JHand::parse("12343").classify(), OnePair);
            assert_eq!(JHand::parse("42343").classify(), TwoPair);
            assert_eq!(JHand::parse("1A2AA").classify(), ThreeOfAKind);
            assert_eq!(JHand::parse("AA3AA").classify(), FourOfAKind);
            assert_eq!(JHand::parse("KKKKK").classify(), FiveOfAKind);
            assert_eq!(JHand::parse("KKKQQ").classify(), FullHouse);
        }

        #[test]
        fn test_classify_jhand_with_joker() {
            assert_eq!(JHand::parse("1234J").classify(), OnePair);

            assert_eq!(JHand::parse("J2343").classify(), ThreeOfAKind);
            assert_eq!(JHand::parse("12J4J").classify(), ThreeOfAKind);

            assert_eq!(JHand::parse("4J343").classify(), FullHouse);
            assert_eq!(JHand::parse("J23J3").classify(), FourOfAKind);

            assert_eq!(JHand::parse("JA2AA").classify(), FourOfAKind);
            assert_eq!(JHand::parse("1J2JJ").classify(), FourOfAKind);

            assert_eq!(JHand::parse("AAJAA").classify(), FiveOfAKind);
            assert_eq!(JHand::parse("JJ1JJ").classify(), FiveOfAKind);

            assert_eq!(JHand::parse("JJJJJ").classify(), FiveOfAKind);
            assert_eq!(JHand::parse("KKKJJ").classify(), FiveOfAKind);
            assert_eq!(JHand::parse("JJJTT").classify(), FiveOfAKind);
        }




        #[test]
        fn test_hand_ordering() {
            assert_eq!(NJHand::parse("33332").cmp(&NJHand::parse("2AAAA")), Ordering::Greater);
            assert_eq!(NJHand::parse("2AAAA").cmp(&NJHand::parse("33332")), Ordering::Less);
            assert_eq!(NJHand::parse("77888").cmp(&NJHand::parse("77788")), Ordering::Greater);
            assert_eq!(NJHand::parse("77788").cmp(&NJHand::parse("77888")), Ordering::Less);
            assert_eq!(NJHand::parse("A7A8A").cmp(&NJHand::parse("A7A8A")), Ordering::Equal);
        }
    }
}

mod exercise {
    use itertools::Itertools;

    use common::load_aoc_input;

    use crate::cards::{Hand};

    pub fn compute<const T: bool>(input_file: &str) -> usize {
        let input = load_aoc_input(input_file);

        input.iter()
            .map(|line| line.split_ascii_whitespace())
            .map(|mut items| (Hand::<T>::parse(items.next().unwrap()), items.next().unwrap().parse::<usize>().unwrap()))
            .sorted_by(|(x1, _), (x2, _)| x1.cmp(x2))
            .enumerate()
            .map(|(idx, (_, bid))| (idx+1) * bid)
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            assert_eq!(compute::<false>("test_data/e1.txt"), 6440);
            assert_eq!(compute::<true>("test_data/e1.txt"), 5905);
        }
    }
}

fn main() {
    println!("{}", compute::<false>("test_data/puzzle1.txt"));
    println!("{}", compute::<true>("test_data/puzzle1.txt"));
}
