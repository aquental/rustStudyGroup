use crate::command::Command;
use crate::fan::Fan;

pub struct FanOffCommand;

impl FanOffCommand {
    pub fn new() -> Self {
        FanOffCommand
    }
}

impl Command for FanOffCommand {
    fn execute(&self, fan: &mut Fan) {
        fan.off();
    }
}
