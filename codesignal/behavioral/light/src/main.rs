mod command;
mod light;
mod light_off_command;
mod light_on_command;

use command::Command;
use light::Light;
use std::collections::HashMap;
// TODO: Add other necessary imports from other modules.

fn main() {
    let mut light = Light::new();

    let mut commands: HashMap<&str, Box<dyn Command>> = HashMap::new();

    // TODO: Populate the commands map
    // - Create an instance of LightOnCommand and add it to the map with the key "lightOn"
    // - Create an instance of LightOffCommand and add it to the map with the key "lightOff"

    // TODO: Simulate user input
    // - Set the command variable to "lightOn"

    // TODO: Execute the command from the commands map
    // - Use the command variable to fetch and execute the corresponding command
}
