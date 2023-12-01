mod extractor {
    pub trait Extractor {
        fn find_first_num<I>(&self, chars: I) -> Option<u32>
            where I: Iterator<Item=char>;
    }

    pub fn extract_first_num(line: &str, extractor: &impl Extractor) -> Result<u32, String> {
        extractor.find_first_num(line.chars()).ok_or("No number".to_string())
    }

    pub fn extract_last_num(line: &str, extractor: &impl Extractor) -> Result<u32, String> {
        extractor.find_first_num(line.chars().rev()).ok_or("No number".to_string())
    }

    pub struct SimpleExtractor {}

    impl Extractor for SimpleExtractor {
        fn find_first_num<I>(&self, mut chars: I) -> Option<u32> where I: Iterator<Item=char> {
            chars.find(|c| { c >= &'0' && c <= &'9' }).map(|c| { c.to_digit(10).unwrap() })
        }
    }

}

mod exercise1 {
    use common::load_aoc_input;
    use crate::extractor::{extract_first_num, extract_last_num, SimpleExtractor};

    pub fn compute(input_file: &str) -> u32 {
        let input = load_aoc_input(input_file);
        let extractor = SimpleExtractor{};
        input.iter().map(|l| { 10 * extract_first_num(l,&extractor).unwrap() + extract_last_num(l,&extractor).unwrap() }).sum()
    }
}

fn main() {
    println!("Excercise1: {}", exercise1::compute("test_data/puzzle1.txt"))
}


#[cfg(test)]
mod test_extractor {
    use super::extractor;

    #[test]
    fn test_extract_first_num() {
        let extractor =extractor::SimpleExtractor{};
        assert_eq!(extractor::extract_first_num("1234", &extractor).unwrap(), 1);
        assert_eq!(extractor::extract_first_num("aaa1assaa", &extractor).unwrap(), 1);
    }

    #[test]
    fn test_extract_last_num() {
        let extractor =extractor::SimpleExtractor{};
        assert_eq!(extractor::extract_last_num("1234", &extractor).unwrap(), 4);
        assert_eq!(extractor::extract_last_num("aaa1assaa", &extractor).unwrap(), 1);
    }
}

#[cfg(test)]
mod test_exercise1 {
    use super::exercise1;

    #[test]
    fn test_with_example() {
        assert_eq!(exercise1::compute("test_data/e1.txt"), 142)
    }
}