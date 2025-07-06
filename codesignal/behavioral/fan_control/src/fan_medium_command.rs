use crate::{command::Command, fan::Fan};

// TODO: Create FanMediumCommand struct implementing Command to control Fan's medium speed
// - Create struct `FanMediumCommand` that implements the `Command` trait.
// - In the `execute` method, invoke the `medium` method on the `Fan` object.
// - In the `undo` method, revert the fan to its previous state (e.g., turn it off).
pub struct FanMediumCommand {
    fan: Fan,
}
impl FanMediumCommand {
    pub fn new() -> Self {
        Self { fan: Fan::new() }
    }
}
impl Command for FanMediumCommand {
    fn execute(&self, fan: &mut Fan) {
        fan.medium();
    }
    fn undo(&self, fan: &mut Fan) {
        fan.off();
    }
}
