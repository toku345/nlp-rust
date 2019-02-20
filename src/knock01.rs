pub fn extract_and_concat_chracters(input_string: &str) -> String {
    let mut chars = input_string.chars();
    let char1 = chars.next().unwrap();
    chars.next();
    let char3 = chars.next().unwrap();
    chars.next();
    let char5 = chars.next().unwrap();
    chars.next();
    let char7 = chars.next().unwrap();
    chars.next();
    format!("{}{}{}{}", char1, char3, char5, char7)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_and_concat_chracters() {
        let input_string = String::from("パタトクカシーー");

        let result = extract_and_concat_chracters(&input_string);

        let exptected = String::from("パトカー");
        assert_eq!(result, exptected);
    }
}
