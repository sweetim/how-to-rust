#[cfg(test)]
mod tests {
    #[test]
    fn it_same_string() {
        let normal_with_escape_text = "{
            \"name\": \"rust\",
            \"year\": 2023
        }";

        let raw_string_literal_text = r#"{
            "name": "rust",
            "year": 2023
        }"#;

        let macro_include_file_text = include_str!("../../resources/string_variable.txt");

        assert_eq!(normal_with_escape_text, raw_string_literal_text);
        assert_eq!(normal_with_escape_text, macro_include_file_text);
    }
}
