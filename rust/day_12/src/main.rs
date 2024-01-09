use crate::exercise1::calculate;

mod decipher {
    use derive_new::new;

    #[derive(PartialEq, Debug, new)]
    pub struct DecipherState {
        pattern: String,
        fixture: String,
        chunks: Vec<usize>,
    }


    impl DecipherState {
        pub fn next_states(&self) -> Vec<DecipherState> {
            let remaining = self.pattern.len() - self.fixture.len();
            if self.chunks.len() == 0 {
                // No more chuncks just fill in

                let fixture = format!("{}{}", self.fixture, ".".repeat(remaining));
                if self.validate(&fixture) {
                    return vec![DecipherState { pattern: self.pattern.clone(), fixture, chunks: vec![] }];
                } else {
                    return vec![];
                }
            }

            let next_chunk_length = *self.chunks.first().unwrap();
            let chunk_txt = "#".repeat(next_chunk_length) + if self.chunks.len() > 1 { "." } else { "" };
            let max_space_size: usize = self.pattern.len()
                - self.fixture.len() // what we already have
                - self.chunks.iter().sum::<usize>() // what sure we will have
                - (self.chunks.len() - 1); // The minimal dot's between

            let mut result = Vec::new();

            for l in 0..=max_space_size {
                let fixture = format!("{}{}{}", self.fixture, ".".repeat(l), chunk_txt);

                if self.validate(&fixture) {
                    result.push(DecipherState { pattern: self.pattern.clone(), fixture, chunks: Vec::from(&self.chunks[1..]) })
                }
            }

            result
        }

        pub fn is_terminal(&self) -> bool {
            self.fixture.len() == self.pattern.len()
        }
        fn validate(&self, fixture: &str) -> bool {
            // we just validate after current fixture
            self.pattern[self.fixture.len()..fixture.len()]
                .chars()
                .enumerate()
                .filter(|(_, char)| char != &'?')
                .all(|(offset, pattern_char)| pattern_char == fixture.as_bytes()[self.fixture.len() + offset] as char)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_next_states() {
            let pattern = "?###????????".to_string();
            assert_eq!(DecipherState::new(pattern.clone(), ".###.".to_string(), vec![]).next_states(),
                       vec![
                           DecipherState::new(pattern.clone(), ".###........".to_string(), vec![])
                       ]);
            assert_eq!(DecipherState::new(pattern.clone(), ".###.".to_string(), vec![2]).next_states(),
                       vec![
                           DecipherState::new(pattern.clone(), ".###.##".to_string(), vec![]),
                           DecipherState::new(pattern.clone(), ".###..##".to_string(), vec![]),
                           DecipherState::new(pattern.clone(), ".###...##".to_string(), vec![]),
                           DecipherState::new(pattern.clone(), ".###....##".to_string(), vec![]),
                           DecipherState::new(pattern.clone(), ".###.....##".to_string(), vec![]),
                           DecipherState::new(pattern.clone(), ".###......##".to_string(), vec![]),
                       ]);
            assert_eq!(DecipherState::new(pattern.clone(), ".###.".to_string(), vec![2, 3]).next_states(),
                       vec![
                           DecipherState::new(pattern.clone(), ".###.##.".to_string(), vec![3]),
                           DecipherState::new(pattern.clone(), ".###..##.".to_string(), vec![3]),
                       ]);
            assert_eq!(DecipherState::new(pattern.clone(), "".to_string(), vec![3, 2, 1]).next_states(),
                       vec![
                           DecipherState::new(pattern.clone(), ".###.".to_string(), vec![2, 1]),
                       ])
        }
    }
}

mod exercise1 {
    use std::collections::HashSet;
    use itertools::{Itertools, repeat_n};
    use rayon::prelude::*;
    use common::load_aoc_input;
    use crate::decipher::DecipherState;

    fn get_line_results(line: &str, unfold: usize, token: &str) -> usize {
        let mut split_line = line.split_whitespace();
        let pattern = split_line.next().unwrap().to_string();
        let pattern = repeat_n(pattern, unfold).intersperse(token.to_string()).collect();

        let chunks: Vec<_> = split_line.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let chunks= repeat_n(chunks, unfold).flatten().collect();

        let init_state = DecipherState::new(pattern, String::new(), chunks);
        let mut work_queue = Vec::new();
        work_queue.push(init_state);

        let mut result: usize = 0;


        while let Some(state) = work_queue.pop() {
            for state in state.next_states() {
                if state.is_terminal() {
                    result += 1;
                } else {
                    work_queue.push(state)
                }
            }
        }

        result

    }

    pub fn calculate(input_file: &str, unfold: bool) -> usize {
        let input = load_aoc_input(input_file);

        // let mut in_progress = HashSet::new();

        input.iter()
            .enumerate()
            .inspect(|(idx, res)| {
                // println!("Line starting {idx}");

            })
            .map(|(idx,line)| {
                let single = get_line_results(line, 1, "");
                let twofold = get_line_results(line, 2, "#");
                let threefold = get_line_results(line, 3, "#");

                let with_dot = single*single;
                let with_dot_3 = with_dot*single;

                if (twofold != threefold) && twofold != with_dot && threefold != with_dot_3 {
                    println!("{idx} {single} | {with_dot} {twofold} | {with_dot_3} {threefold}");
                }

                (idx,2)
            })
            .inspect(|(idx, res)| {
                // println!("Line matched {idx} -> {res}");
            })
            .map(|(_, res)| res )
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate(){
            assert_eq!(calculate("test_data/e1.txt",false), 21);
            assert_eq!(calculate("test_data/e1.txt",true), 525152);
        }
    }
}

fn main() {
    // println!("{}", calculate("test_data/puzzle1.txt", false));
    println!("{}", calculate("test_data/puzzle1.txt", true));
}
