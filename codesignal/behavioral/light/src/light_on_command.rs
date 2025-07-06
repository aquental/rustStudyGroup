// TODO: Add necessary imports from other modules

use crate::command::Command;
use crate::light::Light;

// TODO: Define the LightOnCommand struct
// - Implement the Command trait for this struct
// - Implement the execute method to turn on the light by calling the `on` method of the Light
pub struct LightOnCommand;

impl LightOnCommand {
    pub fn new() -> Self {
        LightOnCommand
    }
}

impl Command for LightOnCommand {
    fn execute(&self, light: &mut Light) {
        light.on();
    }
}
