use crate::{command::Command, fan::Fan};

// TODO: Create FanOffCommand struct implementing Command to turn off the Fan
// - Create struct `FanOffCommand` that implements the `Command` trait.
// - In the `execute` method, invoke the `off` method on the `Fan` object.
// - In the `undo` method, handle the undo operation appropriately (e.g., print a message).
pub struct FanOffCommand {
    fan: Fan,
}
impl FanOffCommand {
    pub fn new() -> FanOffCommand {
        FanOffCommand { fan: Fan::new() }
    }
}
impl Command for FanOffCommand {
    fn execute(&self, fan: &mut Fan) {
        fan.off();
    }

    fn undo(&self, fan: &mut Fan) {
        fan.low();
    }
}
