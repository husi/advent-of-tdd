use crate::exercise1::Race;

mod exercise1 {
    use std::cmp::{max, min};

    pub struct Race {
        time: usize,
        distance: usize,
    }

    impl Race {
        pub fn new(time: usize, distance: usize) -> Race {
            Race { time, distance }
        }
    }


    fn solve_race(race: &Race) -> (usize, usize) {
        let t = race.time as u128;
        let d = race.distance as u128;

        let tmp1 = (t as f64) / 2.0;
        let tmp2 = ((t.pow(2) - 4 * d) as f64).sqrt() / 2.0;
        let left = tmp1 - tmp2;
        let right = tmp1 + tmp2;

        (
            max(left.ceil() as usize, (left + 1.0).floor() as usize),
            min(right.floor() as usize, (right - 1.0).ceil() as usize)
        )
    }

    pub fn compute(races: Vec<Race>) -> usize {
        races.iter()
            .map(|race| {
                let (min, max) = solve_race(race);
                max - min + 1
            }).product()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compute() {
            assert_eq!(compute(vec![
                Race::new(7, 9),
                Race::new(15, 40),
                Race::new(30, 200),
            ]), 288);
            assert_eq!(compute(vec![
                Race::new(71530, 940200),
            ]), 71503);
        }
    }
}

fn main() {
    println!("{}", exercise1::compute(
        vec![
            Race::new(47, 400),
            Race::new(98, 1213),
            Race::new(66, 1011),
            Race::new(98, 1540),
        ]
    ));

    println!("{}", exercise1::compute(
        vec![
            Race::new(47986698, 400121310111540),
        ]
    ));
}
