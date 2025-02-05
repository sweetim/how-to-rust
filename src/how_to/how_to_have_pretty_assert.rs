use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CpuStates {
    cpu: i32,
    user: f32,
    system: f32,
    nice: f32,
    idle: f32,
    io_wait: f32,
    hw_irq: f32,
    soft_irq: f32,
    steal: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> (CpuStates, CpuStates) {
        let actual = CpuStates {
            cpu: 1,
            user: 2.0,
            system: 3.0,
            nice: 4.0,
            idle: 5.0,
            io_wait: 6.0,
            hw_irq: 7.0,
            soft_irq: 8.0,
            steal: 9.0,
        };

        let expected = CpuStates {
            cpu: 1,
            user: 1.0,
            system: 1.0,
            nice: 1.0,
            idle: 1.0,
            io_wait: 1.0,
            hw_irq: 1.0,
            soft_irq: 1.0,
            steal: 1.0,
        };

        (actual, expected)
    }

    #[test]
    #[ignore]
    fn with_normal_assertion() {
        let (actual, expected) = get_sample();

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn with_pretty_assertion() {
        let (actual, expected) = get_sample();

        pretty_assertions::assert_eq!(actual, expected);
    }
}
