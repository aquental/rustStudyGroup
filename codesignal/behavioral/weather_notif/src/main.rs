mod weather;

use weather::{WeatherStation, ConcreteDisplay};

fn main() {
    let mut weather_station = WeatherStation::new();
    let display1 = Box::new(ConcreteDisplay::new("Balcony Display"));
    let display2 = Box::new(ConcreteDisplay::new("Living Room Display"));

    // Adding displays to the station
    weather_station.add_display(display1);
    weather_station.add_display(display2);

    // Setting weather updates
    weather_station.set_weather("Sunny");
    weather_station.set_weather("Rainy");
}
