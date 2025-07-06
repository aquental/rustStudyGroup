use crate::command::Command;
use crate::fan::Fan;

pub struct FanOnCommand;

impl FanOnCommand {
    pub fn new() -> Self {
        FanOnCommand
    }
}

impl Command for FanOnCommand {
    fn execute(&self, fan: &mut Fan) {
        fan.on();
    }
}
