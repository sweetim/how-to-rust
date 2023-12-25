#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_can_parse_string_using_different_patterns() {
        let input = "123";
        let expected = 123;

        assert_eq!(expected, input.parse::<u32>().unwrap());
        assert_eq!(expected, str::parse::<u32>(input).unwrap());
    }

    #[test]
    fn it_show_different_usage_of_parse_patterns_in_closure() {
        let input = vec!["123", "321"];

        let actual_1 = input.iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();

        let actual_2 = input.clone()
            .into_iter()
            .map(str::parse::<u32>)
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();

        let actual_3 = input.into_iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();

        let expected = vec![123, 321];

        assert_eq!(actual_1, expected);
        assert_eq!(actual_2, expected);
        assert_eq!(actual_3, expected);
    }
}
