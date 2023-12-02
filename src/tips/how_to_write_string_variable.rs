#[cfg(test)]
mod tests {
    #[test]
    fn it_same_string() {
        let text_1 = "{
            \"name\": \"rust\",
            \"year\": 2023
        }";

        let text_2 = r#"{
            "name": "rust",
            "year": 2023
        }"#;

        assert_eq!(text_1, text_2);
    }
}
