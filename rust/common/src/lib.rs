use std::fs;

pub fn load_aoc_input(file_name: &str) -> Vec<String> {
    let content = fs::read_to_string(file_name).unwrap();
    content.split('\n').map(|s| {String::from(s.trim())}).collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_has_4_lines() {
        let data = load_aoc_input("test_data/test.txt");
        assert_eq!(data.len(),4);

    }
}
