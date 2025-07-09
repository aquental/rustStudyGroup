use crate::command::Command;
use crate::device::Device;

// TODO: Implement the TurnOnCommand struct that implements Command
// - Add a constructor
// - Implement the execute method to call on the device
pub struct TurnOnCommand;

impl TurnOnCommand {
    pub fn new() -> TurnOnCommand {
        TurnOnCommand
    }
}

impl Command for TurnOnCommand {
    fn execute(&self, device: &mut dyn Device) {
        device.on();
    }
}
