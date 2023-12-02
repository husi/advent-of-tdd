mod extractor {
    use fancy_regex::CaptureMatches;
    pub trait Extractor {
        fn parse_line(&self, line: &str) -> (u32, u32);
    }

    pub fn get_bounding_matches(matches: &mut CaptureMatches) -> (String, String) {
        let first = String::from(&(matches.next().unwrap().unwrap()[1]));
        let second = if let Some(value) = matches.last() {
            String::from(&(value.unwrap()[1]))
        } else {
            first.clone()
        };
        (first, second)
    }

    pub fn compute(input: Vec<String>, extractor: &impl Extractor) -> u32 {
        input.iter()
            .map(|l| {
                let (first, last) = extractor.parse_line(l);
                10 * first + last
            }).sum()
    }


}

mod exercise1 {
    use fancy_regex::Regex;

    use common::load_aoc_input;

    use crate::extractor::{Extractor, get_bounding_matches};
    use crate::extractor::compute as common_compute;

    struct SimpleExtractor {
        re: Regex,
    }

    impl SimpleExtractor {
        fn new() -> Self {
            Self { re: Regex::new(r"(?=(\d))").unwrap() }
        }

    }

    impl Extractor for SimpleExtractor {
        fn parse_line(&self, line: &str) -> (u32, u32) {
            let mut matches = self.re.captures_iter(line);
            let (first, second) = get_bounding_matches(&mut matches);
            (first.parse().unwrap(), second.parse().unwrap())
        }
    }

    pub fn compute(input_file: &str) -> u32 {
        let  input = load_aoc_input(input_file);
        let extractor = SimpleExtractor::new();
        common_compute(input, &extractor)
    }


    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_extractor() {
            let extractor = SimpleExtractor::new();

            assert_eq!(extractor.parse_line("1234"), (1,4));
            assert_eq!(extractor.parse_line("1"), (1,1));
            assert_eq!(extractor.parse_line("sadsa1das23da4dasda"), (1,4));
            assert_eq!(extractor.parse_line("asdas1dasda"), (1,1))
        }
        #[test]
        fn test_with_example() {
            assert_eq!(compute("test_data/e1.txt"), 142)
        }
    }

}

mod exercise2 {
    use fancy_regex::Regex;

    use common::load_aoc_input;

    use crate::extractor::{Extractor, get_bounding_matches, compute as common_compute};

    const NUMBERS:[&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    struct AdvancedExtractor {
        re: Regex
    }

    impl AdvancedExtractor {
        fn new() -> Self {
            let pattern = String::from(r"(?=(") + &NUMBERS.join("|") + r"|\d))";
            Self{re: Regex::new(&pattern).unwrap()}
        }

        fn parse_number(num: &str) -> u32 {
            match NUMBERS.iter().position(|word| *word == num) {
                Some(position) => (position +1) as u32,
                None => num.parse().unwrap()
            }
        }

    }

    impl Extractor for AdvancedExtractor {
        fn parse_line(&self, line: &str) -> (u32, u32) {
            let mut matches = self.re.captures_iter(line);
            let (first, second) = get_bounding_matches(&mut matches);
            (Self::parse_number(&first), Self::parse_number(&second))
        }

    }


    pub fn compute(input_file: &str) -> u32 {
        let  input = load_aoc_input(input_file);
        let extractor = AdvancedExtractor::new();
        common_compute(input, &extractor)
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_extractor() {
            let extractor = AdvancedExtractor::new();

            assert_eq!(extractor.parse_line("1234"), (1,4));
            assert_eq!(extractor.parse_line("1"), (1,1));
            assert_eq!(extractor.parse_line("sadsa1das23da4dasda"), (1,4));
            assert_eq!(extractor.parse_line("asdas1dasda"), (1,1));
            assert_eq!(extractor.parse_line("one"), (1,1));
            assert_eq!(extractor.parse_line("onetwo"), (1,2));
            assert_eq!(extractor.parse_line("some1onemore"), (1,1));
            assert_eq!(extractor.parse_line("fancyeightwoandmore"), (8,2));
        }
        #[test]
        fn test_with_example() {
            assert_eq!(compute("test_data/e2.txt"), 281)
        }
    }

}

fn main() {
    println!("Exercise1: {}", exercise1::compute("test_data/puzzle1.txt"));
    println!("Exercise2: {}", exercise2::compute("test_data/puzzle1.txt"));
}


#[cfg(test)]
mod test_extractor {
}


