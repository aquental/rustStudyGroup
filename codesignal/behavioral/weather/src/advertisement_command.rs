// TODO: import Command and WeatherStation
use crate::command::Command;
use crate::weather_station::WeatherStation;
// TODO: Define the AdvertisementCommand struct implementing the Command trait:
// - Store a String for the ad
// - Define a constructor taking this String
// - Implement the `execute(&self, station: &mut WeatherStation)` method
//   to show and broadcast the advertisement

pub struct AdvertisementCommand {
    ad: String,
}

impl AdvertisementCommand {
    pub fn new(ad: String) -> Self {
        Self { ad }
    }
}
impl Command for AdvertisementCommand {
    fn execute(&self, station: &mut WeatherStation) {
        println!("{}", self.ad);
        station.broadcast_advertisement(&self.ad);
    }
}
