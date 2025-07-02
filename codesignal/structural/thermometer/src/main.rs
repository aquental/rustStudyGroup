use crate::thermometer::{CelsiusThermometer, FahrenheitThermometer, ThermometerAdapter};

mod thermometer;


fn main() {
    // Creating an instance of CelsiusThermometer
    let celsius = CelsiusThermometer::new();

    // TODO: Create an instance of ThermometerAdapter using the BritishPlug instance
    let adapter = ThermometerAdapter::new(celsius);

    // TODO: Call the get_temperature_in_fahrenheit method on the adapter
    adapter.get_temperature_in_fahrenheit();
}
