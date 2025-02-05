#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn it_give_same_value() {
        let items = vec![1, 2, 3, 4, 5];
        let items_initial_len = items.len();

        let mut items_iter = items.iter();
        let first_item_iter = items_iter.next().unwrap();
        let second_item_iter = items_iter.next().unwrap();
        items_iter.next();
        let fourth_item_iter = items_iter.next().unwrap();

        assert_eq!(*first_item_iter, 1);
        assert_eq!(*second_item_iter, 2);
        assert_eq!(*fourth_item_iter, 4);

        let first_item_index_access = items[0];
        let fourth_item_index_access = items[3];
        let second_item_index_access = items[1];

        assert_eq!(first_item_index_access, 1);
        assert_eq!(second_item_index_access, 2);
        assert_eq!(fourth_item_index_access, 4);

        let first_item_nth = items.iter().nth(0).unwrap();
        let fourth_item_nth = items.iter().nth(3).unwrap();
        let second_item_nth = items.iter().nth(1).unwrap();

        assert_eq!(*first_item_nth, 1);
        assert_eq!(*second_item_nth, 2);
        assert_eq!(*fourth_item_nth, 4);

        assert_eq!(items.len(), items_initial_len);
    }

    #[test]
    fn it_show_the_benefit_of_using_iter_when_empty_vec() {
        let items: Vec<u32> = vec![];
        let items_initial_len = items.len();

        let first_item_iter = items.iter().next();
        let first_item_nth = items.iter().nth(0);

        assert!(first_item_iter.is_none());
        assert!(first_item_nth.is_none());
        assert_eq!(items.len(), items_initial_len);
    }

    #[test]
    #[should_panic]
    fn it_show_the_disadvantage_of_unbounded_index_access() {
        let items: Vec<u32> = vec![];
        let items_initial_len = items.len();

        let _first_item_index_access = items[0];

        assert_eq!(items.len(), items_initial_len);
    }
}
