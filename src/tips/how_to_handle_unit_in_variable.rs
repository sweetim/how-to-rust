#[derive(Debug)]
pub struct VehicleStatus {
    pub speed: f32,
    pub battery_soc: f32,
    pub battery_voltage: f32,
    pub uptime: u32,
}

/// Represents the status of a vehicle, including its speed, battery state of charge (SOC), battery voltage, and vehicle uptime.
#[derive(Debug)]
pub struct VehicleStatusWithDocs {
    /// The current speed of the vehicle in kilometers per hour (km/h).
    pub speed: f32,

    /// The current state of charge (SOC) of the vehicle's battery, expressed as a percentage.
    /// A value of 100% indicates a fully charged battery, while a value of 0% indicates a fully discharged battery.
    pub battery_soc: f32,

    /// The current voltage of the vehicle's battery in mili volts (mV).
    pub battery_voltage: f32,

    /// The up time of the vehicle since turn on in miliseconds (ms).
    pub uptime: u32,
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct VehicleStatusWithUnitSuffix {
    pub speed_kph: f32,
    pub battery_soc_percentage: f32,
    pub battery_voltage_mV: f32,
    pub uptime_ms: u32
}

#[cfg(test)]
mod tests {
    use super::{VehicleStatus, VehicleStatusWithDocs, VehicleStatusWithUnitSuffix};

    #[test]
    #[allow(unused_variables)]
    fn it_init_all_vehicle_status_struct() {
        let vehicle_status = VehicleStatus {
            speed: 10.2, // kph
            battery_soc: 50.2, // percentage
            battery_voltage: 120000.1, // mili voltage
            uptime: 21300, // miliseconds
        };

        let vehicle_status_with_docs = VehicleStatusWithDocs {
            speed: 10.2,
            battery_soc: 50.2,
            battery_voltage: 120000.1,
            uptime: 21300,
        };

        let vehicle_status_with_unit_suffix = VehicleStatusWithUnitSuffix {
            speed_kph: 10.2,
            battery_soc_percentage: 50.2,
            battery_voltage_mV: 120000.1,
            uptime_ms: 21300,
        };
    }
}
