// TODO: import Command and WeatherStation

use crate::command::Command;
use crate::weather_station::WeatherStation;

// TODO: Define the WeatherCommand struct implementing the Command trait:
// - Store a String for the weather
// - Define a constructor taking this String
// - Implement the `execute(&self, station: &mut WeatherStation)` method to show and broadcast the weather update
pub struct WeatherCommand {
    weather: String,
}
impl WeatherCommand {
    pub fn new(weather: String) -> Self {
        Self { weather }
    }
}
impl Command for WeatherCommand {
    fn execute(&self, station: &mut WeatherStation) {
        println!("{}", self.weather);
        station.notify_devices(&self.weather);
    }
}
