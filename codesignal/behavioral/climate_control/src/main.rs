mod climate_control;
mod climate_controller;

use climate_control::{AirConditionerFactory, ClimateDevice, DeviceFactory, HeaterFactory};
use climate_controller::{ThermostatAdapter, ThirdPartyThermostat};

fn main() {
    // TODO: Use HeaterFactory to create a Heater and display its status
    let heater = HeaterFactory.create_device();
    println!("{}", heater.get_status());

    // TODO: Use AirConditionerFactory to create an Air Conditioner and display its status
    let air_conditioner = AirConditionerFactory.create_device();
    println!("{}", air_conditioner.get_status());

    // TODO: Use ThermostatAdapter to adapt ThirdPartyThermostat and display its status
    let thermostat = ThirdPartyThermostat;
    let thermostat_adapter = ThermostatAdapter::new(thermostat);
    println!("{}", thermostat_adapter.get_status());
}
