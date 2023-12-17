pub struct TemperatureSensorNoChaining {
    sampling_rate_hz: f32,
    is_enabled: bool,
}

impl TemperatureSensorNoChaining {
    pub fn create() -> Self {
        Self {
            sampling_rate_hz: 1.0,
            is_enabled: false
        }
    }

    pub fn configure_sampling_rate(&mut self, rate_hz: f32) {
        self.sampling_rate_hz = rate_hz;
    }

    pub fn activate(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn get_value(&self) -> Option<f32> {
        match self.is_enabled {
            true => Some(self.sampling_rate_hz * 123.0),
            false => None,
        }
    }
}

pub struct TemperatureSensorChaining {
    sampling_rate_hz: f32,
    is_enabled: bool,
}

impl TemperatureSensorChaining {
    pub fn create() -> Self {
        Self {
            sampling_rate_hz: 1.0,
            is_enabled: false
        }
    }

    pub fn configure_sampling_rate(&mut self, rate_hz: f32) -> &mut Self {
        self.sampling_rate_hz = rate_hz;
        self
    }

    pub fn activate(&mut self, enabled: bool) -> &mut Self {
        self.is_enabled = enabled;
        self
    }

    pub fn get_value(&self) -> Option<f32> {
        match self.is_enabled {
            true => Some(self.sampling_rate_hz * 123.0),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(false, None)]
    #[case(true, Some(1230.0))]
    fn it_should_return_value_based_on_activation_no_chaining(#[case] enabled: bool, #[case] expected: Option<f32>) {
        let mut sensor = TemperatureSensorNoChaining::create();
        sensor.configure_sampling_rate(10.0);
        sensor.activate(enabled);

        let actual = sensor.get_value();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(false, None)]
    #[case(true, Some(1230.0))]
    fn it_should_return_value_based_on_activation_chaining(#[case] enabled: bool, #[case] expected: Option<f32>) {
        let actual = TemperatureSensorChaining::create()
            .configure_sampling_rate(10.0)
            .activate(enabled)
            .get_value();

        assert_eq!(actual, expected);
    }
}
