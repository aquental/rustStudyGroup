// TODO: Define the WeatherDevice struct implementing the Observer pattern
// - Define a constructor that takes a name as a &str
// - Define a method `update_weather` which takes a string parameter to display
//   a received weather update, including the device's name
// - Define a method `receive_advertisement` which takes a string parameter to display
//   a received advertisement, including the device's name
pub struct WeatherDevice {
    name: String,
}

impl WeatherDevice {
    pub fn new(name: &str) -> WeatherDevice {
        WeatherDevice {
            name: name.to_string(),
        }
    }
    pub fn update_weather(&self, weather: String) {
        println!("{}: {}", self.name, weather);
    }
    pub fn receive_advertisement(&self, advertisement: String) {
        println!("{}: {}", self.name, advertisement);
    }
    pub fn show_weather(&self) {
        println!("{}: ", self.name);
    }
    pub fn show_advertisement(&self) {
        println!("{}: ", self.name);
    }
}
