use std::collections::HashMap;

// TODO: Define the Display trait
// - Define the update method (parameter: &str weather_update) for receiving updates.
pub trait Display {
    fn update(&self, weather_update: &str);
    fn name(&self) -> &str;
}

// TODO: Define the ConcreteDisplay struct implementing Display
// - Create a constructor to initialize the name (parameter: &str name)
// - Override the update method to display:
//    - "{name} received weather update: {weather_update}"

pub struct ConcreteDisplay {
    name: String,
}
impl ConcreteDisplay {
    pub fn new(name: &str) -> Self {
        ConcreteDisplay {
            name: name.to_string(),
        }
    }
}
impl Display for ConcreteDisplay {
    fn update(&self, weather_update: &str) {
        println!("{} received weather update: {}", self.name, weather_update);
    }
    fn name(&self) -> &str {
        &self.name
    }
}

// TODO: Define the WeatherStation struct
// - Define a HashMap to hold displays
// - Implement the add_display method to add a display to the list (parameter: Box<dyn Display> display)
// - Implement the set_weather method to notify all displays about the weather update (parameter: &str weather_update)
//    - Iterate through the collection of displays and call each display's update method with the weather update
pub struct WeatherStation {
    displays: HashMap<String, Box<dyn Display>>,
}

impl WeatherStation {
    pub fn new() -> Self {
        WeatherStation {
            displays: HashMap::new(),
        }
    }
    pub fn add_display(&mut self, display: Box<dyn Display>) {
        self.displays.insert(display.name().to_string(), display);
    }
    pub fn set_weather(&self, weather_update: &str) {
        for display in self.displays.values() {
            display.update(weather_update);
        }
    }
}
