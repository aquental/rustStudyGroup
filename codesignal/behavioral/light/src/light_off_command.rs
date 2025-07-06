// TODO: Add necessary imports from other modules

use crate::{command::Command, light::Light};

// TODO: Define the LightOffCommand struct
// - Implement the Command trait for this struct
// - Implement the execute method to turn off the light by calling the `off` method of the Light
pub struct LightOffCommand;

impl LightOffCommand {
    pub fn new() -> Self {
        LightOffCommand
    }
}

impl Command for LightOffCommand {
    fn execute(&self, light: &mut Light) {
        light.off();
    }
}
