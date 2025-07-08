// TODO: import WeatherStation
use crate::weather_station::WeatherStation;

// TODO: Define a trait Command with a method `execute(&self, station: &mut WeatherStation)`
pub trait Command {
    fn execute(&self, station: &mut WeatherStation);
}
