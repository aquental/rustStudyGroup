// Adaptee
// TODO: Implement the CelsiusThermometer struct
pub struct CelsiusThermometer {
    temperature: f64,
}
// It should have a method get_temperature_in_celsius that returns a temperature value
// You are allowed to hardcode the temperature value for simplicity
impl CelsiusThermometer {
    pub fn new() -> Self {
        CelsiusThermometer { temperature: 25.0 } // Hardcoded to 25°C for simplicity
    }

    pub fn get_temperature_in_celsius(&self) -> f64 {
        self.temperature
    }
}

// Target interface
// TODO: Define the FahrenheitThermometer trait
// It should have a method get_temperature_in_fahrenheit
pub trait FahrenheitThermometer {
    fn get_temperature_in_fahrenheit(&self) -> f64;
}

// Adapter
// TODO: Implement the ThermometerAdapter struct
// It should have a constructor that takes a CelsiusThermometer as parameter
// Implement the get_temperature_in_fahrenheit method to convert the Celsius temperature to Fahrenheit
pub struct ThermometerAdapter {
    celsius_thermometer: CelsiusThermometer,
}

impl ThermometerAdapter {
    pub fn new(celsius_thermometer: CelsiusThermometer) -> Self {
        ThermometerAdapter {
            celsius_thermometer,
        }
    }
}

impl FahrenheitThermometer for ThermometerAdapter {
    fn get_temperature_in_fahrenheit(&self) -> f64 {
        // Convert Celsius to Fahrenheit: F = (C * 9/5) + 32
        (self.celsius_thermometer.get_temperature_in_celsius() * 9.0 / 5.0) + 32.0
    }
}
