use crate::{command::Command, fan::Fan};

// TODO: Create FanHighCommand struct implementing Command to control Fan's high speed
// - Create struct `FanHighCommand` that implements the `Command` trait.
// - In the `execute` method, invoke the `high` method on the `Fan` object.
// - In the `undo` method, revert the fan to its previous state (e.g., turn it off).
pub struct FanHighCommand {
    fan: Fan,
}
impl FanHighCommand {
    pub fn new() -> FanHighCommand {
        FanHighCommand { fan: Fan::new() }
    }
}
impl Command for FanHighCommand {
    fn execute(&self, fan: &mut Fan) {
        fan.high();
    }
    fn undo(&self, fan: &mut Fan) {
        fan.off();
    }
}
