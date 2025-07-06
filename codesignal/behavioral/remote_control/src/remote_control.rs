use crate::command::Command;
use crate::fan::Fan;

pub struct RemoteControl {
    command: Option<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl { command: None }
    }

    // TODO: Implement the set_command method to get the command object and set it to the command variable
    pub fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = Some(command);
    }

    // TODO: Implement the press_button method to execute the command, if any command is set
    pub fn press_button(&self, fan: &mut Fan) {
        if let Some(command) = &self.command {
            command.execute(fan);
        }
    }
}
