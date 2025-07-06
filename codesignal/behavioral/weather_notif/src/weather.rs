use std::collections::HashMap;

// TODO: Define the Display trait
// - Define the update method (parameter: &str weather_update) for receiving updates.

// TODO: Define the ConcreteDisplay struct implementing Display
// - Create a constructor to initialize the name (parameter: &str name)
// - Override the update method to display:
//    - "{name} received weather update: {weather_update}"

// TODO: Define the WeatherStation struct
// - Define a HashMap to hold displays
// - Implement the add_display method to add a display to the list (parameter: Box<dyn Display> display)
// - Implement the set_weather method to notify all displays about the weather update (parameter: &str weather_update)
//    - Iterate through the collection of displays and call each display's update method with the weather update
