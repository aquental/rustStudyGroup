use crate::{command::Command, fan::Fan};

// TODO: Create FanLowCommand struct implementing Command to control Fan's low speed
// - Create struct `FanLowCommand` that implements the `Command` trait.
// - In the `execute` method, invoke the `low` method on the `Fan` object.
// - In the `undo` method, revert the fan to its previous state (e.g., turn it off).
pub struct FanLowCommand {
    fan: Fan,
    prev_speed: Option<Fan>,
}
impl FanLowCommand {
    pub fn new() -> Self {
        Self {
            fan: Fan::new(),
            prev_speed: None,
        }
    }
}
impl Command for FanLowCommand {
    fn execute(&self, fan: &mut Fan) {
        fan.low();
    }
    fn undo(&self, fan: &mut Fan) {
        fan.off();
    }
}
