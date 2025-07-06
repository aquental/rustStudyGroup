use crate::{command::Command, fan::Fan};

// TODO: Implement a struct named FanOffCommand that implements the Command trait.
// - Create a struct named FanOffCommand.
// - Implement a constructor `new` for FanOffCommand.
// - Implement the Command trait for FanOffCommand.
//     - The `execute` method should call `fan.off()`.
//     - The `undo` method should call `fan.on()` and print "Undo: Fan turned on.".

pub struct FanOffCommand {
    fan: Fan,
}
impl FanOffCommand {
    pub fn new(fan: Fan) -> Self {
        FanOffCommand { fan }
    }
}
impl Command for FanOffCommand {
    fn execute(&mut self, fan: &mut Fan) {
        fan.off();
    }
    fn undo(&mut self, fan: &mut Fan) {
        fan.on();
        println!("Undo: Fan turned on.");
    }
}
