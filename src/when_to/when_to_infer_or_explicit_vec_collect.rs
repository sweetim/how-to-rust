#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn it_shows_different_ways_to_be_explicit_and_infer_for_vec_collect() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let is_odd = |v: &i32| v % 2 == 0;

        let explicit_type = items.iter().map(|v| v % 2 == 0).collect::<Vec<bool>>();
        let using_closure_explicit_type = items.iter().map(is_odd).collect::<Vec<bool>>();
        let using_auto_infer = items.iter().map(is_odd).collect::<Vec<_>>();
        let using_explicit_variable_type_annotate: Vec<bool> = items.iter().map(is_odd).collect();

        assert_eq!(explicit_type, using_closure_explicit_type);
        assert_eq!(explicit_type, using_auto_infer);
        assert_eq!(explicit_type, using_explicit_variable_type_annotate);
    }
}
