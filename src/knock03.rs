use std::collections::HashMap;

pub fn calc_word_length_map(input_string: &str) -> HashMap<&str, usize> {
    [(input_string, input_string.len())]
        .iter()
        .cloned()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_make_word_length_map_from_a_word() {
        let input_string = String::from("Now");

        let result = calc_word_length_map(&input_string);

        let expected: HashMap<&str, usize> = [("Now", 3)].iter().cloned().collect();

        assert_eq!(result, expected);
    }
}
