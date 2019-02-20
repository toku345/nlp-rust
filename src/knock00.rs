pub fn reverse_string(input_string: &str) -> String {
    input_string.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_stressed_to_desserts() {
        let input_string = String::from("stressed");

        let result = reverse_string(&input_string);

        let exptedted = String::from("desserts");
        assert_eq!(result, exptedted);
    }
}
