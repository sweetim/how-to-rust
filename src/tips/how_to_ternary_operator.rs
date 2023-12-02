const MEMBER_RATE: f32 = 10.0;
const NON_MEMBER_RATE: f32 = 5.0;

pub fn get_discount_rate_using_if_else_style_1(is_member: bool) -> f32 {
    if is_member {
        MEMBER_RATE
    } else {
        NON_MEMBER_RATE
    }
}

pub fn get_discount_rate_using_if_else_style_2(is_member: bool) -> f32 {
    if is_member { MEMBER_RATE } else { NON_MEMBER_RATE }
}

pub fn get_discount_rate_using_if_else_style_3(is_member: bool) -> f32 {
    if is_member {
        return MEMBER_RATE;
    }

    NON_MEMBER_RATE
}

pub fn get_discount_rate_using_match(is_member: bool) -> f32 {
    match is_member {
        true => MEMBER_RATE,
        false => NON_MEMBER_RATE
    }
}

pub fn get_discount_rate_using_chaining(is_member: bool) -> f32 {
    is_member
        .then(|| MEMBER_RATE)
        .unwrap_or_else(|| NON_MEMBER_RATE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, NON_MEMBER_RATE)]
    #[case(true, MEMBER_RATE)]
    fn it_should_all_get_same_value(#[case] is_member: bool, #[case] expected: f32) {
        let actual = vec![
            get_discount_rate_using_if_else_style_1(is_member),
            get_discount_rate_using_if_else_style_2(is_member),
            get_discount_rate_using_if_else_style_3(is_member),
            get_discount_rate_using_match(is_member),
            get_discount_rate_using_chaining(is_member),
        ];

        assert_eq!(actual, vec![ expected; actual.len() ]);
    }
}
