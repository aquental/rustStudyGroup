use crate::{command::Command, fan::Fan};

// TODO: Implement a struct named FanOnCommand that implements the Command trait.
// - Create a struct named FanOnCommand.
// - Implement a constructor `new` for FanOnCommand.
// - Implement the Command trait for FanOnCommand.
//     - The `execute` method should call `fan.on()`.
//     - The `undo` method should call `fan.off()` and print "Undo: Fan turned off.".
pub struct FanOnCommand {
    fan: Fan,
}
impl FanOnCommand {
    pub fn new(fan: Fan) -> Self {
        FanOnCommand { fan }
    }
}
impl Command for FanOnCommand {
    fn execute(&mut self, fan: &mut Fan) {
        fan.on();
    }
    fn undo(&mut self, fan: &mut Fan) {
        fan.off();
        println!("Undo: Fan turned off.");
    }
}
