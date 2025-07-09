mod climate_control;
mod security_control;

use climate_control::{AirQualityStrategy, ClimateControl, HumidityStrategy};
use security_control::{FireSensor, GasSensor, SecurityControl};

fn main() {
    // Observer Pattern usage
    let mut security_control = SecurityControl::new();
    // TODO: Add FireSensor and GasSensor as listeners to security_control
    security_control.add_listener(Box::new(FireSensor {}));
    security_control.add_listener(Box::new(GasSensor {}));

    println!("Triggering alarm:");
    // TODO: Trigger the alarm on security_control
    security_control.trigger_alarm();

    // Strategy Pattern usage
    // TODO: Create instances of HumidityStrategy and AirQualityStrategy
    // TODO: Initialize ClimateControl with HumidityStrategy
    let humidity = HumidityStrategy {};
    let aircontrol = AirQualityStrategy {};
    let mut climate_control = ClimateControl::new(Box::new(humidity));

    println!("Adjusting climate control:");
    // TODO: Execute the current strategy on climate_control
    climate_control.execute();

    println!("Changing strategy to air quality:");
    // TODO: Change the strategy to AirQualityStrategy and execute
    climate_control.set_strategy(Box::new(aircontrol));
    climate_control.execute();
}
