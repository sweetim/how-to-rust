use mockall::*;

#[automock]
pub trait Sensor {
    fn get_value(&self) -> f32;
}

pub fn is_overheating(input: &dyn Sensor) -> bool {
    input.get_value() > 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn without_parameterized_is_overheating() {
        [
            (123.0, true),
            (13.0, false)
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let mut mock = MockSensor::new();

            mock.expect_get_value()
                .times(1)
                .return_const(input);

            assert_eq!(is_overheating(&mock), expected, "input is {} expected to be ({})", input, expected);
        });
    }

    #[rstest]
    #[case(123.0, true)]
    #[case(13.0, false)]
    fn with_parameterized_is_overheating(#[case] input: f32, #[case] expected: bool) {
        let mut mock = MockSensor::new();

        mock.expect_get_value()
            .times(1)
            .return_const(input);

        assert_eq!(is_overheating(&mock), expected);
    }
}
