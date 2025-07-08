mod advertisement_command;
mod command;
mod weather_command;
mod weather_device;
mod weather_station;

use advertisement_command::AdvertisementCommand;
use command::Command;
use weather_command::WeatherCommand;
use weather_device::WeatherDevice;
use weather_station::WeatherStation;

fn main() {
    // TODO: Create a WeatherStation instance
    let mut station = WeatherStation::new();

    // TODO: Create two WeatherDevice instances with names "Thermometer" and "Barometer"
    let thermometer = WeatherDevice::new("Thermometer");
    let barometer = WeatherDevice::new("Barometer");

    // TODO: Add devices to the weather station
    station.add_device(thermometer);
    station.add_device(barometer);

    // TODO: Create a WeatherCommand instance with a weather update "Sunny"
    let weather_command = WeatherCommand::new("Sunny".to_string());

    // TODO: Execute the command to display and broadcast the weather update
    weather_command.execute(&mut station);

    // TODO: Create an AdvertisementCommand instance with an advertisement "Buy 1 Get 1 Free!"
    let advertisement_command = AdvertisementCommand::new("Buy 1 Get 1 Free!".to_string());

    // TODO: Execute the command to display and broadcast the advertisement
    advertisement_command.execute(&mut station);
}
