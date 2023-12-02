fn get_membership_fee_using_if_else_style_1(is_member: bool) -> f32 {
    if is_member {
        10.0
    } else {
        100.0
    }
}

fn get_membership_fee_using_if_else_style_2(is_member: bool) -> f32 {
    if is_member { 10.0 } else { 100.0 }
}

fn get_membership_fee_using_if_else_style_3(is_member: bool) -> f32 {
    if is_member {
        return 10.0;
    }

    100.0
}

fn get_membership_fee_using_match(is_member: bool) -> f32 {
    match is_member {
        true => 10.0,
        false => 100.0
    }
}

fn get_membership_fee_using_chaining(is_member: bool) -> f32 {
    is_member
        .then(|| 10.0)
        .unwrap_or_else(|| 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, 100.0)]
    #[case(true, 10.0)]
    fn it_should_all_get_same_value(#[case] is_member: bool, #[case] expected: f32) {
        let actual = vec![
            get_membership_fee_using_if_else_style_1(is_member),
            get_membership_fee_using_if_else_style_2(is_member),
            get_membership_fee_using_if_else_style_3(is_member),
            get_membership_fee_using_match(is_member),
            get_membership_fee_using_chaining(is_member),
        ];

        assert_eq!(actual, vec![ expected; actual.len() ]);
    }
}
