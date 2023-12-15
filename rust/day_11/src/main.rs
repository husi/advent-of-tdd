use crate::exercise1::calculate;

mod star_map {
    use std::collections::HashSet;

    pub fn get_galaxies(star_map: &Vec<String>) -> Vec<(usize, usize)> {
        star_map.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
            }).collect()
    }

    pub fn get_expansion_offsets(galaxies: &Vec<(usize, usize)>, expansion_speed: usize) -> (Vec<usize>, Vec<usize>) {
        let mut x_vectors: HashSet<usize> = HashSet::new();
        let mut y_vectors: HashSet<usize> = HashSet::new();

        for (x, y) in galaxies {
            x_vectors.insert(*x);
            y_vectors.insert(*y);
        }

        (get_expansion_offset_for_one_direction(x_vectors, expansion_speed),
         get_expansion_offset_for_one_direction(y_vectors, expansion_speed))
    }

    fn get_expansion_offset_for_one_direction(galaxy_vectors: HashSet<usize>, expansion_speed: usize) -> Vec<usize> {
        (0..=*galaxy_vectors.iter().max().unwrap())
            .fold(Vec::new(), |mut result, num| {
                if galaxy_vectors.contains(&num) {
                    result.push(*result.last().unwrap_or(&0usize))
                } else {
                    result.push(result.last().unwrap_or(&0usize) + expansion_speed)
                };
                result
            })
    }

    pub fn expand(galaxies: Vec<(usize, usize)>, expansion_offsets:(Vec<usize>, Vec<usize>)) -> Vec<(usize,usize)> {
        let (x_offsets, y_offsets) = expansion_offsets;
        galaxies.iter().map(|(x,y)| (x + x_offsets[*x], y+y_offsets[*y])).collect()
    }

    #[cfg(test)]
    mod tests {
        use common::load_aoc_input;
        use super::*;

        #[test]
        fn test_get_galaxies() {
            let input = load_aoc_input("test_data/e1.txt");

            assert_eq!(get_galaxies(&input), vec![(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)]);
        }

        #[test]
        fn test_get_expansion_offsets() {
            let input = load_aoc_input("test_data/e1.txt");
            let galaxies = get_galaxies(&input);
            assert_eq!(get_expansion_offsets(&galaxies, 1), (vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3], vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2]));
            assert_eq!(get_expansion_offsets(&galaxies, 10), (vec![0, 0, 10, 10, 10, 20, 20, 20, 30, 30], vec![0, 0, 0, 10, 10, 10, 10, 20, 20, 20]));
        }
    }
}

mod exercise1 {
    use common::load_aoc_input;
    use crate::star_map::{expand, get_expansion_offsets, get_galaxies};
    use itertools::Itertools;

    pub fn calculate(input_file: &str, expansion_speed: usize) -> usize {
        let input = load_aoc_input(input_file);
        let galaxies = get_galaxies(&input);
        let expansion_offsets = get_expansion_offsets(&galaxies, expansion_speed);
        let expanded = expand(galaxies, expansion_offsets);

        expanded.iter()
            .combinations(2)
            .map(|nodes| {
                let (x1,y1) = nodes[0];
                let (x2,y2) = nodes[1];

                x1.abs_diff(*x2) + y1.abs_diff(*y2)
            })
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate() {
            assert_eq!(calculate("test_data/e1.txt", 1), 374);
            assert_eq!(calculate("test_data/e1.txt", 9), 1030);
        }
    }
}

fn main() {
    println!("{}", calculate("test_data/puzzle1.txt",1));
    println!("{}", calculate("test_data/puzzle1.txt",999999));
}
