use crate::command::Command;
use crate::device::Device;

// TODO: Implement the TurnOffCommand struct that implements Command
// - Add a constructor
// - Implement the execute method to call off on the device

pub struct TurnOffCommand;

impl TurnOffCommand {
    pub fn new() -> TurnOffCommand {
        TurnOffCommand
    }
}

impl Command for TurnOffCommand {
    fn execute(&self, device: &mut dyn Device) {
        device.off();
    }
}
