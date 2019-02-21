pub fn zip_strings(input_string1: &str, input_string2: &str) -> String {
    input_string1
        .chars()
        .zip(input_string2.chars())
        .fold(String::new(), |acc, tuple| {
            format!("{}{}{}", acc, tuple.0, tuple.1)
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zip_strings() {
        let input_string1 = String::from("パトカー");
        let input_string2 = String::from("タクシー");

        let result = zip_strings(&input_string1, &input_string2);

        let exptected = String::from("パタトクカシーー");
        assert_eq!(result, exptected);
    }
}
