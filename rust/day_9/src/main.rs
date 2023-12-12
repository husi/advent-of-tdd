use crate::exercise1::calculate;

mod exercise1 {
    use itertools::Itertools;

    use common::load_aoc_input;

    fn transform(history: Vec<i32>) -> Vec<i32> {
        history.iter().tuple_windows::<(_,_)>().map(|item| item.1 - item.0).collect()
    }

    fn extrapolate_front(mut history: Vec<i32>) -> i32 {
        let mut acc = 0;
        loop {
            acc += history.last().unwrap();
            history = transform(history);

            if let Ok(0) = history.iter().all_equal_value() {
                break
            }
        }
        acc
    }

    fn extrapolate_back(mut history: Vec<i32>) -> i32 {
        let mut acc = history[0];
        let mut sign: i32 = -1;
        loop {
            history = transform(history);
            acc += sign*history.first().unwrap();
            sign *= -1;

            if let Ok(0) = history.iter().all_equal_value() {
                break
            }
        }
        acc
    }

    pub fn calculate(input_file: &str, backward: bool) -> i32 {
        let lines = load_aoc_input(input_file);
        let histories: Vec<Vec<i32>> = lines.iter()
            .map(|line| {
                line.split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<i32>>()
            })
            .collect();

        histories.into_iter().map(|history| if backward {
            extrapolate_back(history)
        }  else {
            extrapolate_front(history)
        }).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate() {
            assert_eq!(calculate("test_data/e1.txt", false), 114);
            assert_eq!(calculate("test_data/e1.txt", true), 2);
        }

        #[test]
        fn test_transform() {
            assert_eq!(transform(vec![10, 13, 16, 21, 30, 45, 68]), vec![3, 3, 5, 9, 15, 23]);
        }

        #[test]
        fn test_extrapolate_front() {
            assert_eq!(extrapolate_front(vec![10, 13, 16, 21, 30, 45]), 68);
        }

        #[test]
        fn test_extrapolate_back() {
            assert_eq!(extrapolate_back(vec![10, 13, 16, 21, 30, 45]), 5);
            assert_eq!(extrapolate_back(vec![1,3,6,10,15,21]), 0);
        }
    }
}

fn main() {
    println!("{}",calculate("test_data/puzzle1.txt", false));
    println!("{}",calculate("test_data/puzzle1.txt", true));
}
