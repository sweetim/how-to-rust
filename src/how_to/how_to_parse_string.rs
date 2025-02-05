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
        let expected = vec![123, 321];
        let input = vec!["123", "321"];

        let actual_using_iter = input
            .iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();

        let actual_using_filter_map_ok_closure = input
            .clone()
            .into_iter()
            .map(str::parse::<u32>)
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();

        let actual_using_filter_map_method = input
            .clone()
            .into_iter()
            .map(str::parse::<u32>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let actual_using_into_iter = input
            .into_iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();

        assert_eq!(actual_using_iter, expected);
        assert_eq!(actual_using_filter_map_ok_closure, expected);
        assert_eq!(actual_using_filter_map_method, expected);
        assert_eq!(actual_using_into_iter, expected);
    }
}
