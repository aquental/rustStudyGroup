// TODO: import WeatherDevice
use crate::weather_device::WeatherDevice;

// TODO: Define the WeatherStation struct
// - It should store a list of WeatherDevice in a vector
// - Define a constructor that initializes an empty device list
// - Define the methods:
//    - `show_weather` to display the current weather
//    - `show_advertisement` to display an advertisement
//    - `add_device` to add a weather device to the station
//    - `notify_devices` to broadcast a weather update to all devices
//    - `broadcast_advertisement` to broadcast an advertisement to all devices
pub struct WeatherStation {
    list: Vec<WeatherDevice>,
}
impl WeatherStation {
    pub fn new() -> WeatherStation {
        WeatherStation { list: Vec::new() }
    }
    pub fn show_weather(&self) {
        for device in &self.list {
            device.show_weather();
        }
    }
    pub fn show_advertisement(&self) {
        for device in &self.list {
            device.show_advertisement();
        }
    }
    pub fn add_device(&mut self, device: WeatherDevice) {
        self.list.push(device);
    }
    pub fn notify_devices(&self, weather: &str) {
        for device in &self.list {
            device.update_weather(weather.to_string());
        }
    }
    pub fn broadcast_advertisement(&self, advertisement: &str) {
        for device in &self.list {
            device.receive_advertisement(advertisement.to_string());
        }
    }
}
