use crate::climate_control::ClimateDevice;

// Existing third-party thermostat
pub struct ThirdPartyThermostat;

impl ThirdPartyThermostat {
    pub fn read_temperature(&self) -> String {
        "Third-party thermostat reads 68°F.".to_string()
    }
}

// TODO: Implement the ThermostatAdapter struct
// - It should hold a ThirdPartyThermostat instance
pub struct ThermostatAdapter {
    thermostat: ThirdPartyThermostat,
}

// TODO: Implement a new method for ThermostatAdapter that takes a ThirdPartyThermostat and returns Self
impl ThermostatAdapter {
    pub fn new(thermostat: ThirdPartyThermostat) -> Self {
        Self { thermostat }
    }
}
// TODO: Implement ClimateDevice for ThermostatAdapter
// - Implement get_status: return the result of read_temperature from the ThirdPartyThermostat
impl ClimateDevice for ThermostatAdapter {
    fn get_status(&self) -> String {
        self.thermostat.read_temperature()
    }
}
